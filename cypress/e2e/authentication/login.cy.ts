context('Login', () => {
  beforeEach(() => {
    const loginUrl = `${Cypress.env('baseUrl')}${Cypress.env('loginUrl')}`;
    cy.visit(loginUrl);
  });

  it('should redirect back to home on successful login', () => {
    // arrange
    cy.get('#authentication-form-email')
      .type(Cypress.env('mockUserEmail'))
      .should('have.value', Cypress.env('mockUserEmail'));

    cy.get('#authentication-form-password')
      .type(Cypress.env('mockUserPassword'))
      .should('have.value', Cypress.env('mockUserPassword'));

    // act
    cy.get('#authentication-form-submit-button').click();

    // assert
    cy.get('.error-messages').should('not.be.visible');

    cy.location().should((location) =>
      expect(location.href).to.eq(`${Cypress.env('baseUrl')}/`)
    );
  });

  it('should display errors when credentials are not provided', () => {
    // arrange/act
    cy.get('#authentication-form-submit-button').click();

    // assert
    cy.location().should((location) =>
      expect(location.href).to.eq(
        `${Cypress.env('baseUrl')}${Cypress.env('loginUrl')}`
      )
    );

    cy.get('.error-messages')
      .should('be.visible')
      .children()
      .should('contain.text', 'password is required')
      .should('contain.text', 'email is required')
      .should('contain.text', 'email is invalid');
  });

  it('should display a subset of errors when credentials are partially provided', () => {
    // arrange
    cy.get('#authentication-form-email')
      .type(Cypress.env('mockUserEmail'))
      .should('have.value', Cypress.env('mockUserEmail'));

    // act
    cy.get('#authentication-form-submit-button').click();

    // assert
    cy.location().should((location) =>
      expect(location.href).to.eq(
        `${Cypress.env('baseUrl')}${Cypress.env('loginUrl')}`
      )
    );

    cy.get('.error-messages')
      .should('be.visible')
      .children()
      .should('contain.text', 'password is required')
      .should('not.contain.text', 'email is required')
      .should('not.contain.text', 'email is invalid');
  });
});
