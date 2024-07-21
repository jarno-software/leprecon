-- Get token
local cjson = require("cjson");
local headers= ngx.req.get_headers()
local method = ngx.var.request_method
local params = nil
local token = "";

-- Query params on GET and post args on rest of methods
if method == "GET" then
  params = ngx.req.get_uri_args()
else
  params = ngx.req.get_post_args()
end

for k, v in pairs(headers) do
  if k == "authorization" then
    token = v:gsub("Bearer ", "");
  end
end

-- Get jwks
local shared = ngx.shared.jwks;
local res = shared:get("jwks"); -- Shared memory cache

if res == nil then
  res = ngx.location.capture("/_jwk_key").body -- Makes internal request to auth0 jwks endpoint
  shared:set("jwks", res)
  shared:expire("jwks", 0) -- Never expire jwks (only resets when the server restarts)
end

local data = cjson.decode(res);
local keys = data["keys"];
local jwks = keys[1];

-- Load jwt
local resty_jwt = require("resty.jwt")
-- local validators = require("resty.jwt-validators")
local jwt = resty_jwt:load_jwt(token)

local function valid_token(jwt, jwks, params)
  -- Check if valid
  if jwt.valid ~= true then
    ngx.log(ngx.WARN, "Invalid token!")
    return false
  end

  -- Check header values
  if jwt.header.kid ~= jwks.kid or jwt.header.alg ~= jwks.alg or jwt.header.typ ~= "JWT" then
    ngx.log(ngx.WARN, "Invalid header value!")
    return false
  end

  -- Check expiry date
  if jwt.payload.exp <= os.time() then
    ngx.log(ngx.WARN, "Token expired!")
    return false
  end

  -- Check permissions
  if next(jwt.payload.permissions) == nil then
    ngx.log(ngx.WARN, "Empty permissions")
    return false
  end

  -- Check sub
  if params.sub ~= jwt.payload.sub then
    ngx.log(ngx.WARN, "Sub not the same")
    return false
  end

  -- Create certificate
  local cert = "-----BEGIN CERTIFICATE-----\n" .. jwks["x5c"][1] .. "\n-----END CERTIFICATE-----\n"
  local openssl = require('openssl')
  local x509, err = openssl.x509.read(cert)
  local pk, err = x509:pubkey()  
  local pem, err = pk:export() -- Cannot use openssl because of crash when using verify method

  -- Get public key
  local resty_rsa = require("resty.rsa")
  local pub, err = resty_rsa:new({ public_key = pem, algorithm = "SHA256" })

  -- Split token into parts
  local parts = {}
  for part in token:gmatch("[^.]+") do
    table.insert(parts, part)
  end

  -- Verify token
  local b64 = require("ngx.base64")
  local verify, err = pub:verify(parts[1] .. "." .. parts[2], b64.decode_base64url(parts[3]))
  if not verify then
      ngx.log(ngx.WARN, "Unable to verify token!: " .. err)
      return false
  end

  return true
end

-- Verify jwt
if not valid_token(jwt, jwks, params) then
  ngx.exit(ngx.HTTP_UNAUTHORIZED)
end