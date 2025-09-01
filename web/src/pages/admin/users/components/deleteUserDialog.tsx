import { Bomb, Trash2, X } from "lucide-solid";
import { Component } from "solid-js";
import { Button } from "src/components/ui/button";
import { Dialog } from "src/components/ui/dialog";
import { IconButton } from "src/components/ui/icon-button";
import { createAsyncAction } from "src/hooks/createAsyncAction";
import { createBackendClient } from "src/hooks/createBackendClient";
import { User } from "src/types";
import { HStack, Spacer, VStack } from "styled-system/jsx";

type Props = {
  userToDelete: User;
  authenticatedUser?: User;
  reloadUsers: () => void;
  createToast: (toast: {
    title: string;
    description: string;
    type: string;
  }) => void;
};

export const DeleteUserDialog: Component<Props> = (props) => {
  const adminClient = createBackendClient("admin");

  const { call: deleteUser, isLoading } = createAsyncAction(
    async (username: string) => {
      const { statusCode, text: errorMessage } = await adminClient.delete(
        "/users",
        { username },
      );

      if (statusCode === 200) {
        props.createToast({
          title: "User deleted",
          description: `"${username}" was successfully deleted.`,
          type: "success",
        });
      } else {
        props.createToast({
          title: "Failed",
          description: `We couldn't delete "${username}": ${errorMessage ?? "unknown error"}.`,
          type: "error",
        });
      }
      props.reloadUsers();
    },
  );

  return (
    <>
      <Dialog.Root>
        <Dialog.Trigger>
          <Button
            size="xs"
            bgColor={{ base: "red.7", _hover: "red.8" }}
            disabled={
              props.userToDelete.username === props.authenticatedUser?.username
            }
          >
            <Trash2 />
          </Button>
        </Dialog.Trigger>
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content textAlign="start">
            <VStack gap="8" p="6" maxW="md">
              <Dialog.Title>Delete User</Dialog.Title>
              <Dialog.Description>
                This action cannot be reverted. Please make sure this is an
                intended action. All user data will be lost. Do you want to
                proceeed?
              </Dialog.Description>
              <HStack gap="3" width="full">
                <Spacer />
                <Dialog.CloseTrigger>
                  <Button variant="outline">Cancel</Button>
                </Dialog.CloseTrigger>
                <Dialog.CloseTrigger>
                  <Button
                    bgColor={{ base: "red.7", _hover: "red.8" }}
                    loading={isLoading()}
                    onClick={() => deleteUser(props.userToDelete.username)}
                  >
                    Confirm
                    <Bomb />
                  </Button>
                </Dialog.CloseTrigger>
              </HStack>
            </VStack>
            <Dialog.CloseTrigger>
              <IconButton
                aria-label="Close Dialog"
                variant="ghost"
                size="sm"
                position="absolute"
                top="2"
                right="2"
              >
                <X />
              </IconButton>
            </Dialog.CloseTrigger>
          </Dialog.Content>
        </Dialog.Positioner>
      </Dialog.Root>
    </>
  );
};
