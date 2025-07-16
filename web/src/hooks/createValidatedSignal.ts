import { Accessor, Setter, createSignal } from "solid-js";
import zxcvbn from "zxcvbn";

type ValidatedSignal<T> = [Accessor<T>, Setter<T>, () => boolean];

export const createValidatedSignal = <T>(
  validator: (value: T) => boolean,
  initialValue: T,
): ValidatedSignal<T> => {
  const [value, setValue] = createSignal(initialValue);
  const isValid = () => validator(value());

  return [value, setValue, isValid];
};

export const USERNAME_VALIDATOR = (username: string) =>
  !!username.match("^[a-zA-Z0-9.-_]{6,32}$");
export const PASSWORD_VALIDATOR = (password: string) =>
  zxcvbn(password).score === 4;
