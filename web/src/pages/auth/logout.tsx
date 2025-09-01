import { Container } from "../../../styled-system/jsx/container";
import { Component, onMount } from "solid-js";
import { Spinner } from "src/components/ui/spinner";
import { useRouter } from "src/contexts/router";
import { createAsyncAction } from "src/hooks/createAsyncAction";
import { createBackendClient } from "src/hooks/createBackendClient";
import { HStack, Spacer } from "styled-system/jsx";

export const Logout: Component = () => {
  const { navigate } = useRouter();
  const authClient = createBackendClient("auth");

  const { call: logout } = createAsyncAction(async () => {
    await authClient.post("/logout");
    navigate("auth/login");
  });

  onMount(() => {
    logout();
  });

  return (
    <Container p="12" maxW="md">
      <HStack>
        <Spacer />
        <Spinner size="md" />
        <Spacer />
      </HStack>
    </Container>
  );
};
