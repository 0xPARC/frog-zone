import { SERVER_URL } from "../const/env.const";
// tells the game server to reset all players
export const resetGame = async () => {
	try {
		// TODD: implement endpoint
		const response = await fetch(`${SERVER_URL}/reset_game`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
		});

		if (!response.ok) {
			throw new Error(`Failed to reset game: ${response.statusText}`);
		}

		const data = await response.json();
		return data;
	} catch (error) {
		console.error("Error resetting game:", error);
		throw error;
	}
};
