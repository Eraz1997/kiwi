import { ArrowRight, Plus } from "lucide-solid";
import { Component, For, createResource } from "solid-js";
import { NavigationBar } from "src/components/navigationBar";
import { Button } from "src/components/ui/button";
import { Table } from "src/components/ui/table";
import { createBackendClient } from "src/hooks/createBackendClient";
import { Container } from "styled-system/jsx";

export const AdminUsers: Component = () => {
  const adminClient = createBackendClient("admin");
  const [users] = createResource<User[]>(async () => {
    const { jsonPayload } = await adminClient.get("/users");
    return jsonPayload;
  });

  return (
    <>
      <NavigationBar />
      <Container p={{ base: "12" }} maxW="4xl">
        <Table.Root>
          <Table.Head>
            <Table.Row>
              <Table.Header>Username</Table.Header>
              <Table.Header textAlign="right">Role</Table.Header>
              <Table.Header />
            </Table.Row>
          </Table.Head>
          <Table.Body>
            <For each={users()}>
              {(user) => (
                <Table.Row>
                  <Table.Cell fontWeight="medium">{user.username}</Table.Cell>
                  <Table.Cell textAlign="right">{user.role}</Table.Cell>
                  <Table.Cell width="24" textAlign="right">
                    <Button size="xs" onClick={() => null}>
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
              <Table.Cell textAlign="right">
                <Button size="xs" onClick={() => null}>
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
