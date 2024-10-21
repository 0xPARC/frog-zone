import { SERVER_URL } from "../const/env.const";
import { Coord, Tile, TileWithCoord } from "../game/store";

export const fetchTiles = async (
	playerId: number,
	coords: Coord[],
): Promise<TileWithCoord[]> => {
	try {
		const response = await fetch(`${SERVER_URL}/get_cells`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				player_id: playerId,
				coords: coords.map(({ x, y }) => ({
					x: {
						val: x,
					},
					y: {
						val: y,
					},
				})),
			}),
		});

		if (!response.ok) {
			throw new Error(`Failed to fetch: ${response.statusText}`);
		}

		const data = await response.json();
		const dataWithCoords = data.cell_data.map(
			(item: Tile, i: number): TileWithCoord => ({
				...item,
				coord: coords[i],
				fetchedAt: Date.now(),
			}),
		);
		return dataWithCoords;
	} catch (error) {
		console.error("Error fetching cells:", error);
		throw error;
	}
};
