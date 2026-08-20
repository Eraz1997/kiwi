import { ArrowRight, Plus } from "lucide-solid";
import { type Component, createResource, For, Show } from "solid-js";
import { Container } from "styled-system/jsx";
import { Button, NavigationBar, Table } from "~/components";
import { useRouter } from "~/contexts/router";
import { createBackendClient } from "~/hooks/createBackendClient";
import type { Service } from "~/types";

export const AdminServices: Component = () => {
	const { navigate } = useRouter();
	const adminClient = createBackendClient("admin");

	const [services] = createResource<Service[]>(async () => {
		const { jsonPayload } = await adminClient.get("/services");
		const services: Service[] = jsonPayload.services;
		services.forEach((service) => {
			service.createdAt = new Date(service.createdAt);
			service.lastModifiedAt = new Date(service.lastModifiedAt);
			service.lastDeployedAt = new Date(service.lastDeployedAt);
		});

		return services;
	});

	return (
		<>
			<NavigationBar />
			<Container p="12" maxW="4xl" overflowX="scroll">
				<Table.Root>
					<Table.Head>
						<Table.Row>
							<Table.Header>Name</Table.Header>
							<Table.Header>Repository</Table.Header>
							<Table.Header>Created</Table.Header>
							<Table.Header>Last Modified</Table.Header>
							<Table.Header>Last Deployed</Table.Header>
							<Table.Header />
						</Table.Row>
					</Table.Head>
					<Table.Body>
						<For each={services()}>
							{(service) => (
								<Table.Row>
									<Table.Cell fontWeight="medium">
										{service.containerConfiguration.name}
									</Table.Cell>
									<Table.Cell>
										<Show
											when={service.containerConfiguration.githubRepository}
											fallback="-"
										>
											{(repo) => `${repo().owner}/${repo().name}`}
										</Show>
									</Table.Cell>
									<Table.Cell>{formatDate(service.createdAt)}</Table.Cell>
									<Table.Cell>{formatDate(service.lastModifiedAt)}</Table.Cell>
									<Table.Cell>{formatDate(service.lastDeployedAt)}</Table.Cell>
									<Table.Cell width="24" textAlign="end">
										<Button
											size="xs"
											onClick={() =>
												navigate("admin/services/edit", {
													name: service.containerConfiguration.name,
												})
											}
										>
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
							<Table.Cell />
							<Table.Cell />
							<Table.Cell />
							<Table.Cell textAlign="end">
								<Button
									size="xs"
									onClick={() => navigate("admin/services/new")}
								>
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

const formatDate = (date: Date): string => {
	return `${date.getDate()}/${date.getMonth()}/${date.getFullYear()}`;
};
