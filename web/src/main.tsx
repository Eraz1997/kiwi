/* @refresh reload */
import { App } from "./app.jsx";
import { RouterProvider } from "./contexts/router.jsx";
import "./index.css";
import { render } from "solid-js/web";

render(
  () => (
    <RouterProvider>
      <App />
    </RouterProvider>
  ),
  document.getElementById("root") as HTMLElement,
);
