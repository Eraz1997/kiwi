export enum Role {
  Admin = "Admin",
  Customer = "Customer",
}

export type User = {
  username: string;
  role: Role;
};

type EnvironmentVariable = { name: string; value: string };
type GithubRepository = { name: string; owner: string };

export type ContainerConfiguration = {
  name: string;
  image_name: string;
  image_sha: { value: string };
  exposed_port: { internal: number; external: number };
  environment_variables: EnvironmentVariable[];
  secrets: EnvironmentVariable[];
  internal_secrets: EnvironmentVariable[];
  stateful_volume_paths: string[];
  github_repository: GithubRepository | null;
  required_role: Role | null;
};

export type Service = {
  container_configuration: ContainerConfiguration;
  created_at: Date;
  last_modified_at: Date;
  last_deployed_at: Date;
};

export enum ContainerLogType {
  Output = "Output",
  Error = "Error",
  Input = "Input",
  Console = "Console",
}

export type ContainerLog = {
  message: string;
  log_type: ContainerLogType;
};
