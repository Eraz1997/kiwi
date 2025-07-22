import { useRouter } from "./contexts/router.jsx";
import { AdminIndex } from "./pages/admin/index.jsx";
import { AdminServices } from "./pages/admin/services.jsx";
import { AdminUsers } from "./pages/admin/users.jsx";
import { CreateUser } from "./pages/auth/createUser.jsx";
import { Login } from "./pages/auth/login.jsx";
import { NotFound } from "./pages/internal/notFound.jsx";
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
        <Match when={currentPage() === "admin"}>
          <AdminIndex />
        </Match>
        <Match when={currentPage() === "admin/users"}>
          <AdminUsers />
        </Match>
        <Match when={currentPage() === "admin/services"}>
          <AdminServices />
        </Match>
        <Match when={currentPage() === "internal/not-found"}>
          <NotFound />
        </Match>
      </Switch>
    </Box>
  );
};
