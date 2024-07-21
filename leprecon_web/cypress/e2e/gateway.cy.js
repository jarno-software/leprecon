describe("Gateway", () => {
  it("No access token provided", () => {
    cy.request({ url: "/user", failOnStatusCode: false })
      .its("status")
      .should("equal", 401);
  });

  it("Invalid access token", () => {
    const access_token = Cypress.env("access_token");
    cy.request({
      url: "/user",
      failOnStatusCode: false,
      headers: {
        Authorization: `Bearer ${access_token}`,
      },
    })
      .its("status")
      .should("equal", 401);
  });
});
