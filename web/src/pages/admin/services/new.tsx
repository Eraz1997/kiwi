import { ServiceDetailsCard } from "./components/serviceDetailsCard";
import { Component } from "solid-js";
import { createStore } from "solid-js/store";
import { NavigationBar } from "src/components/navigationBar";
import { ContainerConfiguration } from "src/types";
import { Container } from "styled-system/jsx";

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
    },
  );

  return (
    <>
      <NavigationBar />
      <Container p={{ base: "12" }} maxW="4xl">
        <ServiceDetailsCard
          containerConfiguration={configuration}
          setContainerConfiguration={setConfiguration}
          mode="create"
        />
      </Container>
    </>
  );
};
