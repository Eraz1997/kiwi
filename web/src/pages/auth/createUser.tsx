import { Container } from "../../../styled-system/jsx/container";
import { Atom, CircleSlash, Sparkle } from "lucide-solid";
import { Component, Match, Show, Switch, createSignal } from "solid-js";
import { Alert } from "src/components/ui/alert";
import { Button } from "src/components/ui/button";
import { Card } from "src/components/ui/card";
import { Field } from "src/components/ui/field";
import { Heading } from "src/components/ui/heading";
import { useRouter } from "src/contexts/router";
import { createAsyncAction } from "src/hooks/createAsyncAction";
import { createBackendClient } from "src/hooks/createBackendClient";
import { createCredentialsClient } from "src/hooks/createCredentialsClient";
import {
  PASSWORD_VALIDATOR,
  USERNAME_VALIDATOR,
  createValidatedSignal,
} from "src/hooks/createValidatedSignal";
import { VStack } from "styled-system/jsx";

type CreationError = "unknown" | "bad invitation" | "invalid credentials";

export const CreateUser: Component = () => {
  const [username, setUsername, isUsernameValid] =
    createValidatedSignal<string>(USERNAME_VALIDATOR, "");
  const [password, setPassword, isPasswordValid] =
    createValidatedSignal<string>(PASSWORD_VALIDATOR, "");
  const [error, setError] = createSignal<CreationError | null>();
  const [success, setSuccess] = createSignal(false);

  const authBackendClient = createBackendClient("auth");
  const credentialsClient = createCredentialsClient();
  const { queryParams } = useRouter();

  const { isLoading, call: createUser } = createAsyncAction(async () => {
    const passwordHash =
      await credentialsClient.getLoginPasswordHash(password());

    const result = await authBackendClient.post("/create-user", {
      username: username(),
      password_hash: passwordHash,
      invitation_id: queryParams().invitation_id,
    });

    if (result.statusCode === 401) {
      setError("bad invitation");
    } else if (result.statusCode === 400) {
      setError("invalid credentials");
    } else if (result.statusCode >= 400) {
      setError("unknown");
    } else {
      setSuccess(true);
      setError(null);
    }
  });

  return (
    <Container p={{ base: "12" }} maxW="md">
      <VStack gap="6">
        <Heading size="6xl">🥝</Heading>
        <Show when={success()}>
          <Alert.Root borderColor="lime.default">
            <Alert.Icon
              color="lime.text"
              asChild={(iconProps) => <Sparkle {...iconProps()} />}
            />
            <Alert.Content>
              <Alert.Title color="lime.text">User Created</Alert.Title>
              <Alert.Description color="lime.text">
                Your user has been created and you are now logged in. Go visit
                Kiwi services!
              </Alert.Description>
            </Alert.Content>
          </Alert.Root>
        </Show>
        <Show when={error()}>
          <Alert.Root borderColor="red.default">
            <Alert.Icon
              color="red.text"
              asChild={(iconProps) => <CircleSlash {...iconProps()} />}
            />
            <Alert.Content>
              <Alert.Title color="red.text">Access denied</Alert.Title>
              <Alert.Description color="red.text">
                <Switch>
                  <Match when={error() === "bad invitation"}>
                    The invitation code is invalid.
                  </Match>
                  <Match when={error() === "invalid credentials"}>
                    The credentials you submitted are invalid.
                  </Match>
                  <Match when={error() === "unknown"}>
                    Something went wrong.
                  </Match>
                </Switch>
              </Alert.Description>
            </Alert.Content>
          </Alert.Root>
        </Show>
        <Card.Root>
          <Card.Header>
            <Card.Title>Create User</Card.Title>
            <Card.Description>
              Choose a username and a password for your Kiwi account.
            </Card.Description>
          </Card.Header>
          <Card.Body>
            <VStack gap="4">
              <Field.Root
                width="full"
                invalid={!!username() && !isUsernameValid()}
              >
                <Field.Label>Username</Field.Label>
                <Field.Input
                  onChange={(event) => setUsername(event.target.value)}
                />
                <Field.ErrorText>Please enter a valid username</Field.ErrorText>
              </Field.Root>
              <Field.Root
                width="full"
                invalid={!!password() && !isPasswordValid()}
              >
                <Field.Label>Password</Field.Label>
                <Field.Input
                  type="password"
                  onChange={(event) => setPassword(event.target.value)}
                />
                <Field.ErrorText>Please enter a valid password</Field.ErrorText>
              </Field.Root>
            </VStack>
          </Card.Body>
          <Card.Footer>
            <Button
              loading={isLoading()}
              disabled={!isUsernameValid() || !isPasswordValid() || success()}
              onClick={() => createUser()}
            >
              Create User
              <Atom />
            </Button>
          </Card.Footer>
        </Card.Root>
      </VStack>
    </Container>
  );
};
