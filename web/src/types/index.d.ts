declare enum Role {
  Admin = "Admin",
  Customer = "Customer",
}

declare type User = {
  username: string;
  role: Role;
};
