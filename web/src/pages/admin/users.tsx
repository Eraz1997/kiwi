import { Bomb, Plus, Trash2, X } from "lucide-solid";
import { Component, For, createResource } from "solid-js";
import { NavigationBar } from "src/components/navigationBar";
import { Button } from "src/components/ui/button";
import { Dialog } from "src/components/ui/dialog";
import { IconButton } from "src/components/ui/icon-button";
import { Table } from "src/components/ui/table";
import { Toast } from "src/components/ui/toast";
import { createAsyncAction } from "src/hooks/createAsyncAction";
import { createBackendClient } from "src/hooks/createBackendClient";
import { Container, HStack, Spacer, VStack } from "styled-system/jsx";

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

  const { call: deleteUser } = createAsyncAction(async (username: string) => {
    const { statusCode, jsonPayload: errorMessage } = await adminClient.delete(
      "/user",
      { username },
    );

    if (statusCode === 200) {
      toaster.create({
        title: "User deleted",
        description: `"${username}" was successfully deleted.`,
        type: "success",
      });
    } else {
      toaster.create({
        title: "Failed",
        description: `We couldn't delete "${username}": ${errorMessage}.`,
        type: "error",
      });
    }
    reloadUsers();
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
                    <Dialog.Root>
                      <Dialog.Trigger
                        asChild={(triggerProps) => (
                          <Button
                            {...triggerProps}
                            size="xs"
                            bgColor={{ base: "red.7", _hover: "red.8" }}
                            disabled={user.username === currentUser()?.username}
                          >
                            <Trash2 />
                          </Button>
                        )}
                      />
                      <Dialog.Backdrop />
                      <Dialog.Positioner>
                        <Dialog.Content>
                          <VStack gap="8" p="6" maxW="md">
                            <Dialog.Title>Delete User</Dialog.Title>
                            <Dialog.Description>
                              This action cannot be reverted. Please make sure
                              this is an intended action. All user data will be
                              lost. Do you want to proceeed?
                            </Dialog.Description>
                            <HStack gap="3" width="full">
                              <Spacer />
                              <Dialog.CloseTrigger
                                asChild={(closeTriggerProps) => (
                                  <Button
                                    {...closeTriggerProps()}
                                    variant="outline"
                                  >
                                    Cancel
                                  </Button>
                                )}
                              />
                              <Dialog.CloseTrigger
                                asChild={(closeTriggerProps) => (
                                  <Button
                                    {...closeTriggerProps()}
                                    bgColor={{ base: "red.7", _hover: "red.8" }}
                                    onClick={() => deleteUser(user.username)}
                                  >
                                    Confirm
                                    <Bomb />
                                  </Button>
                                )}
                              />
                            </HStack>
                          </VStack>
                          <Dialog.CloseTrigger
                            asChild={(closeTriggerProps) => (
                              <IconButton
                                {...closeTriggerProps()}
                                aria-label="Close Dialog"
                                variant="ghost"
                                size="sm"
                                position="absolute"
                                top="2"
                                right="2"
                              >
                                <X />
                              </IconButton>
                            )}
                          />
                        </Dialog.Content>
                      </Dialog.Positioner>
                    </Dialog.Root>
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
      <Toast.Toaster toaster={toaster}>
        {(toast) => {
          const color = toast().type === "success" ? "lime" : "red";
          return (
            <Toast.Root borderColor={`${color}.default`}>
              <Toast.Title color={`${color}.text`}>{toast().title}</Toast.Title>
              <Toast.Description color={`${color}.text`}>
                {toast().description}
              </Toast.Description>
              <Toast.CloseTrigger
                color={`${color}.text`}
                asChild={(closeProps) => (
                  <IconButton {...closeProps()} size="sm" variant="link">
                    <X />
                  </IconButton>
                )}
              />
            </Toast.Root>
          );
        }}
      </Toast.Toaster>
    </>
  );
};
