import { createContext, createSignal, type JSX, useContext } from "solid-js";
import type {
	CertificateInfo,
	ContainerConfiguration,
	ContainerLog,
	Service,
	User,
} from "~/types";

type GenericAdminPayload = {
	me: User;
};

type AdminUsersPayload = {
	me: User;
	users: User[];
};

type AdminCertificatesPayload = {
	me: User;
	certificate: CertificateInfo;
};

type AdminDynamicDnsPayload = {
	me: User;
	enabled: boolean;
};

type AdminServicesPayload = {
	me: User;
	services: Service[];
};

type AdminEditServicePayload = {
	me: User;
	generalInfo: {
		containerConfiguration: ContainerConfiguration;
	};
	status: string;
	logs: ContainerLog[];
};

type KangarooDataPayload =
	| GenericAdminPayload
	| AdminUsersPayload
	| AdminCertificatesPayload
	| AdminDynamicDnsPayload
	| AdminServicesPayload
	| AdminEditServicePayload;

type KangarooData = {
	consumeGenericAdminData: () => GenericAdminPayload | null;
	consumeAdminUsersData: () => AdminUsersPayload | null;
	consumeAdminCertificatesData: () => AdminCertificatesPayload | null;
	consumeAdminDynamicDnsData: () => AdminDynamicDnsPayload | null;
	consumeAdminServicesData: () => AdminServicesPayload | null;
	consumeAdminEditServiceData: () => AdminEditServicePayload | null;
};

const KangarooContext = createContext<KangarooData | undefined>();

export function useKangaroo() {
	const context = useContext(KangarooContext);

	if (!context) {
		throw new Error("useKangaroo must be used inside KangarooProvider");
	}

	return context;
}

export const KangarooProvider = (props: { children: JSX.Element }) => {
	const [consumed, setIsConsumed] = createSignal<boolean>(false);

	const kangarooDataRaw = JSON.parse(
		document.getElementById("kangaroo-data")?.textContent ?? "null",
	);
	const kangarooData =
		kangarooDataRaw && !kangarooDataRaw.errorMessage ? kangarooDataRaw : null;

	const consumeData = (): KangarooDataPayload | null => {
		const wasConsumed = consumed();
		setIsConsumed(true);
		return wasConsumed ? null : kangarooData;
	};

	return (
		<KangarooContext.Provider
			value={{
				consumeGenericAdminData: () => {
					const data = consumeData();
					return data && "me" in data ? data : null;
				},
				consumeAdminUsersData: () => {
					const data = consumeData();
					return data && "users" in data ? data : null;
				},
				consumeAdminCertificatesData: () => {
					const data = consumeData();
					return data && "certificate" in data ? data : null;
				},
				consumeAdminDynamicDnsData: () => {
					const data = consumeData();
					return data && "enabled" in data ? data : null;
				},
				consumeAdminServicesData: () => {
					const data = consumeData();
					if (!(data && "services" in data)) return null;
					data.services.forEach((service) => {
						service.createdAt = new Date(service.createdAt);
						service.lastModifiedAt = new Date(service.lastModifiedAt);
						service.lastDeployedAt = new Date(service.lastDeployedAt);
					});

					return data;
				},
				consumeAdminEditServiceData: () => {
					const data = consumeData();
					return data && "generalInfo" in data ? data : null;
				},
			}}
		>
			{props.children}
		</KangarooContext.Provider>
	);
};
