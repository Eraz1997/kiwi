import { Container } from "../../../styled-system/jsx/container";
import { Component, onMount } from "solid-js";
import { HStack, Spacer } from "styled-system/jsx";
import { Spinner } from "~/components";
import { useRouter } from "~/contexts/router";
import { createAsyncAction } from "~/hooks/createAsyncAction";
import { createBackendClient } from "~/hooks/createBackendClient";

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
