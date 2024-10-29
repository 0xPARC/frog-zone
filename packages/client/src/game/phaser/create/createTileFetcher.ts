import { coordToKey } from "@smallbraingames/small-phaser";
import { fetchTiles } from "../../../utils/fetchTiles";
import { getSurroundingCoordinates } from "../../../utils/getSurroundingCoordinates";
import useStore, { Coord, TileWithCoord } from "../../store";
import { IS_MOCK } from "../../../const/env.const";

const FETCH_INTERVAL = IS_MOCK ? 1000 : 15000;
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

		const newTiles = await fetchTiles(initialCoordinate, nextBatch);

		onSuccessfulFetch({ tiles: newTiles, viewportCoords: coordinates });

		currentIndex += batchSize;

		if (currentIndex >= coordinates.length) {
			currentIndex = 0;
		}
	};

	const start = () => {
		if (intervalId === null) {
			fetchNextBatch();
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

		const priorityCoords = coords.slice(0, 5);
		const nonPriorityCoords = coords.slice(5);

		const staleCoordinates = nonPriorityCoords.filter((coord) => {
			const tile = grid.get(coordToKey(coord));
			return (
				tile &&
				(tile.fetchedAt === 0 || now - tile.fetchedAt > STALE_TIME_MS)
			);
		});

		const freshCoordinates = nonPriorityCoords.filter((coord) => {
			const tile = grid.get(coordToKey(coord));
			return tile && now - tile.fetchedAt <= STALE_TIME_MS;
		});

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

		return [...priorityCoords, ...staleCoordinates, ...freshCoordinates];
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
