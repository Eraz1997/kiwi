import { ArrowRight, Plus } from "lucide-solid";
import { Component, For, Show, createResource } from "solid-js";
import { NavigationBar } from "src/components/navigationBar";
import { Button } from "src/components/ui/button";
import { Table } from "src/components/ui/table";
import { useRouter } from "src/contexts/router";
import { createBackendClient } from "src/hooks/createBackendClient";
import { Service } from "src/types";
import { Container } from "styled-system/jsx";

export const AdminServices: Component = () => {
  const { navigate } = useRouter();
  const adminClient = createBackendClient("admin");

  const [services] = createResource<Service[]>(async () => {
    const { jsonPayload } = await adminClient.get("/services");
    const services: Service[] = jsonPayload.services;
    services.forEach((service) => {
      service.created_at = new Date(service.created_at);
      service.last_modified_at = new Date(service.last_modified_at);
      service.last_deployed_at = new Date(service.last_deployed_at);
    });

    return services;
  });

  return (
    <>
      <NavigationBar />
      <Container p={{ base: "12" }} maxW="4xl">
        <Table.Root>
          <Table.Head>
            <Table.Row>
              <Table.Header>Name</Table.Header>
              <Table.Header>Repository</Table.Header>
              <Table.Header>Created</Table.Header>
              <Table.Header>Last Modified</Table.Header>
              <Table.Header>Last Deployed</Table.Header>
              <Table.Header />
            </Table.Row>
          </Table.Head>
          <Table.Body>
            <For each={services()}>
              {(service) => (
                <Table.Row>
                  <Table.Cell fontWeight="medium">
                    {service.container_configuration.name}
                  </Table.Cell>
                  <Table.Cell>
                    <Show
                      when={service.container_configuration.github_repository}
                      fallback="-"
                    >
                      {(repo) => `${repo().owner}/${repo().name}`}
                    </Show>
                  </Table.Cell>
                  <Table.Cell>{formatDate(service.created_at)}</Table.Cell>
                  <Table.Cell>
                    {formatDate(service.last_modified_at)}
                  </Table.Cell>
                  <Table.Cell>
                    {formatDate(service.last_deployed_at)}
                  </Table.Cell>
                  <Table.Cell width="24" textAlign="end">
                    <Button
                      size="xs"
                      onClick={() =>
                        navigate("admin/services/edit", {
                          name: service.container_configuration.name,
                        })
                      }
                    >
                      <ArrowRight />
                    </Button>
                  </Table.Cell>
                </Table.Row>
              )}
            </For>
          </Table.Body>
          <Table.Foot>
            <Table.Row>
              <Table.Cell />
              <Table.Cell />
              <Table.Cell />
              <Table.Cell />
              <Table.Cell />
              <Table.Cell textAlign="end">
                <Button
                  size="xs"
                  onClick={() => navigate("admin/services/new")}
                >
                  <Plus />
                </Button>
              </Table.Cell>
            </Table.Row>
          </Table.Foot>
        </Table.Root>
      </Container>
    </>
  );
};

const formatDate = (date: Date): string => {
  return `${date.getDate()}/${date.getMonth()}/${date.getFullYear()}`;
};
