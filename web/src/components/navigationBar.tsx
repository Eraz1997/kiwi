import { Button } from "./ui/button";
import { ChevronLeft } from "lucide-solid";
import { Component } from "solid-js";
import { Heading } from "src/components/ui/heading";
import { Page, useRouter } from "src/contexts/router";
import { HStack, Spacer } from "styled-system/jsx";

export const NavigationBar: Component = () => {
  const { currentPage, navigate } = useRouter();
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
      </HStack>
      <HStack height="16" />
    </>
  );
};
