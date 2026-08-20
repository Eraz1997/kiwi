import type { Component } from "solid-js";
import { createStore } from "solid-js/store";
import { Container } from "styled-system/jsx";
import { NavigationBar } from "~/components";
import type { ContainerConfiguration } from "~/types";
import { ServiceDetailsCard } from "./components/serviceDetailsCard";

export const AdminServicesNew: Component = () => {
	const [configuration, setConfiguration] = createStore<ContainerConfiguration>(
		{
			name: "",
			imageName: "",
			imageSha: { value: "" },
			exposedPort: {
				internal: 3000,
				external: Math.floor(Math.random() * 5000) + 3000,
			},
			environmentVariables: [],
			secrets: [],
			internalSecrets: [],
			statefulVolumePaths: [],
			githubRepository: null,
			requiredRole: null,
		},
	);

	return (
		<>
			<NavigationBar />
			<Container p="12" maxW="4xl" overflowX="scroll">
				<ServiceDetailsCard
					containerConfiguration={configuration}
					setContainerConfiguration={setConfiguration}
					mode="create"
				/>
			</Container>
		</>
	);
};
