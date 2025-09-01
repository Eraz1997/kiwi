import { useRouter } from "./contexts/router";
import { DynamicDns } from "./pages/admin/dynamicDns";
import { AdminIndex } from "./pages/admin/index";
import { AdminServices } from "./pages/admin/services";
import { AdminServicesEdit } from "./pages/admin/services/edit";
import { AdminServicesNew } from "./pages/admin/services/new";
import { AdminUsers } from "./pages/admin/users";
import { CreateUser } from "./pages/auth/createUser";
import { Login } from "./pages/auth/login";
import { Logout } from "./pages/auth/logout";
import { NotFound } from "./pages/internal/notFound";
import { Component, Match, Switch } from "solid-js";
import { Box } from "styled-system/jsx/box";

export const App: Component = () => {
  const { currentPage } = useRouter();

  return (
    <Box class="light">
      <Switch>
        <Match when={currentPage() === "auth/create-user"}>
          <CreateUser />
        </Match>
        <Match when={currentPage() === "auth/login"}>
          <Login />
        </Match>
        <Match when={currentPage() === "auth/logout"}>
          <Logout />
        </Match>
        <Match when={currentPage() === "admin"}>
          <AdminIndex />
        </Match>
        <Match when={currentPage() === "admin/users"}>
          <AdminUsers />
        </Match>
        <Match when={currentPage() === "admin/services"}>
          <AdminServices />
        </Match>
        <Match when={currentPage() === "admin/services/new"}>
          <AdminServicesNew />
        </Match>
        <Match when={currentPage() === "admin/services/edit"}>
          <AdminServicesEdit />
        </Match>
        <Match when={currentPage() === "admin/dynamic-dns"}>
          <DynamicDns />
        </Match>
        <Match when={currentPage() === "internal/not-found"}>
          <NotFound />
        </Match>
      </Switch>
    </Box>
  );
};
