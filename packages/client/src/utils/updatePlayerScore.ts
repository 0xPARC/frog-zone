import { GameResponse } from "./fetchGame";

interface PlayerGame {
	id: number;
	gameId: number; // Foreign key to Game
	playerId: number; // Foreign key to Player
	publicKey: string; // Player's public key
	machineId: string; // Machine ID associated with this PlayerGame (0-3)
	score: number; // Player's score in this game
	createdAt: Date; // Timestamp when the PlayerGame record was created
	updatedAt: Date; // Timestamp when the PlayerGame record was last updated
}

export interface PlayerScoreResponse {
	success: boolean;
	message?: string;
	playerGame?: PlayerGame;
}

export const updatePlayerScore = async ({
	gameId,
	publicKey,
	score,
}: {
	gameId: string;
	publicKey: string; // identifies the player
	score: number;
}): Promise<PlayerScoreResponse> => {
	try {
		const response = await fetch(`/api/game/${gameId}/player`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ score, publicKey }),
		});

		if (!response.ok) {
			throw new Error(
				`Failed to update game status: ${response.statusText}`,
			);
		}

		const data: PlayerScoreResponse = await response.json();
		console.log("Player score updated:", data);
		return data;
	} catch (error) {
		console.error("Error updating game status:", error);
		throw error;
	}
};
