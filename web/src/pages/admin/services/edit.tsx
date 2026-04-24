import { LogsViewer } from "./components/logsViewer";
import { ServiceDetailsCard } from "./components/serviceDetailsCard";
import { Component, Show, createEffect } from "solid-js";
import { SetStoreFunction, createStore } from "solid-js/store";
import { Container, HStack, VStack } from "styled-system/jsx";
import { Card } from "~/components";
import { NavigationBar } from "~/components";
import { Spinner } from "~/components";
import { Text } from "~/components";
import { useRouter } from "~/contexts/router";
import { createBackendClient } from "~/hooks/createBackendClient";
import { ContainerConfiguration } from "~/types";

type ContainerInfo = {
  configuration: ContainerConfiguration | null;
  status: string | null;
};

export const AdminServicesEdit: Component = () => {
  const { queryParams } = useRouter();
  const adminClient = createBackendClient("admin");

  const [containerInfo, setContainerInfo] = createStore<ContainerInfo>({
    configuration: null,
    status: null,
  });

  createEffect(async () => {
    const { jsonPayload: service } = await adminClient.get(
      `/services/${queryParams().name}`,
    );
    setContainerInfo(
      "configuration",
      service.general_info.container_configuration,
    );
    setContainerInfo("status", service.status);
  });

  return (
    <>
      <NavigationBar />
      <Container p="12" maxW="4xl" overflowX="scroll">
        <HStack gap="10" alignItems="start">
          <Container w="sm">
            <Show
              when={containerInfo.configuration}
              fallback={
                <VStack gap="6">
                  <Spinner size="xl" />
                  <Text textStyle="lg">Loading service details...</Text>
                </VStack>
              }
            >
              {(configuration) => (
                <ServiceDetailsCard
                  containerConfiguration={configuration()}
                  setContainerConfiguration={
                    setContainerInfo.bind(
                      null,
                      "configuration",
                    ) as SetStoreFunction<ContainerConfiguration>
                  }
                  mode="edit"
                />
              )}
            </Show>
          </Container>
          <VStack w="sm" gap="12">
            <Card.Root width="full">
              <Card.Header>
                <Card.Title>Status</Card.Title>
              </Card.Header>
              <Card.Body>
                <Show when={containerInfo.status} fallback={<Spinner />}>
                  {(status) => capitalise(status())}
                </Show>
              </Card.Body>
            </Card.Root>
            <LogsViewer serviceName={queryParams().name} />
          </VStack>
        </HStack>
      </Container>
    </>
  );
};

const capitalise = (text: string): string => {
  if (!text) return text;
  return `${text[0].toUpperCase()}${text.substring(1)}`;
};
