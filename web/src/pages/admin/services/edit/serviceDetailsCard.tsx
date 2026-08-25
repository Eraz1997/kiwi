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
import {
	type Component,
	createSignal,
	For,
	Match,
	Show,
	Switch,
} from "solid-js";
import type { SetStoreFunction } from "solid-js/store";
import { HStack, VStack } from "styled-system/jsx";
import {
	Alert,
	Button,
	Card,
	Checkbox,
	Field,
	Heading,
	Input,
	RadioGroup,
	Text,
} from "~/components";
import { useRouter } from "~/contexts/router";
import { createAsyncAction } from "~/hooks/createAsyncAction";
import { createBackendClient } from "~/hooks/createBackendClient";
import { type ContainerConfiguration, Role } from "~/types";
import { DeleteServiceDialog } from "./deleteServiceDialog";
import { EditServiceDialog } from "./editServiceDialog";

type Props = {
	mode: "create" | "edit";
	containerConfiguration: ContainerConfiguration;
	setContainerConfiguration: SetStoreFunction<ContainerConfiguration>;
};

export const ServiceDetailsCard: Component<Props> = (props) => {
	const isNameValid = () =>
		!!props.containerConfiguration.name.match("^[a-zA-Z0-9-_]{3,32}$");
	const isImageNameValid = () => props.containerConfiguration.imageName !== "";
	const isShaValid = () =>
		!!props.containerConfiguration.imageSha.value.match("^[0-9a-f]{64}$");
	const isConfigurationValid = () =>
		isNameValid() &&
		isImageNameValid() &&
		isShaValid() &&
		![
			props.containerConfiguration.exposedPort.internal,
			props.containerConfiguration.exposedPort.external,
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
					<Alert.Indicator>
						<CircleX />
					</Alert.Indicator>
					<Alert.Content>
						<Alert.Title>Something went wrong</Alert.Title>
						<Alert.Description>{error()}</Alert.Description>
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
							<Heading textStyle="md" display="flex" gap="2">
								General <Info />
							</Heading>
							<Field.Root
								invalid={!!props.containerConfiguration.name && !isNameValid()}
								disabled={props.mode === "edit"}
								width="full"
							>
								<Field.Label>Name</Field.Label>
								<Input
									onChange={(event) =>
										props.setContainerConfiguration("name", event.target.value)
									}
									value={props.containerConfiguration.name}
								/>
								<Field.ErrorText>Please enter a valid name</Field.ErrorText>
							</Field.Root>
						</VStack>
						<VStack gap="4" alignItems="start" width="full">
							<Heading textStyle="md" display="flex" gap="2">
								Authorisation <ScanFace />
							</Heading>
							<RadioGroup.Root
								defaultValue={props.containerConfiguration.requiredRole}
								onValueChange={(event) => {
									const value = event.value as Role;
									const requiredRole = [Role.Admin, Role.Customer].includes(
										value,
									)
										? value
										: null;
									props.setContainerConfiguration("requiredRole", requiredRole);
								}}
							>
								<For each={[null, Role.Customer, Role.Admin]}>
									{(role) => (
										<RadioGroup.Item value={role ?? "None"}>
											<RadioGroup.ItemHiddenInput />
											<RadioGroup.ItemControl />
											<RadioGroup.ItemText>
												{role ?? "None"}
											</RadioGroup.ItemText>
										</RadioGroup.Item>
									)}
								</For>
							</RadioGroup.Root>
						</VStack>
						<VStack gap="4" alignItems="start" width="full">
							<Heading textStyle="md" display="flex" gap="2">
								Docker Image <Shrimp />
							</Heading>
							<HStack gap="2" width="full">
								<Field.Root width="full">
									<Field.Label>Name</Field.Label>
									<Input
										onChange={(event) =>
											props.setContainerConfiguration(
												"imageName",
												event.target.value,
											)
										}
										value={props.containerConfiguration.imageName ?? ""}
										disabled={props.containerConfiguration.imageName === null}
									/>
								</Field.Root>
								<Field.Root
									invalid={
										!!props.containerConfiguration.imageSha.value &&
										!isShaValid()
									}
									width="full"
								>
									<Field.Label>Sha</Field.Label>
									<Input
										onChange={(event) =>
											props.setContainerConfiguration(
												"imageSha",
												"value",
												event.target.value,
											)
										}
										value={props.containerConfiguration.imageSha.value}
									/>
									<Field.ErrorText>Please enter a valid Sha</Field.ErrorText>
								</Field.Root>
							</HStack>
							<Checkbox.Root
								checked={props.containerConfiguration.imageName === null}
								onCheckedChange={(event) =>
									props.setContainerConfiguration(
										"imageName",
										event.checked === true ? null : "",
									)
								}
							>
								<Checkbox.HiddenInput />
								<Checkbox.Control>
									<Checkbox.Indicator />
								</Checkbox.Control>
								<Checkbox.Label>
									I don't have a registry, I will directly push image tarballs
								</Checkbox.Label>
							</Checkbox.Root>
						</VStack>
						<VStack gap="4" alignItems="start" width="full">
							<Heading textStyle="md" display="flex" gap="2">
								Github Repository (Optional) <GitBranchPlus />
							</Heading>
							<HStack gap="2" width="full">
								<Field.Root width="full">
									<Field.Label>Owner</Field.Label>
									<Input
										onChange={(event) =>
											props.setContainerConfiguration(
												"githubRepository",
												"owner",
												event.target.value,
											)
										}
										value={props.containerConfiguration.githubRepository?.owner}
									/>
								</Field.Root>
								<Field.Root width="full">
									<Field.Label>Name</Field.Label>
									<Input
										onChange={(event) =>
											props.setContainerConfiguration(
												"githubRepository",
												"name",
												event.target.value,
											)
										}
										value={props.containerConfiguration.githubRepository?.name}
									/>
								</Field.Root>
							</HStack>
						</VStack>
						<VStack gap="4" alignItems="start" width="full">
							<Heading textStyle="md" display="flex" gap="2">
								Networking <Globe />
							</Heading>
							<HStack gap="2" width="full">
								<Field.Root width="full">
									<Field.Label>Exposed Port (Internal)</Field.Label>
									<Input
										onChange={(event) =>
											props.setContainerConfiguration(
												"exposedPort",
												"internal",
												parseInt(event.target.value, 10),
											)
										}
										value={props.containerConfiguration.exposedPort.internal}
										type="number"
									/>
								</Field.Root>
								<Field.Root width="full" disabled={props.mode === "edit"}>
									<Field.Label>Exposed Port (External)</Field.Label>
									<Input
										onChange={(event) =>
											props.setContainerConfiguration(
												"exposedPort",
												"external",
												parseInt(event.target.value, 10),
											)
										}
										value={props.containerConfiguration.exposedPort.external}
										type="number"
									/>
								</Field.Root>
							</HStack>
						</VStack>
						<VStack gap="4" alignItems="start" width="full">
							<Heading textStyle="md" display="flex" gap="2">
								Volumes <HardDrive />
							</Heading>
							<VStack gap="2" width="full">
								<For each={props.containerConfiguration.statefulVolumePaths}>
									{(path, index) => (
										<HStack gap="2" width="full">
											<Field.Root width="full">
												<Field.Label>Stateful Path</Field.Label>
												<Input
													onChange={(event) =>
														props.setContainerConfiguration(
															"statefulVolumePaths",
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
														"statefulVolumePaths",
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
											"statefulVolumePaths",
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
							<Heading textStyle="md" display="flex" gap="2">
								Environment Variables <Variable />
							</Heading>
							<VStack gap="2" width="full">
								<For each={props.containerConfiguration.environmentVariables}>
									{(variable, index) => (
										<HStack gap="2" width="full">
											<Field.Root width="full">
												<Field.Label>Name</Field.Label>
												<Input
													onChange={(event) =>
														props.setContainerConfiguration(
															"environmentVariables",
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
												<Input
													onChange={(event) =>
														props.setContainerConfiguration(
															"environmentVariables",
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
														"environmentVariables",
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
											"environmentVariables",
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
							<Heading textStyle="md" display="flex" gap="2">
								Secrets <Variable />
							</Heading>
							<Text textStyle="md">
								They are still passed as environment variables to the
								application
							</Text>
							<VStack gap="2" width="full">
								<For each={props.containerConfiguration.secrets}>
									{(secret, index) => (
										<HStack gap="2" width="full">
											<Field.Root width="full">
												<Field.Label>Name</Field.Label>
												<Input
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
												<Input
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
