import { CircleX, Earth, Orbit, Power } from "lucide-solid";
import {
  Component,
  Match,
  Show,
  Switch,
  createResource,
  createSignal,
} from "solid-js";
import { NavigationBar } from "src/components/navigationBar";
import { Alert } from "src/components/ui/alert";
import { Button } from "src/components/ui/button";
import { Card } from "src/components/ui/card";
import { Field } from "src/components/ui/field";
import { Spinner } from "src/components/ui/spinner";
import { Text } from "src/components/ui/text";
import { useRouter } from "src/contexts/router";
import { createAsyncAction } from "src/hooks/createAsyncAction";
import { createBackendClient } from "src/hooks/createBackendClient";
import { Container, HStack, VStack } from "styled-system/jsx";

export const DynamicDns: Component = () => {
  const adminClient = createBackendClient("admin");

  const { domain } = useRouter();

  const [apiKey, setApiKey] = createSignal<string>();
  const [apiSecret, setApiSecret] = createSignal<string>();
  const [error, setError] = createSignal<string>();
  const [isDynamicDnsEnabled, { refetch: reloadState }] =
    createResource<boolean>(async () => {
      const { jsonPayload } = await adminClient.get("/dynamic-dns");
      return jsonPayload.enabled;
    });

  const { call: enableDynamicDns, isLoading: isEnableLoading } =
    createAsyncAction(async () => {
      const { statusCode, text: errorMessage } = await adminClient.put(
        "/dynamic-dns",
        {
          provider: "GoDaddy",
          authorization_header: {
            value: `sso-key ${apiKey()}: ${apiSecret()}`,
          },
          domain: { value: domain() },
        },
      );

      if (statusCode === 200) {
        reloadState();
      } else {
        setError(errorMessage ?? "unknown error");
      }
    });

  const { call: disableDynamicDns, isLoading: isDisableLoading } =
    createAsyncAction(async () => {
      const { statusCode, text: errorMessage } =
        await adminClient.delete("/dynamic-dns");

      if (statusCode === 200) {
        reloadState();
      } else {
        setError(errorMessage ?? "unknown error");
      }
    });

  const isAnythingLoading = () => isEnableLoading() || isDisableLoading();

  return (
    <>
      <NavigationBar />
      <Container p="12" maxW="md">
        <VStack gap="6">
          <Show when={error()}>
            <Alert.Root borderColor="red.default">
              <Alert.Icon
                color="red.text"
                asChild={(iconProps) => <CircleX {...iconProps()} />}
              />
              <Alert.Content>
                <Alert.Title color="red.text">Something went wrong</Alert.Title>
                <Alert.Description color="red.text">
                  {error()}
                </Alert.Description>
              </Alert.Content>
            </Alert.Root>
          </Show>
          <Show
            when={!isDynamicDnsEnabled.loading}
            fallback={<Spinner size="xl" />}
          >
            <Card.Root>
              <Card.Header>
                <Card.Title>
                  <Switch>
                    <Match when={isDynamicDnsEnabled()}>
                      Edit Dynamic DNS Configuration
                    </Match>
                    <Match when={!isDynamicDnsEnabled()}>
                      Enable Dynamic DNS
                    </Match>
                  </Switch>
                </Card.Title>
              </Card.Header>
              <Card.Body>
                <VStack gap="4">
                  <Text size="md">
                    Only GoDaddy is supported as a provider at this time.
                  </Text>
                  <Field.Root width="full">
                    <Field.Label>API Key</Field.Label>
                    <Field.Input
                      onChange={(event) => setApiKey(event.target.value)}
                      value={apiKey()}
                      type="password"
                    />
                  </Field.Root>
                  <Field.Root width="full">
                    <Field.Label>API Secret</Field.Label>
                    <Field.Input
                      onChange={(event) => setApiSecret(event.target.value)}
                      value={apiSecret()}
                      type="password"
                    />
                  </Field.Root>
                </VStack>
              </Card.Body>
              <Card.Footer>
                <Switch>
                  <Match when={isDynamicDnsEnabled()}>
                    <HStack gap="4">
                      <Button
                        size="xs"
                        bgColor={{ base: "red.7", _hover: "red.8" }}
                        loading={isAnythingLoading()}
                        onClick={disableDynamicDns}
                      >
                        Disable
                        <Power />
                      </Button>
                      <Button
                        bgColor={{
                          base: "amber.light.9",
                          _hover: "amber.light.11",
                        }}
                        loading={isAnythingLoading()}
                        onClick={enableDynamicDns}
                        disabled={!apiKey() || !apiSecret()}
                      >
                        Save Any Changes
                        <Orbit />
                      </Button>
                    </HStack>
                  </Match>
                  <Match when={!isDynamicDnsEnabled()}>
                    <Button
                      loading={isAnythingLoading()}
                      onClick={enableDynamicDns}
                      disabled={!apiKey() || !apiSecret()}
                    >
                      Enable <Earth />
                    </Button>
                  </Match>
                </Switch>
              </Card.Footer>
            </Card.Root>
          </Show>
        </VStack>
      </Container>
    </>
  );
};
