import { v4 as uuidv4 } from 'uuid';

context('register', () => {
  beforeEach(() => {
    const registerUrl = `${Cypress.env('baseUrl')}${Cypress.env(
      'registerUrl'
    )}`;
    cy.visit(registerUrl);
  });

  it('should contain the "Have an account?" link and navigate to login when clicked', () => {
    // arrange
    const link = cy.contains('a', 'Have an account?').should('be.visible');

    // act
    link.click();

    // assert
    cy.location().should((location) =>
      expect(location.href).to.eq(
        `${Cypress.env('baseUrl')}${Cypress.env('loginUrl')}`
      )
    );
  });

  it('should contain the "Sign up" submit button', () => {
    // singular assert
    cy.get('#authentication-form-submit-button')
      .should('be.visible')
      .should('contain.text', 'Sign up');
  });

  it('should redirect back to home on successful register', () => {
    // arrange, generate a random user ID and email so we don't clash
    const username = uuidv4();
    const email = `${username}@gmail.com`;

    cy.get('#authentication-form-email')
      .type(email)
      .should('have.value', email);

    cy.get('#authentication-form-username')
      .type(username)
      .should('have.value', username);

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
        `${Cypress.env('baseUrl')}${Cypress.env('registerUrl')}`
      )
    );

    cy.get('.error-messages')
      .should('be.visible')
      .children()
      .should('contain.text', 'username is required')
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
        `${Cypress.env('baseUrl')}${Cypress.env('registerUrl')}`
      )
    );

    cy.get('.error-messages')
      .should('be.visible')
      .children()
      .should('contain.text', 'password is required')
      .should('contain.text', 'username is required')
      .should('not.contain.text', 'email is required')
      .should('not.contain.text', 'email is invalid');
  });
});
