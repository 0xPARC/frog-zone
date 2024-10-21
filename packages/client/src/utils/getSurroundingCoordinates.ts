import { Coord } from "../game/store";

/**
 * Generates an array of coordinates in a 5x5 grid around a given center coordinate.
 * The center coordinate will be surrounded by coordinates at offsets of -2, -1, 0, 1, and 2.
 * Coordinates with negative values are excluded.
 * Returns the coordinates ordered by their distance from the center (closest to farthest).
 *
 * @param {number} x - The x coordinate of the center.
 * @param {number} y - The y coordinate of the center.
 * @returns {Array<{ x: number, y: number }>} An array of coordinates around the center.
 */
export const getSurroundingCoordinates = ({ x, y }: Coord) => {
	const surroundingCoords = [];

	// Loop through -2 to 2 for both x and y to generate a 5x5 grid
	for (let dx = -2; dx <= 2; dx++) {
		for (let dy = -2; dy <= 2; dy++) {
			const newX = x + dx;
			const newY = y + dy;

			if (newX >= 0 && newY >= 0) {
				surroundingCoords.push({ x: newX, y: newY });
			}
		}
	}

	// Sort the coordinates by their Taxicab distance to the center (x, y)
	surroundingCoords.sort((a, b) => {
		const distA = Math.abs(a.x - x) + Math.abs(a.y - y);
		const distB = Math.abs(b.x - x) + Math.abs(b.y - y);
		return distA - distB; // Sort by closest to farthest
	});

	return surroundingCoords;
};
