import { TerrainType } from "../game/store";

export interface Grid {
	[key: string]: {
		terrainType: TerrainType;
	};
}

export const findBorderCoordinates = (grid: Grid): string[] => {
	const result: string[] = [];
	const getNeighbors = (x: number, y: number): string[] => [
		`${x - 1},${y}`, // Up
		`${x + 1},${y}`, // Down
		`${x},${y - 1}`, // Left
		`${x},${y + 1}`, // Right
	];

	for (const key in grid) {
    const myTerrain = grid[key as keyof Grid].terrainType;
		if (myTerrain === "WATER" || myTerrain === "ROCK") {
			const [x, y] = key.split(",").map(Number);

			const hasLandNeighbor = getNeighbors(x, y).some(
        (neighbor) => {
          const neighborTerrain = grid[neighbor]?.terrainType;
          // if water is next to ice/grass/sand it is a border tile
          if (myTerrain === "WATER" && ["ICE", "GRASS", "SAND"].includes(neighborTerrain)) return true;
          // if rock is next to grass/sand it is a border tile. if it is next to ice we don't need to count it in the current map
          if (myTerrain === "ROCK" && ["GRASS", "SAND"].includes(neighborTerrain)) return true;
          return false;
        },
			);
			if (hasLandNeighbor) {
				result.push(key);
			}
		}
	}

	return result;
};
