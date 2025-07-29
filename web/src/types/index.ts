export enum Role {
  Admin = "Admin",
  Customer = "Customer",
}

export type User = {
  username: string;
  role: Role;
};
