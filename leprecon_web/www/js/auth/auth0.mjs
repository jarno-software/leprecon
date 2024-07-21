import {
  checkAccessExpired,
  checkAlreadyVerified,
  checkLoginCode,
  checkSuccessfulVerification,
} from "./query_handler.mjs";

let userClaims = null;

window.onload = async () => {
  localStorage.removeItem("htmx-history-cache");

  if (
    window.location.pathname != "/" &&
    !window.location.pathname.includes("/?")
  ) {
    history.pushState({}, "", "/");
  }
  const query = new URL(document.location).searchParams;

  await getToken();
  await updateUI();

  if (query.size !== 0) {
    checkLoginCode(query);
    checkAlreadyVerified(query);
    checkSuccessfulVerification(query);
    checkAccessExpired(query);
  }
};

export async function getToken() {
  try {
    return await client.getTokenSilently();
  } catch (e) {
    if (e.toString() == "Error: Unknown or invalid refresh token.") {
      logout();
      login();
    }
  }
}

export async function updateUI() {
  const isAuthenticated = await client.isAuthenticated();
  const claims = await client.getUser();

  if (!isAuthenticated) {
    loginState(false, null);
  } else {
    setUserClaims(claims);
    verificationState(claims.email_verified);
    loginState(true, claims.nickname);
  }
}

function setUserClaims(claims) {
  userClaims = claims;
}

export function getUserClaims() {
  if (userClaims) {
    return userClaims;
  }

  return null;
}

async function verificationState(email_verified) {
  if (email_verified) {
    document
      .getElementById("email-verification-snackbar")
      .classList.add("invisible", "hidden");
    return;
  }

  document
    .getElementById("email-verification-snackbar")
    .classList.remove("invisible", "hidden");
}
