import { useRouter } from "src/contexts/router";

type ParsedResponse = {
  statusCode: number;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  jsonPayload: any | null;
  text: string | null;
};

type Service = "auth" | "admin";

type BackendClient = {
  get: (path: string) => Promise<ParsedResponse>;
  post: (path: string, body?: unknown) => Promise<ParsedResponse>;
  delete: (path: string, body?: unknown) => Promise<ParsedResponse>;
};

export const createBackendClient = (service: Service): BackendClient => {
  const { domain, isLocalhost } = useRouter();

  const scheme = () => (isLocalhost() ? "http://" : "https://");
  const baseUrl = () => `${scheme()}${service}.${domain()}/api`;

  const request = async (
    path: string,
    method: string,
    body?: unknown,
  ): Promise<ParsedResponse> => {
    const response = await fetch(`${baseUrl()}${path}`, {
      method,
      body: body ? JSON.stringify(body) : undefined,
      headers: { "Content-Type": "application/json" },
    });

    let jsonPayload, text;
    try {
      jsonPayload = await response.json();
    } catch {
      jsonPayload = null;
    }
    try {
      text = await response.text();
    } catch {
      text = null;
    }

    return {
      statusCode: response.status,
      jsonPayload,
      text,
    };
  };

  return {
    get: async (path) => await request(path, "GET"),
    post: async (path, body) => await request(path, "POST", body),
    delete: async (path, body) => await request(path, "DELETE", body),
  };
};
