import type { GameResponse } from "./fetchGame";

export interface GameStatusResponse {
	success: boolean;
	message: string;
	game?: GameResponse["game"];
}

export const updateGameStatus = async ({
	gameId,
	status,
}: {
	gameId: string;
	status: string;
}): Promise<GameStatusResponse> => {
	try {
		const response = await fetch(`/api/game/${gameId}`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ status }),
		});

		if (!response.ok) {
			throw new Error(`Failed to update game status: ${response.statusText}`);
		}

		const data: GameStatusResponse = await response.json();
		console.log("Game status updated:", data);
		return data;
	} catch (error) {
		console.error("Error updating game status:", error);
		throw error;
	}
};
