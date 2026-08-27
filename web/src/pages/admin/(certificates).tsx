import {
	Beer,
	CalendarClock,
	CircleX,
	Signature,
	Telescope,
} from "lucide-solid";
import { type Component, createSignal, Show } from "solid-js";
import { Container, HStack, VStack } from "styled-system/jsx";
import { Alert, Button, Card, Heading, Spinner, Text } from "~/components";
import { useKangaroo } from "~/contexts/kangaroo";
import { useRouter } from "~/contexts/router";
import { createAsyncAction } from "~/hooks/createAsyncAction";
import { createBackendClient } from "~/hooks/createBackendClient";
import { createResourceWithInitialValue } from "~/hooks/createResourceWithInitialValue";
import type { CertificateInfo } from "~/types";
import { NavigationBar } from "./navigationBar";

export const Certificates: Component = () => {
	const adminClient = createBackendClient("admin");
	const { consumeAdminCertificatesData } = useKangaroo();
	const adminCertificatesData = consumeAdminCertificatesData();

	const { domain } = useRouter();

	const [error, setError] = createSignal<string>();
	const [certificateInfo, { refetch: reloadState }] =
		createResourceWithInitialValue<CertificateInfo>(async () => {
			const { jsonPayload } = await adminClient.get("/certificates");
			return jsonPayload;
		}, adminCertificatesData?.certificate);

	const { call: orderNewCertificate, isLoading: isNewOrderLoading } =
		createAsyncAction(async () => {
			const { statusCode, text: errorMessage } = await adminClient.post(
				"/certificates",
				{ domain: domain() },
			);

			if (statusCode === 200) {
				reloadState();
			} else {
				setError(errorMessage ?? "unknown error");
			}
		});

	const { call: verifyDns, isLoading: isVerifyingDns } = createAsyncAction(
		async () => {
			const { statusCode, text: errorMessage } =
				await adminClient.post("/finalise");

			if (statusCode === 200) {
				reloadState();
			} else {
				setError(errorMessage ?? "unknown error");
			}
		},
	);

	const isAnythingLoading = () => isNewOrderLoading() || isVerifyingDns();

	return (
		<>
			<NavigationBar me={adminCertificatesData?.me} />
			<Container p="12" maxW="md" overflowX="scroll">
				<VStack gap="6">
					<Show when={error()}>
						<Alert.Root borderColor="red.default">
							<Alert.Icon
								color="red.text"
								asChild={(iconProps) => <CircleX {...iconProps()} />}
							/>
							<Alert.Content>
								<Alert.Title color="red.text">Something went wrong</Alert.Title>
								<Alert.Description color="red.text">
									{error()}
								</Alert.Description>
							</Alert.Content>
						</Alert.Root>
					</Show>
					<Show
						when={!certificateInfo.loading}
						fallback={<Spinner size="xl" />}
					>
						<Card.Root>
							<Card.Header>
								<Card.Title>TLS Certificate Info</Card.Title>
							</Card.Header>
							<Card.Body>
								<VStack gap="4" alignItems="start" width="full">
									<Text textStyle="xs">
										TLS certificates are issued by Let's Encrypt for free.
									</Text>
									<Heading textStyle="md" display="flex" gap="2">
										Issuer <Signature />
									</Heading>
									<Text>{certificateInfo()?.issuer}</Text>
									<Heading textStyle="md" display="flex" gap="2">
										Expiration <CalendarClock />
									</Heading>
									<Text>{certificateInfo()?.expirationDate}</Text>
								</VStack>
							</Card.Body>
							<Card.Footer>
								<HStack gap="4">
									<Button
										bgColor={{
											base: "amber.light.9",
											_hover: "amber.light.11",
										}}
										loading={isAnythingLoading()}
										disabled={!certificateInfo()?.newPendingOrder}
										onClick={verifyDns}
									>
										Verify DNS
										<Telescope />
									</Button>
									<Button
										loading={isAnythingLoading()}
										onClick={orderNewCertificate}
									>
										Order New Certificate
										<Beer />
									</Button>
								</HStack>
							</Card.Footer>
						</Card.Root>
					</Show>
				</VStack>
			</Container>
		</>
	);
};
