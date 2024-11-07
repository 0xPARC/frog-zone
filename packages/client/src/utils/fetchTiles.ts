import { IS_MOCK, SERVER_URL } from "../const/env.const";
import type { Coord, Tile, TileWithCoord } from "../game/store";

/*

// 20% perf optimization we can add later

const isCross = (coords: Coord[], player_coord: Coord): boolean => {
  if (coords.length !== 5) return false;

  let hasNorth = false;
  let hasSouth = false;
  let hasEast = false;
  let hasWest = false;
  let hasCenter = false;

  for (let i = 0; i < coords.length; i++) {
    const coord = coords[i];
    if (coord.x === player_coord.x && coord.y === player_coord.y) {
      hasCenter = true;
    } else if (coord.x === player_coord.x && coord.y === player_coord.y - 1) {
      hasNorth = true;
    } else if (coord.x === player_coord.x && coord.y === player_coord.y + 1) {
      hasSouth = true;
    } else if (coord.x === player_coord.x - 1 && coord.y === player_coord.y) {
      hasWest = true;
    } else if (coord.x === player_coord.x + 1 && coord.y === player_coord.y) {
      hasEast = true;
    }
  }

  return (hasNorth && hasSouth && hasEast && hasWest && hasCenter);
}

const getVerticalLineCenter = (coords: Coord[], player_coord: Coord): Coord | undefined => {
  if (coords.length !== 5) return undefined;

  // check that all coords are in the same column
  const x = coords[0].x;
  for (const coord of coords) {
    if (coord.x !== x) return undefined;
  }

  // check column x is within 2 tiles of player x
  if (Math.abs(x - player_coord.x) > 2) return undefined;

  // check all distinct and in a row
  const ys = coords.map((coord) => coord.y);
  ys.sort();
  for (let i = 0; i < 3; i++) {
    if (ys[i] + 1 !== ys[i + 1]) return undefined;
  }

  // check that middle y equals player y
  if (ys[2] !== player_coord.y) return undefined;

  return {x, y: ys[2]};
}

const getHorizontalLineCenter = (coords: Coord[], player_coord: Coord): Coord | undefined => {
  if (coords.length !== 5) return undefined;

  // check that all coords are in the same row
  const y = coords[0].y;
  for (const coord of coords) {
    if (coord.y !== y) return undefined;
  }

  // check row y is within 2 tiles of player y
  if (Math.abs(y - player_coord.y) > 2) return undefined;

  // check all distinct and in a row
  const xs = coords.map((coord) => coord.x);
  xs.sort();
  for (let i = 0; i < 3; i++) {
    if (xs[i] + 1 !== xs[i + 1]) return undefined;
  }

  // check that middle x equals player x
  if (xs[2] !== player_coord.x) return undefined;

  return {x: xs[2], y};
}
*/

export const fetchTiles = async (
	player_coord: Coord,
	coords_to_fetch: Coord[],
): Promise<TileWithCoord[]> => {
	try {
		const body = JSON.stringify({
			coords: coords_to_fetch,
		});
		let route = "get_cells";

		if (coords_to_fetch.length === 5) route = "get_five_cells";

		/*

    // 20% perf optimization we can add later

    const verticalLineCenter = getVerticalLineCenter(coords_to_fetch, player_coord);
    const horizontalLineCenter = getHorizontalLineCenter(coords_to_fetch, player_coord);

    if (isCross(coords_to_fetch, player_coord)) {
      route = "get_cross_cells";
      body = JSON.stringify({});
    } else if (verticalLineCenter) {
      route = "get_vertical_cells";
      body = JSON.stringify({
        center_coord: verticalLineCenter,
      });
    } else if (horizontalLineCenter) {
      route = "get_horizontal_cells";
      body = JSON.stringify({
        center_coord: horizontalLineCenter,
      });
    } else if (coords_to_fetch.length === 5) {
      route = "get_five_cells";
    }
    */

		if (IS_MOCK) route = "mock_get_cells";

		const response = await fetch(`${SERVER_URL}/${route}`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body,
		});

		if (!response.ok) {
			throw new Error(`Failed to fetch: ${response.statusText}`);
		}

		const data = await response.json();
		const dataWithCoords = data.cell_data.map(
			(item: Tile, i: number): TileWithCoord => ({
				...item,
				coord: coords_to_fetch[i],
				fetchedAt: Date.now(),
			}),
		);
		return dataWithCoords;
	} catch (error) {
		console.error("Error fetching cells:", error);
		throw error;
	}
};
