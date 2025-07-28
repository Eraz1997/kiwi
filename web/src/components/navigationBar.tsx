import { Avatar } from "./ui/avatar";
import { Button } from "./ui/button";
import { Menu } from "./ui/menu";
import { ChevronLeft, LogOutIcon } from "lucide-solid";
import { Component, createResource } from "solid-js";
import { Heading } from "src/components/ui/heading";
import { Page, useRouter } from "src/contexts/router";
import { createBackendClient } from "src/hooks/createBackendClient";
import { HStack, Spacer } from "styled-system/jsx";

export const NavigationBar: Component = () => {
  const { currentPage, navigate } = useRouter();
  const adminClient = createBackendClient("admin");

  const [user] = createResource<User>(async () => {
    const { jsonPayload: user } = await adminClient.get("/me");
    return user;
  });

  const title = () => {
    if (currentPage() === "admin") {
      return "Home";
    }
    if (currentPage() === "admin/users") {
      return "Users";
    }
    if (currentPage() === "admin/services") {
      return "Services";
    }
    return "";
  };
  const backPage = (): Page | null => {
    if (currentPage() === "admin/users") {
      return "admin";
    }
    if (currentPage() === "admin/services") {
      return "admin";
    }
    return null;
  };

  return (
    <>
      <HStack
        gap="16"
        borderColor="border.subtle"
        borderBottomWidth="1"
        borderStyle="solid"
        position="fixed"
        left="0"
        top="0"
        bg="bg.canvas"
        zIndex="sticky"
        height="16"
        px="12"
        w="full"
      >
        <HStack gap="6">
          <Button
            disabled={!backPage()}
            variant="outline"
            size="xs"
            marginTop="1"
            onClick={() => {
              const targetPage = backPage();
              if (targetPage) {
                navigate(targetPage);
              }
            }}
          >
            <ChevronLeft />
          </Button>
          <Heading size="4xl">Kiwi 🥝</Heading>
        </HStack>
        <Spacer />
        <Heading size="xl">{title()}</Heading>
        <Menu.Root>
          <Menu.Trigger
            asChild={(triggerProps) => (
              <Avatar
                {...triggerProps}
                cursor="pointer"
                name={user()?.username ?? ""}
              />
            )}
          />
          <Menu.Positioner>
            <Menu.Content>
              <Menu.ItemGroup>
                <Menu.ItemGroupLabel>
                  {user()?.username ?? "No Account"}
                </Menu.ItemGroupLabel>
                <Menu.Separator />
                <Menu.Item
                  value="logout"
                  onClick={() => navigate("auth/logout")}
                >
                  <HStack gap="2">
                    <LogOutIcon />
                    Logout
                  </HStack>
                </Menu.Item>
              </Menu.ItemGroup>
            </Menu.Content>
          </Menu.Positioner>
        </Menu.Root>
      </HStack>
      <HStack height="16" />
    </>
  );
};
