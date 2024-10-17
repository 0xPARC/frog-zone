import { Coord, Tile, TileWithCoord } from "../game/store";
import { getSurroundingCoordinates } from "./getSurroundingCoordinates";
export const serverUrl = import.meta.env.VITE_SERVER_URL;

export const getTilesAroundPlayer = async ({
	playerId,
	coord,
}: {
	playerId: number;
	coord: Coord;
}): Promise<TileWithCoord[]> => {
	try {
		const coords = getSurroundingCoordinates(coord);
		const response = await fetch(`${serverUrl}/get_cells`, {
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
			}),
		);
		return dataWithCoords;
	} catch (error) {
		console.error("Error fetching cells:", error);
		throw error;
	}
};
