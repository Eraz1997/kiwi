import { type Component, createEffect, Show } from "solid-js";
import { createStore, type SetStoreFunction } from "solid-js/store";
import { Container, HStack, VStack } from "styled-system/jsx";
import { Card, Spinner, Text } from "~/components";
import { useKangaroo } from "~/contexts/kangaroo";
import { useRouter } from "~/contexts/router";
import { createBackendClient } from "~/hooks/createBackendClient";
import type { ContainerConfiguration } from "~/types";
import { NavigationBar } from "../../navigationBar";
import { LogsViewer } from "./logsViewer";
import { ServiceDetailsCard } from "./serviceDetailsCard";

type ContainerInfo = {
	configuration: ContainerConfiguration | null;
	status: string | null;
};

type ContainerApiInfo = {
	generalInfo: {
		containerConfiguration: ContainerConfiguration;
	};
	status: string;
};

export const AdminServicesEdit: Component = () => {
	const { queryParams } = useRouter();
	const { consumeAdminEditServiceData } = useKangaroo();
	const adminEditServiceData = consumeAdminEditServiceData();
	const adminClient = createBackendClient("admin");

	const [containerInfo, setContainerInfo] = createStore<ContainerInfo>({
		configuration: null,
		status: null,
	});

	const getServiceFromApi = async (): Promise<ContainerApiInfo> => {
		const { jsonPayload: service } = await adminClient.get(
			`/services/${queryParams().name}`,
		);
		return service;
	};

	createEffect(async () => {
		const service = adminEditServiceData ?? (await getServiceFromApi());
		setContainerInfo(
			"configuration",
			service.generalInfo.containerConfiguration,
		);
		setContainerInfo("status", service.status);
	});

	return (
		<>
			<NavigationBar me={adminEditServiceData?.me} />
			<Container p="12" maxW="4xl" overflowX="scroll">
				<HStack gap="10" alignItems="start">
					<Container w="sm">
						<Show
							when={containerInfo.configuration}
							fallback={
								<VStack gap="6">
									<Spinner size="xl" />
									<Text textStyle="lg">Loading service details...</Text>
								</VStack>
							}
						>
							{(configuration) => (
								<ServiceDetailsCard
									containerConfiguration={configuration()}
									setContainerConfiguration={
										setContainerInfo.bind(
											null,
											"configuration",
										) as SetStoreFunction<ContainerConfiguration>
									}
									mode="edit"
								/>
							)}
						</Show>
					</Container>
					<VStack w="sm" gap="12">
						<Card.Root width="full">
							<Card.Header>
								<Card.Title>Status</Card.Title>
							</Card.Header>
							<Card.Body>
								<Show when={containerInfo.status} fallback={<Spinner />}>
									{(status) => capitalise(status())}
								</Show>
							</Card.Body>
						</Card.Root>
						<LogsViewer
							serviceName={queryParams().name}
							logs={adminEditServiceData?.logs}
						/>
					</VStack>
				</HStack>
			</Container>
		</>
	);
};

const capitalise = (text: string): string => {
	if (!text) return text;
	return `${text[0].toUpperCase()}${text.substring(1)}`;
};
