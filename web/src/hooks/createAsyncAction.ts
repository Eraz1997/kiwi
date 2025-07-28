import { createResource, createSignal } from "solid-js";

export const createAsyncAction = <T extends unknown[]>(
  action: (...input: T) => Promise<void>,
) => {
  const [isLoading, setIsLoading] = createSignal(false);
  const [inputState, setInputState] = createSignal<T>([] as unknown as T);
  createResource(isLoading, async () => {
    await action(...inputState());
    setIsLoading(false);
  });
  return {
    isLoading,
    call: (...input: T) => {
      setInputState(() => input);
      setIsLoading(true);
    },
  };
};
