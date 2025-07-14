import { createResource, createSignal } from "solid-js";

export const createAsyncAction = (action: () => Promise<void>) => {
  const [isLoading, setIsLoading] = createSignal(false);
  createResource(isLoading, async () => {
    await action();
    setIsLoading(false);
  });
  return {
    isLoading,
    call: () => setIsLoading(true),
  };
};
