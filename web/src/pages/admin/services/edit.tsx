import { LogsViewer } from "./components/logsViewer";
import { ServiceDetailsCard } from "./components/serviceDetailsCard";
import { Component, Show, createResource } from "solid-js";
import { createStore, produce } from "solid-js/store";
import { NavigationBar } from "src/components/navigationBar";
import { Spinner } from "src/components/ui/spinner";
import { Text } from "src/components/ui/text";
import { useRouter } from "src/contexts/router";
import { createBackendClient } from "src/hooks/createBackendClient";
import { ContainerConfiguration } from "src/types";
import { Container, HStack, VStack } from "styled-system/jsx";

export const AdminServicesEdit: Component = () => {
  const { queryParams } = useRouter();
  const adminClient = createBackendClient("admin");

  const [configuration, setConfiguration] = createStore<ContainerConfiguration>(
    {
      name: "",
      image_name: "",
      image_sha: { value: "" },
      exposed_port: { internal: 0, external: 0 },
      environment_variables: [],
      secrets: [],
      internal_secrets: [],
      stateful_volume_paths: [],
      github_repository: null,
    },
  );

  const [{ loading }] = createResource<null>(async () => {
    const { jsonPayload: service } = await adminClient.get(
      `/services/${queryParams().name}`,
    );
    setConfiguration(
      produce((config) =>
        Object.assign(config, service.container_configuration),
      ),
    );
    return null;
  });

  return (
    <>
      <NavigationBar />
      <Container p={{ base: "12" }} maxW="4xl">
        <HStack gap="10">
          <Container w="40vw">
            <Show when={loading}>
              <VStack gap="6">
                <Spinner size="xl" />
                <Text size="lg">Loading service details...</Text>
              </VStack>
            </Show>
            <Show when={!loading}>
              <ServiceDetailsCard
                containerConfiguration={configuration}
                setContainerConfiguration={setConfiguration}
                mode="edit"
              />
            </Show>
          </Container>
          <Container w="40vw">
            <Show when={loading}>
              <VStack gap="6">
                <Spinner size="xl" />
                <Text size="lg">Loading service logs...</Text>
              </VStack>
            </Show>
            <Show when={!loading}>
              <LogsViewer serviceName={configuration.name} />
            </Show>
          </Container>
        </HStack>
      </Container>
    </>
  );
};
