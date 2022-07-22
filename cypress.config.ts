import { defineConfig } from 'cypress';

export default defineConfig({
  env: {
    baseUrl: 'http://localhost:3000',
    loginUrl: '/login',
    registerUrl: '/register',
    mockUserEmail: 'test@gmail.com',
    mockUserUsername: 'test',
    mockUserPassword: 'test',
  },

  e2e: {
    setupNodeEvents(on, config) {
      // implement node event listeners here
    },
  },
});
