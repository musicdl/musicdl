import { validateUsername, validatePassword } from "../src/utils/validation";

describe("Login", () => {
  describe("Username Validation", () => {
    test("enter valid username", () => {
      expect(validateUsername("Dhruvin123")).toBe(true);
    });
    test("enter invalid username", () => {
      expect(validateUsername("hi")).toBe(false);
    });
  });
  describe("Password Validation", () => {
    test("enter valid password", () => {
      expect(validatePassword("Str0ngP@ssword")).toBe(true);
    });
    test("enter invalid password", () => {
      expect(validatePassword("1234")).toBe(false);
    });
  });
});
