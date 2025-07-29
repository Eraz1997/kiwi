import { createListCollection } from "@ark-ui/solid";
import {
  CheckIcon,
  ChevronsUpDown,
  ClipboardCopyIcon,
  Lightbulb,
  Plus,
  Rocket,
  X,
} from "lucide-solid";
import { Component, For, createSignal } from "solid-js";
import { Button } from "src/components/ui/button";
import { Clipboard } from "src/components/ui/clipboard";
import { Dialog } from "src/components/ui/dialog";
import { IconButton } from "src/components/ui/icon-button";
import { Input } from "src/components/ui/input";
import { Select } from "src/components/ui/select";
import { useRouter } from "src/contexts/router";
import { createAsyncAction } from "src/hooks/createAsyncAction";
import { createBackendClient } from "src/hooks/createBackendClient";
import { Role } from "src/types";
import { HStack, Spacer, VStack } from "styled-system/jsx";

type Props = {
  createToast: (toast: {
    title: string;
    description: string;
    type: string;
  }) => void;
};

export const InviteUserDialog: Component<Props> = (props) => {
  const adminClient = createBackendClient("admin");

  const { domain, isLocalhost } = useRouter();

  const rolesCollection = createListCollection<Role>({
    items: [Role.Admin, Role.Customer],
  });
  const [selectedRole, setSelectedRole] = createSignal<Role>();
  const [invitationId, setInvitationId] = createSignal<string>();
  const invitationUrl = () => {
    const scheme = isLocalhost() ? "http://" : "https://";
    return `${scheme}auth.${domain()}/create-user?invitation_id=${invitationId()}`;
  };

  const resetState = () => {
    setSelectedRole();
    setInvitationId();
  };

  const { call: inviteUser, isLoading } = createAsyncAction(
    async (role: Role) => {
      const { statusCode, jsonPayload } = await adminClient.post("/user", {
        role,
      });

      if (statusCode === 200) {
        setInvitationId(jsonPayload.invitation_id);
      } else {
        props.createToast({
          title: "Failed",
          description: `We couldn't create a user invitation: ${jsonPayload}.`,
          type: "error",
        });
      }
    },
  );

  return (
    <>
      <Dialog.Root onOpenChange={resetState}>
        <Dialog.Trigger>
          <Button size="xs">
            <Plus />
          </Button>
        </Dialog.Trigger>
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content textAlign="start">
            <VStack gap="8" p="6" maxW="md">
              <Dialog.Title>Invite User</Dialog.Title>
              <Dialog.Description>
                Please select a role to assign to the new user. Once confirmed,
                you will be given a URL to share with the user to let them set
                up their credentials.
              </Dialog.Description>
              <Select.Root
                positioning={{ sameWidth: true }}
                collection={rolesCollection}
                value={[selectedRole() ?? ""]}
                onValueChange={(event) => setSelectedRole(event.items[0])}
              >
                <Select.Control w="sm">
                  <Select.Trigger>
                    <Select.ValueText placeholder="Select a Role" />
                    <ChevronsUpDown />
                  </Select.Trigger>
                </Select.Control>
                <Select.Positioner>
                  <Select.Content>
                    <Select.ItemGroup>
                      <Select.ItemGroupLabel>Role</Select.ItemGroupLabel>
                      <For each={rolesCollection.items}>
                        {(item) => (
                          <Select.Item item={item}>
                            <Select.ItemText>{item}</Select.ItemText>
                            <Select.ItemIndicator>
                              <CheckIcon />
                            </Select.ItemIndicator>
                          </Select.Item>
                        )}
                      </For>
                    </Select.ItemGroup>
                  </Select.Content>
                </Select.Positioner>
              </Select.Root>
              <HStack gap="3" width="full">
                <Spacer />
                <Dialog.CloseTrigger>
                  <Button variant="outline">Cancel</Button>
                </Dialog.CloseTrigger>
                <Dialog.CloseTrigger>
                  <Button
                    disabled={!selectedRole()}
                    loading={isLoading()}
                    onClick={() => {
                      const role = selectedRole();
                      if (!role) return;
                      inviteUser(role);
                    }}
                  >
                    Create
                    <Lightbulb />
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
      <Dialog.Root
        open={!!invitationId()}
        onOpenChange={(event) => {
          if (!event.open) {
            resetState();
          }
        }}
      >
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content textAlign="start">
            <VStack gap="8" p="6" maxW="md">
              <Dialog.Title>Invitation Created!</Dialog.Title>
              <Dialog.Description>
                Your invitation has been created, share the following link with
                the user.
              </Dialog.Description>
              <Clipboard.Root value={invitationUrl()} w="full">
                <Clipboard.Control>
                  <Clipboard.Input
                    asChild={(inputProps) => <Input {...inputProps()} />}
                  />
                  <Clipboard.Trigger
                    asChild={(triggerProps) => (
                      <IconButton variant="outline" {...triggerProps()}>
                        <Clipboard.Indicator copied={<CheckIcon />}>
                          <ClipboardCopyIcon />
                        </Clipboard.Indicator>
                      </IconButton>
                    )}
                  />
                </Clipboard.Control>
              </Clipboard.Root>
              <HStack gap="3" width="full">
                <Spacer />
                <Dialog.CloseTrigger>
                  <Button>
                    Close <Rocket />
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
