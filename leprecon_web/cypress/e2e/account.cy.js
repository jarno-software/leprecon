describe("Account", () => {
  beforeEach(() => {
    cy.fixture("users.json").then((obj) => {
      cy.login(obj[0]);
    });
  });

  it("Send verification email", () => {
    cy.get("#send-verification-mail").click();

    cy.get('[id^="succes-message"]').should(
      "contain",
      "Succesfully send email",
    );
  });

  it("Already send verification email", () => {
    cy.get("#send-verification-mail").click();

    cy.get('[id^="error-message"]').should("contain", "Already send email");
  });
});
