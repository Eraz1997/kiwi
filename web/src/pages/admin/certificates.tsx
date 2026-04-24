import {
  Beer,
  CalendarClock,
  CircleX,
  Signature,
  Telescope,
} from "lucide-solid";
import { Component, Show, createResource, createSignal } from "solid-js";
import { Container, HStack, VStack } from "styled-system/jsx";
import { Alert } from "~/components";
import { Button } from "~/components";
import { Card } from "~/components";
import { Heading } from "~/components";
import { NavigationBar } from "~/components";
import { Spinner } from "~/components";
import { Text } from "~/components";
import { useRouter } from "~/contexts/router";
import { createAsyncAction } from "~/hooks/createAsyncAction";
import { createBackendClient } from "~/hooks/createBackendClient";

type CertificateInfo = {
  issuer: string;
  expiration_date: string;
  new_pending_order: boolean;
};

export const Certificates: Component = () => {
  const adminClient = createBackendClient("admin");

  const { domain } = useRouter();

  const [error, setError] = createSignal<string>();
  const [certificateInfo, { refetch: reloadState }] =
    createResource<CertificateInfo>(async () => {
      const { jsonPayload } = await adminClient.get("/certificates");
      return jsonPayload;
    });

  const { call: orderNewCertificate, isLoading: isNewOrderLoading } =
    createAsyncAction(async () => {
      const { statusCode, text: errorMessage } = await adminClient.post(
        "/certificates",
        { domain: domain() },
      );

      if (statusCode === 200) {
        reloadState();
      } else {
        setError(errorMessage ?? "unknown error");
      }
    });

  const { call: verifyDns, isLoading: isVerifyingDns } = createAsyncAction(
    async () => {
      const { statusCode, text: errorMessage } =
        await adminClient.post("/finalise");

      if (statusCode === 200) {
        reloadState();
      } else {
        setError(errorMessage ?? "unknown error");
      }
    },
  );

  const isAnythingLoading = () => isNewOrderLoading() || isVerifyingDns();

  return (
    <>
      <NavigationBar />
      <Container p="12" maxW="md" overflowX="scroll">
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
            when={!certificateInfo.loading}
            fallback={<Spinner size="xl" />}
          >
            <Card.Root>
              <Card.Header>
                <Card.Title>TLS Certificate Info</Card.Title>
              </Card.Header>
              <Card.Body>
                <VStack gap="4" alignItems="start" width="full">
                  <Text textStyle="xs">
                    TLS certificates are issued by Let's Encrypt for free.
                  </Text>
                  <Heading textStyle="md" display="flex" gap="2">
                    Issuer <Signature />
                  </Heading>
                  <Text>{certificateInfo()?.issuer}</Text>
                  <Heading textStyle="md" display="flex" gap="2">
                    Expiration <CalendarClock />
                  </Heading>
                  <Text>{certificateInfo()?.expiration_date}</Text>
                </VStack>
              </Card.Body>
              <Card.Footer>
                <HStack gap="4">
                  <Button
                    bgColor={{
                      base: "amber.light.9",
                      _hover: "amber.light.11",
                    }}
                    loading={isAnythingLoading()}
                    disabled={!certificateInfo()?.new_pending_order}
                    onClick={verifyDns}
                  >
                    Verify DNS
                    <Telescope />
                  </Button>
                  <Button
                    loading={isAnythingLoading()}
                    onClick={orderNewCertificate}
                  >
                    Order New Certificate
                    <Beer />
                  </Button>
                </HStack>
              </Card.Footer>
            </Card.Root>
          </Show>
        </VStack>
      </Container>
    </>
  );
};
