import {
  Accessor,
  Component,
  JSX,
  createContext,
  createSignal,
  useContext,
} from "solid-js";

// Types

export type Page = "auth/login" | "notFound";

type QueryParams = {
  [index: string]: string;
};

type Router = {
  currentPage: Accessor<Page>;
  domain: Accessor<string>;
  queryParams: Accessor<QueryParams>;
  isLocalhost: Accessor<boolean>;
};

type Props = {
  children?: JSX.Element | JSX.Element[];
};

// Context

const RouterContext = createContext<Router>();

export const RouterProvider: Component<Props> = (props) => {
  const initialPage = getPageFromLocation(window.location);
  const initialQueryParams = getQueryParamsFromLocation(window.location);
  const initialDomain = getDomainFromLocation(window.location);
  const initialLocalhost = isLocalhostFromLocation(window.location);

  const [domain] = createSignal(initialDomain);
  const [currentPage] = createSignal<Page>(initialPage);
  const [queryParams] = createSignal<QueryParams>(initialQueryParams);
  const [isLocalhost] = createSignal(initialLocalhost);

  return (
    <RouterContext.Provider
      value={{
        currentPage,
        domain,
        queryParams,
        isLocalhost,
      }}
    >
      {props.children}
    </RouterContext.Provider>
  );
};

// Hook

const useRouterContext = () => useContext<Router | undefined>(RouterContext);

export const useRouter = () => {
  const router = useRouterContext();
  if (router === undefined) {
    throw new Error("useRouter must be used within a RouterProvider");
  }
  return router;
};

// Helpers

const getDomainFromLocation = (location: Location): string => {
  const domains = location.host.split(".");
  if (
    domains.length !== 3 &&
    domains.length !== 2 &&
    !domains[domains.length - 1].startsWith("localhost:")
  ) {
    throw "invalid domain";
  }
  return domains.toSpliced(0, 1).join(".");
};

const getPageFromLocation = (location: Location): Page => {
  const domain = getDomainFromLocation(location);
  const subdomain = location.host.replace(`.${domain}`, "");
  const parts = location.href.split(location.host);
  const path = parts[parts.length - 1].split("?")[0];

  if (subdomain === "auth" && path === "/login") {
    return "auth/login";
  }
  return "notFound";
};

const getQueryParamsFromLocation = (location: Location): QueryParams => {
  const search = location.search ?? "?";
  const initialisedQueryParams: QueryParams = {};
  const items = search.split("?");
  if (items.length <= 1) {
    return {};
  }
  return items[1].split("&").reduce((queryParams, pair) => {
    const items = pair.split("=");
    const key = items[0];
    const value = decodeURIComponent(items[1]);
    queryParams[key] = value;
    return queryParams;
  }, initialisedQueryParams);
};

const isLocalhostFromLocation = (location: Location): boolean => {
  const domain = getDomainFromLocation(location);
  const domains = domain.split(".");
  const tld = domains[domains.length - 1];
  return tld.startsWith("localhost:");
};
