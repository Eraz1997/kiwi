import { ArrowRight, Fish, ServerCog } from "lucide-solid";
import { Component } from "solid-js";
import { NavigationBar } from "src/components/navigationBar";
import { Button } from "src/components/ui/button";
import { Card } from "src/components/ui/card";
import { useRouter } from "src/contexts/router";
import { Container, HStack, VStack } from "styled-system/jsx";

export const AdminIndex: Component = () => {
  const { navigate } = useRouter();

  return (
    <VStack gap="16">
      <NavigationBar />
      <Container p={{ base: "12" }} maxW="lg">
        <HStack gap="6">
          <Card.Root>
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
          <Card.Root>
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
        </HStack>
      </Container>
    </VStack>
  );
};
