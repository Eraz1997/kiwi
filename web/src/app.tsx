import { useRouter } from "./contexts/router.jsx";
import { CreateUser } from "./pages/createUser.jsx";
import { Login } from "./pages/login.jsx";
import { NotFound } from "./pages/notFound.jsx";
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
        <Match when={currentPage() === "internal/not-found"}>
          <NotFound />
        </Match>
      </Switch>
    </Box>
  );
};
