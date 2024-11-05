import { DEV_MODE } from "../const/env.const";

export interface PlayerGame {
	id: number;
	gameId: number; // Foreign key to Game
	playerId: number; // Foreign key to Player
	publicKey: string; // Player's public key
	machineId: string; // Machine ID associated with this PlayerGame (0-3)
	score: number; // Player's score in this game
	createdAt: Date; // Timestamp when the PlayerGame record was created
	updatedAt: Date; // Timestamp when the PlayerGame record was last updated
}

export interface PlayerResponse {
	success: boolean;
	message?: string;
	playerGame?: PlayerGame;
}

export const updatePlayer = async (args: {
	gameId: string;
	publicKey: string; // identifies the player
	score?: number;
}): Promise<PlayerResponse> => {
	if (!DEV_MODE) {
		try {
			const { gameId, publicKey, ...rest } = args;
			const response = await fetch(`/api/game/${gameId}/player`, {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({ ...rest, publicKey }),
			});

			if (!response.ok) {
				throw new Error(
					`Failed to update player: ${response.statusText}`,
				);
			}

			const data: PlayerResponse = await response.json();
			return data;
		} catch (error) {
			console.error("Error updating player:", error);
			throw error;
		}
	}

	return {
		success: true,
	};
};
