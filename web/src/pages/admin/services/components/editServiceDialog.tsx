import { Orbit, ShieldAlert, X } from "lucide-solid";
import { Component } from "solid-js";
import { Button } from "src/components/ui/button";
import { Dialog } from "src/components/ui/dialog";
import { IconButton } from "src/components/ui/icon-button";
import { HStack, Spacer, VStack } from "styled-system/jsx";

type Props = {
  loading: boolean;
  onConfirm: () => void;
};

export const EditServiceDialog: Component<Props> = (props) => {
  return (
    <>
      <Dialog.Root>
        <Dialog.Trigger>
          <Button
            bgColor={{ base: "amber.light.9", _hover: "amber.light.11" }}
            loading={props.loading}
          >
            Save Any Changes
            <Orbit />
          </Button>
        </Dialog.Trigger>
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content textAlign="start">
            <VStack gap="8" p="6" maxW="md">
              <Dialog.Title>Edit Service</Dialog.Title>
              <Dialog.Description>
                Some changes might cause irreversible data loss. In particular,
                removing stateful volumes causes data to be permanently lost. Do
                you really want to proceed?
              </Dialog.Description>
              <HStack gap="3" width="full">
                <Spacer />
                <Dialog.CloseTrigger>
                  <Button variant="outline">Cancel</Button>
                </Dialog.CloseTrigger>
                <Dialog.CloseTrigger>
                  <Button
                    bgColor={{
                      base: "amber.light.9",
                      _hover: "amber.light.11",
                    }}
                    loading={props.loading}
                    onClick={props.onConfirm}
                  >
                    Confirm
                    <ShieldAlert />
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
