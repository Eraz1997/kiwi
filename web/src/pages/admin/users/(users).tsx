import { XIcon } from "lucide-solid";
import { type Component, For } from "solid-js";
import { Container } from "styled-system/jsx";
import { IconButton, Table, Toast } from "~/components";
import { useKangaroo } from "~/contexts/kangaroo";
import { createBackendClient } from "~/hooks/createBackendClient";
import { createResourceWithInitialValue } from "~/hooks/createResourceWithInitialValue";
import type { User } from "~/types";
import { NavigationBar } from "../navigationBar";
import { DeleteUserDialog } from "./deleteUserDialog";
import { InviteUserDialog } from "./inviteUserDialog";

const toaster = Toast.createToaster({
	placement: "bottom-end",
	overlap: true,
	gap: 16,
});

export const AdminUsers: Component = () => {
	const adminClient = createBackendClient("admin");
	const { consumeAdminUsersData } = useKangaroo();
	const adminUsersData = consumeAdminUsersData();

	const [currentUser] = createResourceWithInitialValue<User>(async () => {
		const { jsonPayload: user } = await adminClient.get("/users/me");
		return user;
	}, adminUsersData?.me);
	const [users, { refetch: reloadUsers }] = createResourceWithInitialValue<
		User[]
	>(async () => {
		const { jsonPayload } = await adminClient.get("/users");
		return jsonPayload;
	}, adminUsersData?.users);

	return (
		<>
			<NavigationBar me={adminUsersData?.me} />
			<Container p="12" maxW="4xl" overflowX="scroll">
				<Table.Root>
					<Table.Head>
						<Table.Row>
							<Table.Header>Username</Table.Header>
							<Table.Header textAlign="end">Role</Table.Header>
							<Table.Header />
						</Table.Row>
					</Table.Head>
					<Table.Body>
						<For each={users()}>
							{(user) => (
								<Table.Row>
									<Table.Cell fontWeight="medium">{user.username}</Table.Cell>
									<Table.Cell textAlign="end">{user.role}</Table.Cell>
									<Table.Cell width="24" textAlign="end">
										<DeleteUserDialog
											userToDelete={user}
											authenticatedUser={currentUser()}
											reloadUsers={reloadUsers}
											createToast={toaster.create}
										/>
									</Table.Cell>
								</Table.Row>
							)}
						</For>
					</Table.Body>
					<Table.Foot>
						<Table.Row>
							<Table.Cell />
							<Table.Cell />
							<Table.Cell textAlign="end">
								<InviteUserDialog createToast={toaster.create} />
							</Table.Cell>
						</Table.Row>
					</Table.Foot>
				</Table.Root>
			</Container>
			<Toast.Toaster toaster={toaster}>
				{(toast) => {
					const color = toast().type === "success" ? "lime" : "red";
					return (
						<Toast.Root borderColor={`${color}.default`}>
							<Toast.Title color={`${color}.text`}>{toast().title}</Toast.Title>
							<Toast.Description color={`${color}.text`}>
								{toast().description}
							</Toast.Description>
							<Toast.CloseTrigger color={`${color}.text`}>
								<IconButton size="sm" variant="link">
									<XIcon />
								</IconButton>
							</Toast.CloseTrigger>
						</Toast.Root>
					);
				}}
			</Toast.Toaster>
		</>
	);
};
