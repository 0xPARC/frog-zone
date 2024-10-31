import { TerrainType } from "../game/store";

export interface Grid {
	[key: string]: {
		terrainType: TerrainType;
	};
}

export const findBorderingWaterCoordinates = (grid: Grid): string[] => {
	const result: string[] = [];
	const getNeighbors = (x: number, y: number): string[] => [
		`${x - 1},${y}`, // Up
		`${x + 1},${y}`, // Down
		`${x},${y - 1}`, // Left
		`${x},${y + 1}`, // Right
	];

	for (const key in grid) {
		if (grid[key as keyof Grid].terrainType === "WATER") {
			const [x, y] = key.split(",").map(Number);

			const hasLandNeighbor = getNeighbors(x, y).some(
				(neighbor) => grid[neighbor]?.terrainType === "LAND",
			);
			if (hasLandNeighbor) {
				result.push(key);
			}
		}
	}

	return result;
};
