import { coordToKey } from "@smallbraingames/small-phaser";
import { fetchTiles } from "../../../utils/fetchTiles";
import { getSurroundingCoordinates } from "../../../utils/getSurroundingCoordinates";
import useStore, { Coord, TileWithCoord } from "../../store";

const FETCH_INTERVAL = 1000;
const STALE_TIME_MS = 5000;

export const createTileFetcher = ({
	initialCoordinate,
	batchSize,
	playerId,
	onSuccessfulFetch,
}: {
	initialCoordinate: Coord; // the center coordinate of the viewport
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
	let coordinates = getSurroundingCoordinates(initialCoordinate);
	let currentIndex = 0;
	let intervalId: ReturnType<typeof setInterval> | null = null;

	const fetchNextBatch = async () => {
		const nextBatch = coordinates.slice(
			currentIndex,
			currentIndex + batchSize,
		);

		console.log("COORDINATES FETCHED", nextBatch);

		const newTiles = await fetchTiles(playerId, nextBatch);

		onSuccessfulFetch({ tiles: newTiles, viewportCoords: coordinates });

		currentIndex += batchSize;

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
			intervalId = null;
		}
	};

	const updateCoordinates = (newCoordinate: Coord) => {
		stop();
		coordinates = getOrderedQueue(getSurroundingCoordinates(newCoordinate));
		currentIndex = 0;
		start();
	};

	const getOrderedQueue = (coords: Coord[]): Coord[] => {
		const grid = useStore.getState().grid;
		const now = Date.now();

		const staleCoordinates = coords.filter((coord) => {
			const tile = grid.get(coordToKey(coord));
			if (tile) {
				return (
					tile?.fetchedAt === 0 || // Tile has never been fetched
					now - tile?.fetchedAt > STALE_TIME_MS // Tile is too old
				);
			}
			return false;
		});

		const freshCoordinates = coords.filter((coord) => {
			const tile = grid.get(coordToKey(coord));
			return tile && now - tile.fetchedAt <= STALE_TIME_MS;
		});

		// Sort both stale and fresh coordinates by taxicab distance from the player
		staleCoordinates.sort(
			(a, b) =>
				taxicabDistance(a, initialCoordinate) -
				taxicabDistance(b, initialCoordinate),
		);
		freshCoordinates.sort(
			(a, b) =>
				taxicabDistance(a, initialCoordinate) -
				taxicabDistance(b, initialCoordinate),
		);

		return [...staleCoordinates, ...freshCoordinates];
	};

	// Taxicab distance (Manhattan distance) calculation
	const taxicabDistance = (coord1: Coord, coord2: Coord): number => {
		const dx = Math.abs(coord1.x - coord2.x);
		const dy = Math.abs(coord1.y - coord2.y);
		return dx + dy;
	};

	return {
		start,
		stop,
		updateCoordinates,
	};
};
