import { LOGIN_SERVER_URL } from "../const/env.const";

export interface MachineStatusResponse {
	isLoggedIn: boolean;
}

export const fetchMachineStatus = async ({
	playerId,
}: {
	playerId: string;
}): Promise<MachineStatusResponse> => {
	try {
		const response = await fetch(
			`${LOGIN_SERVER_URL}/api/machine-status/${playerId}`,
		);
		if (!response.ok) {
			throw new Error(`Failed to fetch: ${response.statusText}`);
		}
		const data: MachineStatusResponse = await response.json();
		console.log("Machine status:", data);
		return data;
	} catch (error) {
		console.error("Error fetching machine status:", error);
		throw error;
	}
};
