const client = new auth0.Auth0Client({
  domain: "${AUTH_HOST}",
  clientId: "${CLIENT_ID}",
  useRefreshTokens: true,
  cacheLocation: "${CACHE_LOCATION}",
  authorizationParams: {
    audience: "${AUDIENCE}",
    redirect_uri: window.location.origin,
  },
});
