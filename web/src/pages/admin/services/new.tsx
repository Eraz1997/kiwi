import type { Component } from "solid-js";
import { createStore } from "solid-js/store";
import { Container } from "styled-system/jsx";
import { useKangaroo } from "~/contexts/kangaroo";
import type { ContainerConfiguration } from "~/types";
import { NavigationBar } from "../navigationBar";
import { ServiceDetailsCard } from "./edit/serviceDetailsCard";

export const AdminServicesNew: Component = () => {
	const { consumeGenericAdminData } = useKangaroo();
	const adminData = consumeGenericAdminData();
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
			<NavigationBar me={adminData?.me} />
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
