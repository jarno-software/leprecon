const protectedEndpoints = ["/account", "/user", "/email"];

function login() {
  client.loginWithRedirect({
    authorizationParams: {
      redirect_uri: window.location.origin,
    },
  });
}

function logout() {
  client.logout({
    logoutParams: {
      returnTo: window.location.origin,
    },
  });
}

function loginState(loggedIn, nickname) {
  if (!loggedIn || !nickname) {
    return;
  }

  document.getElementById("logout").classList = "visible";
  document.getElementById("login").classList = "hidden invisible";
  document.getElementById("balance").classList = "visible";
  document.getElementById("username").innerText = nickname;
  document.getElementById("username").classList = "cursor-pointer";
}

function handleSnackbar(id) {
  setTimeout(() => {
    document.getElementById(id).outerHTML = "";
  }, 5000);
}

function isProtectedEndpoint(endpoint) {
  let isProtected = false;
  protectedEndpoints.forEach((e) => {
    if (endpoint.includes(e)) {
      isProtected = true;
    }
  });

  return isProtected;
}
