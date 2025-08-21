import { Bomb, Trash2, X } from "lucide-solid";
import { Component } from "solid-js";
import { Button } from "src/components/ui/button";
import { Dialog } from "src/components/ui/dialog";
import { IconButton } from "src/components/ui/icon-button";
import { HStack, Spacer, VStack } from "styled-system/jsx";

type Props = {
  loading: boolean;
  onConfirm: () => void;
};

export const DeleteServiceDialog: Component<Props> = (props) => {
  return (
    <>
      <Dialog.Root>
        <Dialog.Trigger>
          <Button
            bgColor={{ base: "red.7", _hover: "red.8" }}
            loading={props.loading}
          >
            Delete Service
            <Bomb />
          </Button>
        </Dialog.Trigger>
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content textAlign="start">
            <VStack gap="8" p="6" maxW="md">
              <Dialog.Title>Delete Service</Dialog.Title>
              <Dialog.Description>
                This action cannot be reverted. Please make sure this is an
                intended action. All service data will be lost and you will not
                be able to recover it. Do you really want to proceed?
              </Dialog.Description>
              <HStack gap="3" width="full">
                <Spacer />
                <Dialog.CloseTrigger>
                  <Button variant="outline">Cancel</Button>
                </Dialog.CloseTrigger>
                <Dialog.CloseTrigger>
                  <Button
                    bgColor={{ base: "red.7", _hover: "red.8" }}
                    loading={props.loading}
                    onClick={props.onConfirm}
                  >
                    Confirm
                    <Trash2 />
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
