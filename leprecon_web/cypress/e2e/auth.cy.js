describe("Auth", () => {
  beforeEach(() => {
    cy.fixture("users.json").then((obj) => {
      cy.login(obj[0]);
    });
  });
  it("Logout", () => {
    cy.get("#username").should("contain", "swiftyshadower");

    cy.get("#logout").click();

    cy.get("#username").should("be.hidden");
  });
});
