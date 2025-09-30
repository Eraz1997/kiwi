import { DeleteServiceDialog } from "./deleteServiceDialog";
import { EditServiceDialog } from "./editServiceDialog";
import {
  CircleX,
  GitBranchPlus,
  Globe,
  HardDrive,
  Info,
  Plus,
  Satellite,
  ScanFace,
  Shrimp,
  Trash2,
  Variable,
} from "lucide-solid";
import { Component, For, Match, Show, Switch, createSignal } from "solid-js";
import { SetStoreFunction } from "solid-js/store";
import { Alert } from "src/components/ui/alert";
import { Button } from "src/components/ui/button";
import { Card } from "src/components/ui/card";
import { Field } from "src/components/ui/field";
import { Heading } from "src/components/ui/heading";
import { Switch as SwitchToggle } from "src/components/ui/switch";
import { Text } from "src/components/ui/text";
import { useRouter } from "src/contexts/router";
import { createAsyncAction } from "src/hooks/createAsyncAction";
import { createBackendClient } from "src/hooks/createBackendClient";
import { ContainerConfiguration, Role } from "src/types";
import { HStack, VStack } from "styled-system/jsx";

type Props = {
  mode: "create" | "edit";
  containerConfiguration: ContainerConfiguration;
  setContainerConfiguration: SetStoreFunction<ContainerConfiguration>;
};

export const ServiceDetailsCard: Component<Props> = (props) => {
  const isNameValid = () =>
    !!props.containerConfiguration.name.match("^[a-zA-Z0-9-_]{3,32}$");
  const isShaValid = () =>
    !!props.containerConfiguration.image_sha.value.match("^[0-9a-f]{64}$");
  const isConfigurationValid = () =>
    isNameValid() &&
    isShaValid() &&
    ![
      props.containerConfiguration.exposed_port.internal,
      props.containerConfiguration.exposed_port.external,
      props.containerConfiguration.image_name,
    ].find((field) => !field);

  const [error, setError] = createSignal<string>();
  const { navigate } = useRouter();
  const adminBackendClient = createBackendClient("admin");

  const { isLoading: isCreationLoading, call: createService } =
    createAsyncAction(async () => {
      setError();

      const result = await adminBackendClient.post(
        "/services",
        props.containerConfiguration,
      );

      if (result.statusCode === 200) {
        navigate("admin/services");
      } else {
        setError(result.text ?? "unknown error");
      }
    });

  const { isLoading: isEditLoading, call: editService } = createAsyncAction(
    async () => {
      setError();

      const result = await adminBackendClient.put(
        `/services/${props.containerConfiguration.name}`,
        props.containerConfiguration,
      );

      if (result.statusCode !== 200) {
        setError(result.text ?? "unknown error");
      }
    },
  );

  const { isLoading: isDeletionLoading, call: deleteService } =
    createAsyncAction(async () => {
      setError();

      const result = await adminBackendClient.delete(
        `/services/${props.containerConfiguration.name}`,
      );

      if (result.statusCode === 200) {
        navigate("admin/services");
      } else {
        setError(result.text ?? "unknown error");
      }
    });

  const isAnythingLoading = () =>
    isCreationLoading() || isEditLoading() || isDeletionLoading();

  return (
    <VStack gap="6">
      <Show when={error()}>
        <Alert.Root borderColor="red.default">
          <Alert.Icon
            color="red.text"
            asChild={(iconProps) => <CircleX {...iconProps()} />}
          />
          <Alert.Content>
            <Alert.Title color="red.text">Something went wrong</Alert.Title>
            <Alert.Description color="red.text">{error()}</Alert.Description>
          </Alert.Content>
        </Alert.Root>
      </Show>
      <Card.Root>
        <Card.Header>
          <Card.Title>
            <Switch>
              <Match when={props.mode === "create"}>Create Service</Match>
              <Match when={props.mode === "edit"}>Details</Match>
            </Switch>
          </Card.Title>
        </Card.Header>
        <Card.Body>
          <VStack gap="12" alignItems="start">
            <VStack gap="4" alignItems="start" width="full">
              <Heading size="md" display="flex" gap="2">
                General <Info />
              </Heading>
              <Field.Root
                invalid={!!props.containerConfiguration.name && !isNameValid()}
                disabled={props.mode === "edit"}
                width="full"
              >
                <Field.Label>Name</Field.Label>
                <Field.Input
                  onChange={(event) =>
                    props.setContainerConfiguration("name", event.target.value)
                  }
                  value={props.containerConfiguration.name}
                />
                <Field.ErrorText>Please enter a valid name</Field.ErrorText>
              </Field.Root>
            </VStack>
            <VStack gap="4" alignItems="start" width="full">
              <Heading size="md" display="flex" gap="2">
                Authorisation <ScanFace />
              </Heading>
              <SwitchToggle
                checked={!!props.containerConfiguration.required_role}
                onCheckedChange={(event) => {
                  const requiredRole = event.checked ? Role.Customer : null;
                  props.setContainerConfiguration(
                    "required_role",
                    requiredRole,
                  );
                }}
              >
                Restricted to Members
              </SwitchToggle>
            </VStack>
            <VStack gap="4" alignItems="start" width="full">
              <Heading size="md" display="flex" gap="2">
                Docker Image <Shrimp />
              </Heading>
              <HStack gap="2" width="full">
                <Field.Root width="full">
                  <Field.Label>Name</Field.Label>
                  <Field.Input
                    onChange={(event) =>
                      props.setContainerConfiguration(
                        "image_name",
                        event.target.value,
                      )
                    }
                    value={props.containerConfiguration.image_name}
                  />
                </Field.Root>
                <Field.Root
                  invalid={
                    !!props.containerConfiguration.image_sha.value &&
                    !isShaValid()
                  }
                  width="full"
                >
                  <Field.Label>Sha</Field.Label>
                  <Field.Input
                    onChange={(event) =>
                      props.setContainerConfiguration(
                        "image_sha",
                        "value",
                        event.target.value,
                      )
                    }
                    value={props.containerConfiguration.image_sha.value}
                  />
                  <Field.ErrorText>Please enter a valid Sha</Field.ErrorText>
                </Field.Root>
              </HStack>
            </VStack>
            <VStack gap="4" alignItems="start" width="full">
              <Heading size="md" display="flex" gap="2">
                Github Repository (Optional) <GitBranchPlus />
              </Heading>
              <HStack gap="2" width="full">
                <Field.Root width="full">
                  <Field.Label>Owner</Field.Label>
                  <Field.Input
                    onChange={(event) =>
                      props.setContainerConfiguration(
                        "github_repository",
                        "owner",
                        event.target.value,
                      )
                    }
                    value={
                      props.containerConfiguration.github_repository?.owner
                    }
                  />
                </Field.Root>
                <Field.Root width="full">
                  <Field.Label>Name</Field.Label>
                  <Field.Input
                    onChange={(event) =>
                      props.setContainerConfiguration(
                        "github_repository",
                        "name",
                        event.target.value,
                      )
                    }
                    value={props.containerConfiguration.github_repository?.name}
                  />
                </Field.Root>
              </HStack>
            </VStack>
            <VStack gap="4" alignItems="start" width="full">
              <Heading size="md" display="flex" gap="2">
                Networking <Globe />
              </Heading>
              <HStack gap="2" width="full">
                <Field.Root width="full">
                  <Field.Label>Exposed Port (Internal)</Field.Label>
                  <Field.Input
                    onChange={(event) =>
                      props.setContainerConfiguration(
                        "exposed_port",
                        "internal",
                        parseInt(event.target.value),
                      )
                    }
                    value={props.containerConfiguration.exposed_port.internal}
                    type="number"
                  />
                </Field.Root>
                <Field.Root width="full" disabled={props.mode === "edit"}>
                  <Field.Label>Exposed Port (External)</Field.Label>
                  <Field.Input
                    onChange={(event) =>
                      props.setContainerConfiguration(
                        "exposed_port",
                        "external",
                        parseInt(event.target.value),
                      )
                    }
                    value={props.containerConfiguration.exposed_port.external}
                    type="number"
                  />
                </Field.Root>
              </HStack>
            </VStack>
            <VStack gap="4" alignItems="start" width="full">
              <Heading size="md" display="flex" gap="2">
                Volumes <HardDrive />
              </Heading>
              <VStack gap="2" width="full">
                <For each={props.containerConfiguration.stateful_volume_paths}>
                  {(path, index) => (
                    <HStack gap="2" width="full">
                      <Field.Root width="full">
                        <Field.Label>Stateful Path</Field.Label>
                        <Field.Input
                          onChange={(event) =>
                            props.setContainerConfiguration(
                              "stateful_volume_paths",
                              index(),
                              event.target.value,
                            )
                          }
                          value={path}
                        />
                      </Field.Root>
                      <Button
                        size="md"
                        bgColor={{ base: "red.7", _hover: "red.8" }}
                        onClick={() =>
                          props.setContainerConfiguration(
                            "stateful_volume_paths",
                            (paths) =>
                              paths.filter(
                                (_, pathIndex) => pathIndex !== index(),
                              ),
                          )
                        }
                        mt="auto"
                      >
                        <Trash2 />
                      </Button>
                    </HStack>
                  )}
                </For>
                <Button
                  size="sm"
                  onClick={() =>
                    props.setContainerConfiguration(
                      "stateful_volume_paths",
                      (paths) => paths.concat([""]),
                    )
                  }
                  alignSelf="start"
                >
                  Add Stateful Path <Plus />
                </Button>
              </VStack>
            </VStack>
            <VStack gap="4" alignItems="start" width="full">
              <Heading size="md" display="flex" gap="2">
                Environment Variables <Variable />
              </Heading>
              <VStack gap="2" width="full">
                <For each={props.containerConfiguration.environment_variables}>
                  {(variable, index) => (
                    <HStack gap="2" width="full">
                      <Field.Root width="full">
                        <Field.Label>Name</Field.Label>
                        <Field.Input
                          onChange={(event) =>
                            props.setContainerConfiguration(
                              "environment_variables",
                              index(),
                              "name",
                              event.target.value,
                            )
                          }
                          value={variable.name}
                        />
                      </Field.Root>
                      <Field.Root width="full">
                        <Field.Label>Value</Field.Label>
                        <Field.Input
                          onChange={(event) =>
                            props.setContainerConfiguration(
                              "environment_variables",
                              index(),
                              "value",
                              event.target.value,
                            )
                          }
                          value={variable.value}
                        />
                      </Field.Root>
                      <Button
                        size="md"
                        mt="auto"
                        bgColor={{ base: "red.7", _hover: "red.8" }}
                        onClick={() =>
                          props.setContainerConfiguration(
                            "environment_variables",
                            (variables) =>
                              variables.filter(
                                (_, variableIndex) => variableIndex !== index(),
                              ),
                          )
                        }
                      >
                        <Trash2 />
                      </Button>
                    </HStack>
                  )}
                </For>
                <Button
                  size="sm"
                  onClick={() =>
                    props.setContainerConfiguration(
                      "environment_variables",
                      (variables) =>
                        variables.concat([{ name: "", value: "" }]),
                    )
                  }
                  alignSelf="start"
                >
                  Add Environment Variable <Plus />
                </Button>
              </VStack>
            </VStack>
            <VStack gap="4" alignItems="start" width="full">
              <Heading size="md" display="flex" gap="2">
                Secrets <Variable />
              </Heading>
              <Text size="md">
                They are still passed as environment variables to the
                application
              </Text>
              <VStack gap="2" width="full">
                <For each={props.containerConfiguration.secrets}>
                  {(secret, index) => (
                    <HStack gap="2" width="full">
                      <Field.Root width="full">
                        <Field.Label>Name</Field.Label>
                        <Field.Input
                          onChange={(event) =>
                            props.setContainerConfiguration(
                              "secrets",
                              index(),
                              "name",
                              event.target.value,
                            )
                          }
                          value={secret.name}
                        />
                      </Field.Root>
                      <Field.Root width="full">
                        <Field.Label>Value</Field.Label>
                        <Field.Input
                          onChange={(event) =>
                            props.setContainerConfiguration(
                              "secrets",
                              index(),
                              "value",
                              event.target.value,
                            )
                          }
                          value={secret.value}
                          type="password"
                          autocomplete="off"
                        />
                      </Field.Root>
                      <Button
                        size="md"
                        mt="auto"
                        bgColor={{ base: "red.7", _hover: "red.8" }}
                        onClick={() =>
                          props.setContainerConfiguration(
                            "secrets",
                            (secrets) =>
                              secrets.filter(
                                (_, secretIndex) => secretIndex !== index(),
                              ),
                          )
                        }
                      >
                        <Trash2 />
                      </Button>
                    </HStack>
                  )}
                </For>
                <Button
                  size="sm"
                  onClick={() =>
                    props.setContainerConfiguration("secrets", (secrets) =>
                      secrets.concat([{ name: "", value: "" }]),
                    )
                  }
                  alignSelf="start"
                >
                  Add Secret <Plus />
                </Button>
              </VStack>
            </VStack>
          </VStack>
        </Card.Body>
        <Card.Footer>
          <Switch>
            <Match when={props.mode === "edit"}>
              <HStack gap="4">
                <DeleteServiceDialog
                  loading={isAnythingLoading()}
                  onConfirm={deleteService}
                />
                <EditServiceDialog
                  loading={isAnythingLoading()}
                  onConfirm={editService}
                />
              </HStack>
            </Match>
            <Match when={props.mode === "create"}>
              <Button
                loading={isAnythingLoading()}
                disabled={!isConfigurationValid()}
                onClick={createService}
              >
                Create Service <Satellite />
              </Button>
            </Match>
          </Switch>
        </Card.Footer>
      </Card.Root>
    </VStack>
  );
};
