import { Container } from "../../styled-system/jsx/container";
import { CircleSlash, Rocket } from "lucide-solid";
import { Component, Match, Show, Switch, createSignal } from "solid-js";
import { Alert } from "src/components/ui/alert";
import { Button } from "src/components/ui/button";
import { Card } from "src/components/ui/card";
import { Field } from "src/components/ui/field";
import { Heading } from "src/components/ui/heading";
import { useRouter } from "src/contexts/router";
import { createBackendClient } from "src/hooks/createBackendClient";
import { createCredentialsClient } from "src/hooks/createCredentialsClient";
import { VStack } from "styled-system/jsx";
import zxcvbn from "zxcvbn";

type LoginError = "unknown" | "bad credentials";

export const Login: Component = () => {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [isLoading, setIsLoading] = createSignal(false);
  const [error, setError] = createSignal<LoginError | null>();

  const isUsernameValid = () =>
    username().length >= 6 &&
    username().length <= 32 &&
    username().match("^[a-zA-Z0-9.-_]+$");
  const isPasswordValid = () => zxcvbn(password()).score === 4;

  const authBackendClient = createBackendClient("auth");
  const credentialsClient = createCredentialsClient();
  const { domain, isLocalhost, queryParams } = useRouter();

  const signIn = async () => {
    setIsLoading(true);

    const passwordHash =
      await credentialsClient.getLoginPasswordHash(password());

    const result = await authBackendClient.post("/login", {
      username: username(),
      password_hash: passwordHash,
    });

    if (result.statusCode === 401) {
      setError("bad credentials");
      setIsLoading(false);
    } else if (result.statusCode >= 400) {
      setError("unknown");
      setIsLoading(false);
    } else {
      const scheme = isLocalhost() ? "http://" : "https://";
      const returnUri = queryParams().return_uri ?? `${scheme}admin.${domain}`;
      await credentialsClient.storeAndSealLocalEncryptionKey(
        username(),
        password(),
      );
      window.location.replace(returnUri);
    }
  };

  return (
    <Container p={{ base: "12" }} maxW="md">
      <VStack gap="6">
        <Heading size="6xl">🥝</Heading>
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
                  <Match when={error() === "bad credentials"}>
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
            <Card.Title>Sign In</Card.Title>
            <Card.Description>
              Insert your username and password to sign in to Kiwi.
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
              disabled={!isUsernameValid() || !isPasswordValid()}
              onClick={signIn}
            >
              Sign In
              <Rocket />
            </Button>
          </Card.Footer>
        </Card.Root>
      </VStack>
    </Container>
  );
};
