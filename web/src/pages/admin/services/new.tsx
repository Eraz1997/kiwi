import { ServiceDetailsCard } from "./components/serviceDetailsCard";
import { Component } from "solid-js";
import { createStore } from "solid-js/store";
import { Container } from "styled-system/jsx";
import { NavigationBar } from "~/components";
import { ContainerConfiguration } from "~/types";

export const AdminServicesNew: Component = () => {
  const [configuration, setConfiguration] = createStore<ContainerConfiguration>(
    {
      name: "",
      image_name: "",
      image_sha: { value: "" },
      exposed_port: {
        internal: 3000,
        external: Math.floor(Math.random() * 5000) + 3000,
      },
      environment_variables: [],
      secrets: [],
      internal_secrets: [],
      stateful_volume_paths: [],
      github_repository: null,
      required_role: null,
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
