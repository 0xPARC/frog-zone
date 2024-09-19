import { serverUrl } from "./createEventStream";

export enum Direction {
	UP = "Up",
	DOWN = "Down",
	LEFT = "Left",
	RIGHT = "Right",
}

interface type {
	player_id: number;
	direction: Direction;
}

export type Api = ReturnType<typeof createApi>;

const createApi = () => {
	const move = async (
		playerId: number,
		direction: Direction,
	): Promise<string> => {
		const moveRequest: type = { player_id: playerId, direction };

		const response = await fetch(`${serverUrl}/move`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(moveRequest),
		});

		if (!response.ok) {
			throw new Error("Failed to queue move");
		}

		return response.text();
	};

	return { move };
};

export default createApi;
