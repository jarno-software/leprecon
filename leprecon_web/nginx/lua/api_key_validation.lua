local headers= ngx.req.get_headers()
local api_key = "";

-- Get api key
for k, v in pairs(headers) do
  if k == "authorization" then
    api_key = v:gsub("Bearer ", "");
  end
end

-- Check if key is valid
if api_key ~= ngx.var.api_key then
  ngx.exit(ngx.HTTP_UNAUTHORIZED)
end