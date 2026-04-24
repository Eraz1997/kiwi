import { Avatar } from ".";
import { Menu } from ".";
import { Button } from "./button";
import { Heading } from "./heading";
import { ChevronLeft, LogOutIcon } from "lucide-solid";
import { Component, createResource } from "solid-js";
import { css } from "styled-system/css";
import { HStack } from "styled-system/jsx";
import { Page, useRouter } from "~/contexts/router";
import { createBackendClient } from "~/hooks/createBackendClient";
import { User } from "~/types";

export const NavigationBar: Component = () => {
  const { currentPage, navigate } = useRouter();
  const adminClient = createBackendClient("admin");

  const [user] = createResource<User>(async () => {
    const { jsonPayload: user } = await adminClient.get("/users/me");
    return user;
  });
  const userInitials = () => {
    const username = user()?.username;
    if (!username) return ":)";
    return username[0].toUpperCase();
  };

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
        borderColor="border.subtle"
        borderBottomWidth="1"
        borderStyle="solid"
        position="fixed"
        left="0"
        top="0"
        bg="bg.canvas"
        zIndex="sticky"
        height="16"
        px={{ base: "12", mdDown: "4" }}
        w="100vw"
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
          <Heading textWrap="nowrap" textStyle="4xl">
            <span class={css({ display: { smDown: "none" } })}>Kiwi</span> 🥝
          </Heading>
        </HStack>
        <Heading
          textStyle="xl"
          flex="1"
          textAlign="end"
          mr={{ base: "16", smDown: "8" }}
        >
          {title()}
        </Heading>
        <Menu.Root>
          <Menu.Trigger>
            <Avatar.Root cursor="pointer">
              <Avatar.Fallback>{userInitials()}</Avatar.Fallback>
            </Avatar.Root>
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
