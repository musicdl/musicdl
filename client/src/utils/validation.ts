import { USERNAME_MIN_LENGTH, PASSWORD_MIN_LENGTH } from "./constants"

export const validateUsername = (u: string) => {
  return !(u.length < USERNAME_MIN_LENGTH);
}

export const validatePassword = (p: string) => {
  return !(p.length < PASSWORD_MIN_LENGTH);
}
