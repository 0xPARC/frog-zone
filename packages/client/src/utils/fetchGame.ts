export interface GameResponse {
	success: boolean;
	message: string;
	game?: {
		id: number;
		gameId: string;
		status: string;
		createdAt: string;
		updatedAt: string;
		machines: {
			id: number;
			machineId: string;
			publicKey: string;
			gameId: number;
			createdAt: string;
			updatedAt: string;
		}[];
		players: {
			id: string;
			publicKey: string;
			score: number;
			machineId: string;
		}[];
		wasAborted: boolean;
	};
}

export const fetchGame = async ({
	gameId,
}: {
	gameId: string;
}): Promise<GameResponse> => {
	try {
		const response = await fetch(`/api/game/${gameId}`);

		if (!response.ok) {
			throw new Error(`Failed to fetch game: ${response.statusText}`);
		}

		const data: GameResponse = await response.json();
		return data;
	} catch (error) {
		console.error("Error fetching game:", error);
		throw error;
	}
};
