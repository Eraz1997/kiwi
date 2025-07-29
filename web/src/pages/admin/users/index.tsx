import { DeleteUserDialog } from "./deleteUserDialog";
import { InviteUserDialog } from "./inviteUserDialog";
import { X } from "lucide-solid";
import { Component, For, createResource } from "solid-js";
import { NavigationBar } from "src/components/navigationBar";
import { IconButton } from "src/components/ui/icon-button";
import { Table } from "src/components/ui/table";
import { Toast } from "src/components/ui/toast";
import { createBackendClient } from "src/hooks/createBackendClient";
import { User } from "src/types";
import { Container } from "styled-system/jsx";

const toaster = Toast.createToaster({
  placement: "bottom-end",
  overlap: true,
  gap: 16,
});

export const AdminUsers: Component = () => {
  const adminClient = createBackendClient("admin");

  const [currentUser] = createResource<User>(async () => {
    const { jsonPayload: user } = await adminClient.get("/me");
    return user;
  });
  const [users, { refetch: reloadUsers }] = createResource<User[]>(async () => {
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
              <Table.Header textAlign="end">Role</Table.Header>
              <Table.Header />
            </Table.Row>
          </Table.Head>
          <Table.Body>
            <For each={users()}>
              {(user) => (
                <Table.Row>
                  <Table.Cell fontWeight="medium">{user.username}</Table.Cell>
                  <Table.Cell textAlign="end">{user.role}</Table.Cell>
                  <Table.Cell width="24" textAlign="end">
                    <DeleteUserDialog
                      userToDelete={user}
                      authenticatedUser={currentUser()}
                      reloadUsers={reloadUsers}
                      createToast={toaster.create}
                    />
                  </Table.Cell>
                </Table.Row>
              )}
            </For>
          </Table.Body>
          <Table.Foot>
            <Table.Row>
              <Table.Cell />
              <Table.Cell />
              <Table.Cell textAlign="end">
                <InviteUserDialog createToast={toaster.create} />
              </Table.Cell>
            </Table.Row>
          </Table.Foot>
        </Table.Root>
      </Container>
      <Toast.Toaster toaster={toaster}>
        {(toast) => {
          const color = toast().type === "success" ? "lime" : "red";
          return (
            <Toast.Root borderColor={`${color}.default`}>
              <Toast.Title color={`${color}.text`}>{toast().title}</Toast.Title>
              <Toast.Description color={`${color}.text`}>
                {toast().description}
              </Toast.Description>
              <Toast.CloseTrigger color={`${color}.text`}>
                <IconButton size="sm" variant="link">
                  <X />
                </IconButton>
              </Toast.CloseTrigger>
            </Toast.Root>
          );
        }}
      </Toast.Toaster>
    </>
  );
};
