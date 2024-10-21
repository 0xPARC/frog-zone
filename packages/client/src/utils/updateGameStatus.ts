import { LOGIN_SERVER_URL } from "../const/env.const";
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
		await fetch(`${LOGIN_SERVER_URL}/api/game/${gameId}`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			mode: "no-cors",
			body: JSON.stringify({ status }),
		});
	} catch (error) {
		console.error("Error updating game status:", error);
		throw error;
	}
};
