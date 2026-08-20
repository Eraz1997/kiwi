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
	imageName: string | null;
	imageSha: { value: string };
	exposedPort: { internal: number; external: number };
	environmentVariables: EnvironmentVariable[];
	secrets: EnvironmentVariable[];
	internalSecrets: EnvironmentVariable[];
	statefulVolumePaths: string[];
	githubRepository: GithubRepository | null;
	requiredRole: Role | null;
};

export type Service = {
	containerConfiguration: ContainerConfiguration;
	createdAt: Date;
	lastModifiedAt: Date;
	lastDeployedAt: Date;
};

export enum ContainerLogType {
	Output = "Output",
	Error = "Error",
	Input = "Input",
	Console = "Console",
}

export type ContainerLog = {
	message: string;
	logType: ContainerLogType;
};
