import { fetchTiles } from "../../../utils/getTilesAroundPlayer";
import { Coord, TileWithCoord } from "../../store";
// Fetches the tiles in batches of 5

const FETCH_INTERVAL = 1000;
export const createTileFetcher = ({
	initialCoordinates,
	batchSize,
	playerId,
	onSuccessfulFetch,
}: {
	initialCoordinates: Coord[];
	batchSize: number;
	playerId: number;
	onSuccessfulFetch: ({
		tiles,
		viewportCoords,
	}: {
		tiles: TileWithCoord[];
		viewportCoords: Coord[];
	}) => void;
}) => {
	let coordinates = initialCoordinates;
	let currentIndex = 0;
	let intervalId: number | null = null;

	const fetchNextBatch = async () => {
		const nextBatch = coordinates.slice(
			currentIndex,
			currentIndex + batchSize,
		);

		console.log("COORDINATES FETCHED", nextBatch);

		const newTiles = await fetchTiles(playerId, nextBatch);

		onSuccessfulFetch({ tiles: newTiles, viewportCoords: coordinates });

		currentIndex += batchSize;

		// If we have reached the end of the array, reset the index to 0 to start over
		if (currentIndex >= coordinates.length) {
			currentIndex = 0;
		}
	};

	const start = () => {
		if (intervalId === null) {
			intervalId = setInterval(() => {
				fetchNextBatch();
			}, FETCH_INTERVAL);
		}
	};

	const stop = () => {
		if (intervalId !== null) {
			clearInterval(intervalId);
			intervalId = null; // Reset intervalId so we can restart later
		}
	};

	// Updates the coordinates (current viewport) and reset the batch index to 0, and restart automatically
	const updateCoordinates = (newCoordinates: Coord[]) => {
		stop();
		coordinates = newCoordinates;
		currentIndex = 0;
		start();
	};

	return {
		start,
		stop,
		updateCoordinates,
	};
};
