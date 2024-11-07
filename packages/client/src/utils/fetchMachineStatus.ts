import type { GameResponse } from "./fetchGame";

export interface MachineStatusResponse {
	isLoggedIn: boolean;
	publicKey: string | null;
	game: GameResponse["game"] | null;
}

export const fetchMachineStatus = async ({
	playerId,
}: {
	playerId: string;
}): Promise<MachineStatusResponse> => {
	try {
		const response = await fetch(`/api/machine-status/${playerId}`);
		if (!response.ok) {
			throw new Error(`Failed to fetch: ${response.statusText}`);
		}
		const data: MachineStatusResponse = await response.json();
		return data;
	} catch (error) {
		console.error("Error fetching machine status:", error);
		throw error;
	}
};
