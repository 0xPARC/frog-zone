import { GameResponse } from "./fetchGame";

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
}) => {
	try {
		const response = await fetch(`/api/game/${gameId}`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ status }),
		});
		const data = await response.json();
		return data;
	} catch (error) {
		console.error("Error updating game status:", error);
		throw error;
	}
};
