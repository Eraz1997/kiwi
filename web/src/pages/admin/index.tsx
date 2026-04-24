import {
  ArrowRight,
  Fingerprint,
  Fish,
  LibraryBig,
  ServerCog,
} from "lucide-solid";
import { Component } from "solid-js";
import { Container, HStack, VStack } from "styled-system/jsx";
import { Button } from "~/components";
import { Card } from "~/components";
import { NavigationBar } from "~/components";
import { useRouter } from "~/contexts/router";

export const AdminIndex: Component = () => {
  const { navigate } = useRouter();

  return (
    <VStack gap="16">
      <NavigationBar />
      <Container px="12" pb="12" maxW="2xl" overflowX="scroll">
        <HStack
          gap="6"
          flexWrap="wrap"
          justifyContent="space-evenly"
          alignItems="stretch"
        >
          <Card.Root width="2xs">
            <Card.Header>
              <Card.Title>Users</Card.Title>
              <Card.Description>
                Create and manage users and access.
              </Card.Description>
            </Card.Header>
            <Card.Body>
              <Container>
                <Fish size={64} />
              </Container>
            </Card.Body>
            <Card.Footer>
              <Button onClick={() => navigate("admin/users")}>
                <ArrowRight />
              </Button>
            </Card.Footer>
          </Card.Root>
          <Card.Root width="2xs">
            <Card.Header>
              <Card.Title>Services</Card.Title>
              <Card.Description>
                Create and manage deployed services.
              </Card.Description>
            </Card.Header>
            <Card.Body>
              <Container>
                <ServerCog size={64} />
              </Container>
            </Card.Body>
            <Card.Footer>
              <Button onClick={() => navigate("admin/services")}>
                <ArrowRight />
              </Button>
            </Card.Footer>
          </Card.Root>
          <Card.Root width="2xs">
            <Card.Header>
              <Card.Title>Dynamic DNS</Card.Title>
              <Card.Description>
                Enable and manage dynamic DNS.
              </Card.Description>
            </Card.Header>
            <Card.Body>
              <Container>
                <LibraryBig size={64} />
              </Container>
            </Card.Body>
            <Card.Footer>
              <Button onClick={() => navigate("admin/dynamic-dns")}>
                <ArrowRight />
              </Button>
            </Card.Footer>
          </Card.Root>
          <Card.Root width="2xs">
            <Card.Header>
              <Card.Title>TLS</Card.Title>
              <Card.Description>Manage TLS certificates.</Card.Description>
            </Card.Header>
            <Card.Body>
              <Container>
                <Fingerprint size={64} />
              </Container>
            </Card.Body>
            <Card.Footer>
              <Button onClick={() => navigate("admin/certificates")}>
                <ArrowRight />
              </Button>
            </Card.Footer>
          </Card.Root>
        </HStack>
      </Container>
    </VStack>
  );
};
