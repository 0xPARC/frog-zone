import type { PlayerGame } from "./updatePlayer";

export interface PlayerResponse {
	success: boolean;
	message?: string;
	playerGame?: PlayerGame;
}

export const requestEndGame = async (args: {
	gameId: string;
	machineId: string;
}): Promise<PlayerResponse> => {
	try {
		const { gameId, machineId } = args;
		const response = await fetch(`/api/game/${gameId}/player/end-game`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ hasEndedGame: true, machineId }),
		});

		if (!response.ok) {
			throw new Error(`Failed to end game for player: ${response.statusText}`);
		}

		const data: PlayerResponse = await response.json();
		return data;
	} catch (error) {
		console.error("Failed to end game for player:", error);
		throw error;
	}
};
