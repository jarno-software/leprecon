const { defineConfig } = require("cypress");

module.exports = defineConfig({
  e2e: {
    baseUrl: "https://127.0.0.1",
    video: false,
    chromeWebSecurity: false,
    setupNodeEvents(on, config) {
      require("@cypress/code-coverage/task")(on, config);
      return config;
    },
  },
  env: {
    access_token:
      "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjdlYWtPdDMxbi1jS044OU9WbW1ISCJ9.eyJpc3MiOiJodHRwczovL2Rldi0wbjh4YnN5aXA3b20zdmF2LmV1LmF1dGgwLmNvbS8iLCJzdWIiOiJhdXRoMHw2NjJlOGI3YTYwY2Q4OTBmNGRlMzYzNTIiLCJhdWQiOlsiaHR0cDovLzEyNy4wLjAuMTo4MCIsImh0dHBzOi8vZGV2LTBuOHhic3lpcDdvbTN2YXYuZXUuYXV0aDAuY29tL3VzZXJpbmZvIl0sImlhdCI6MTcxNDY0OTM1NCwiZXhwIjoxNzE0NjQ5NjU0LCJzY29wZSI6Im9wZW5pZCBwcm9maWxlIGVtYWlsIG9mZmxpbmVfYWNjZXNzIiwiYXpwIjoiSnVEYW82M3AwQnRIOVcydXVUbFpJeGZnbExUNVdVUnQiLCJwZXJtaXNzaW9ucyI6WyJzZW5kOnZlcmlmaWNhdGlvbi1tYWlsIl19.Q5jvaI9gQQKNEefvfwZcCQC5sYxgnN1Ugxw4ygoE85k8DcjX4K4iJ_0tagBMnAQ4_478AMu29RQC2tFvf08NArEOXwLnNdbPcapVWQluaSPpTc3HNATAmPN0NRb80SQwHEMXwNADE_uA_d9HxUhOGfr094sVPVifVpj2ZUlpS5DxdCm2_JAfUT53ncmYKt3Ioc6pNCehaiSrIsDg9SgIBKeHhKPNJ7k2MacFzReYppUb3lx3oONzOfrNL7W5pjS-xgxdRvWMUlvACnk3orVpkGtLPNPCcCngDwoEygzST1UsNOIHxmKe1wdRhVSkHjY1x8uWzaP6DZqfT9Qohor9GA",
  },
});
