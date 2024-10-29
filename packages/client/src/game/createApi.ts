import { IS_MOCK } from "../const/env.const";

const serverUrl = import.meta.env.VITE_SERVER_URL;

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

interface MoveResponse {
	my_new_coords: {
		x: number;
		y: number;
	};
	rate_limited: boolean;
}

export type Api = ReturnType<typeof createApi>;

const createApi = () => {
	const move = async (
		playerId: number,
		direction: Direction,
	): Promise<MoveResponse> => {
		const moveRequest: type = {
			player_id: playerId,
			direction: direction,
		};

		const response = await fetch(`${serverUrl}/${IS_MOCK ? "mock_move" : "move"}`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(moveRequest),
		});

		if (!response.ok) {
			throw new Error("Failed to queue move");
		}

		const data = await response.json();

		return data;
	};

	return { move };
};

export default createApi;
