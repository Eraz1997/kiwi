import {
  Accessor,
  Component,
  JSX,
  createContext,
  createSignal,
  onCleanup,
  onMount,
  useContext,
} from "solid-js";

// Types

export type Page =
  | "auth/login"
  | "auth/create-user"
  | "internal/not-found"
  | "admin"
  | "admin/users"
  | "admin/services";

type QueryParams = {
  [index: string]: string;
};

type Router = {
  currentPage: Accessor<Page>;
  domain: Accessor<string>;
  queryParams: Accessor<QueryParams>;
  isLocalhost: Accessor<boolean>;
  navigate: (page: Page, queryParams?: QueryParams) => void;
  isValidReturnUri: (returnUri: string) => boolean;
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

  const [domain, setDomain] = createSignal(initialDomain);
  const [currentPage, setCurrentPage] = createSignal<Page>(initialPage);
  const [queryParams, setQueryParams] =
    createSignal<QueryParams>(initialQueryParams);
  const [isLocalhost, setIsLocalHost] = createSignal(initialLocalhost);

  const urlChangeEventListener = () => {
    setCurrentPage(getPageFromLocation(window.location));
    setQueryParams(getQueryParamsFromLocation(window.location));
    setDomain(getDomainFromLocation(window.location));
    setIsLocalHost(isLocalhostFromLocation(window.location));
  };

  onMount(() => {
    window.addEventListener("popstate", urlChangeEventListener);
  });

  onCleanup(() => {
    window.removeEventListener("popstate", urlChangeEventListener);
  });

  const navigate = (page: Page, queryParams?: QueryParams) => {
    const pageParts = page.split("/");
    const service = pageParts[0];
    const path = pageParts.length > 1 ? pageParts.slice(1).join("/") : "";
    const scheme = isLocalhost() ? "http://" : "https://";
    const encodedQueryParams = queryParams
      ? `?${encodeQueryParams(queryParams)}`
      : "";
    const url = `${scheme}${service}.${domain()}/${path}${encodedQueryParams}`;
    window.history.pushState(null, "", url);

    setCurrentPage(page);
    setQueryParams(queryParams ?? {});
  };

  const isValidReturnUri = (returnUri: string) => {
    const scheme = isLocalhost() ? "http://" : "https://";
    if (!returnUri.startsWith(scheme)) {
      return false;
    }

    const uriParts = returnUri.substring(0, scheme.length).split("/");
    if (!uriParts.length) {
      return false;
    }

    const host = uriParts[0];
    const domains = host.split(".");
    if (domains.length !== 3) {
      return false;
    }

    const returnDomain = domains.toSpliced(0, 1).join(".");
    return returnDomain === domain();
  };

  return (
    <RouterContext.Provider
      value={{
        currentPage,
        domain,
        queryParams,
        isLocalhost,
        navigate,
        isValidReturnUri,
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
  if (domains.length !== 3) {
    throw "invalid domain";
  }
  return domains.toSpliced(0, 1).join(".");
};

const getPageFromLocation = (location: Location): Page => {
  const domain = getDomainFromLocation(location);
  const subdomain = location.host.replace(`.${domain}`, "");
  const parts = location.href.split(location.host);
  const path = parts[parts.length - 1]
    .split("?")[0]
    .split("/")
    .filter((part, index) => !!part || index === 0)
    .join("/");

  if (subdomain === "auth" && path === "/login") {
    return "auth/login";
  }
  if (subdomain === "auth" && path === "/create-user") {
    return "auth/create-user";
  }
  if (subdomain === "admin" && (!path || path === "/")) {
    return "admin";
  }
  if (subdomain === "admin" && path === "/users") {
    return "admin/users";
  }
  if (subdomain === "admin" && path === "/services") {
    return "admin/services";
  }
  return "internal/not-found";
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

const encodeQueryParams = (queryParams: QueryParams): string => {
  return Object.entries(queryParams)
    .map(
      ([key, value]) =>
        `${encodeURIComponent(key)}=${encodeURIComponent(value)}`,
    )
    .join("&");
};

const isLocalhostFromLocation = (location: Location): boolean => {
  const domain = getDomainFromLocation(location);
  return domain.startsWith("kiwi-local.com:");
};
