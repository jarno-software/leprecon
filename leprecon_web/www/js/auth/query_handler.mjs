import { updateUI } from "./auth0.mjs";

export async function checkLoginCode(query) {
  if (query.get("code") && query.get("state")) {
    await client.handleRedirectCallback();
    history.replaceState({}, "", "/");
    await updateUI();
  }
}

export function checkAlreadyVerified(query) {
  if (
    query.get("message") === "This URL can be used only once" &&
    query.get("success") === "false"
  ) {
    history.replaceState({}, "", "/");
  }
}

export async function checkSuccessfulVerification(query) {
  if (
    query.get("supportSignUp") === "true" &&
    query.get("supportForgotPassword") === "true" &&
    query.get("message") ===
      "Your email was verified. You can continue using the application." &&
    query.get("success") === "true" &&
    query.get("code") === "success"
  ) {
    history.replaceState({}, "", "/");
    await client.getTokenSilently({ cacheMode: "off" });
    updateUI();
  }
}

export function checkAccessExpired(query) {
  if (
    query.get("message") === "Access expired." &&
    query.get("success") === "false"
  ) {
    history.replaceState({}, "", "/");
  }
}
