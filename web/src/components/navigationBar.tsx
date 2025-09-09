import { Avatar } from "./ui/avatar";
import { Button } from "./ui/button";
import { Menu } from "./ui/menu";
import { ChevronLeft, LogOutIcon } from "lucide-solid";
import { Component, createResource } from "solid-js";
import { Heading } from "src/components/ui/heading";
import { Page, useRouter } from "src/contexts/router";
import { createBackendClient } from "src/hooks/createBackendClient";
import { User } from "src/types";
import { HStack } from "styled-system/jsx";

export const NavigationBar: Component = () => {
  const { currentPage, navigate } = useRouter();
  const adminClient = createBackendClient("admin");

  const [user] = createResource<User>(async () => {
    const { jsonPayload: user } = await adminClient.get("/users/me");
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
    if (currentPage() === "admin/services/new") {
      return "Create Service";
    }
    if (currentPage() === "admin/services/edit") {
      return "Service Details";
    }
    if (currentPage() === "admin/dynamic-dns") {
      return "Dynamic DNS";
    }
    if (currentPage() === "admin/certificates") {
      return "TLS";
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
    if (currentPage() === "admin/services/new") {
      return "admin/services";
    }
    if (currentPage() === "admin/services/edit") {
      return "admin/services";
    }
    if (currentPage() === "admin/dynamic-dns") {
      return "admin";
    }
    if (currentPage() === "admin/certificates") {
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
          <Heading textWrap="nowrap" size={{ base: "4xl", smDown: "2xl" }}>
            Kiwi 🥝
          </Heading>
        </HStack>
        <Heading size="xl" flex="1" textAlign="end">
          {title()}
        </Heading>
        <Menu.Root>
          <Menu.Trigger>
            <Avatar cursor="pointer" name={user()?.username ?? ""} />
          </Menu.Trigger>
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
