import { IS_MOCK, SERVER_URL } from "../const/env.const";
type PlayerData = {
	player_data: {
		atk: number;
		hp: number;
		loc: {
			x: number;
			y: number;
		};
	};
};

// Fetches the player's current location and stats using their player id
export const fetchPlayer = async (playerId: number): Promise<PlayerData> => {
	try {
		const response = await fetch(
			`${SERVER_URL}/${IS_MOCK ? "mock_get_player" : "get_player"}`,
			{
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					player_id: playerId,
				}),
			},
		);

		if (!response.ok) {
			throw new Error(`Failed to fetch: ${response.statusText}`);
		}

		const data = await response.json();
		return {
			...data,
			player_id: playerId,
		};
	} catch (error) {
		console.error("Error fetching player data:", error);
		throw error;
	}
};
