export type Coord = { x: number; y: number };
import { enableMapSet } from "immer";
import { create } from "zustand";
enableMapSet();
import { coordToKey } from "@smallbraingames/small-phaser";
import { immer } from "zustand/middleware/immer";
import { GameResponse } from "../utils/fetchGame";
import tileConfig from "../const/tile.config.json";
import {
	findBorderingWaterCoordinates,
	Grid,
} from "../utils/findBorderingWaterCoordinates";

export type TileEntityType = "None" | "Player" | "Item" | "Monster";

export type Tile = {
	atk: number;
	entity_id: number;
	entity_type: TileEntityType;
	hp: number;
};

export enum TerrainType {
	LAND = "LAND",
	WATER = "WATER",
}

export type TileWithCoord = Tile & {
	terrainType: TerrainType;
	isBorderingLand: boolean; // if its a water tile that is bordering land
	coord: Coord; // phaser coordinate of the tile
	isShown?: boolean; // if the tile is shown in the game
	fetchedAt: number; // timestamp of when the tile was fetched
};

export type Player = {
	id: number;
	hp: number;
	atk: number;
	coord: Coord;
};

export type Item = {
	hp: number;
	atk: number;
};

export type Monster = {
	hp: number;
	atk: number;
};

type ItemEffect = {
	oldHp: number;
	oldAtk: number;
	newHp: number;
	newAtk: number;
};

export enum GameState {
	READY = "ready",
	LOADING = "loading",
}

export type Game = GameResponse["game"];

export type ActionLog = {
	message: string;
};

export type ActionLogs = ActionLog[] | [];

export const NEXT_MOVE_TIME_MILLIS = 3500;

interface State {
	gameState: GameState;
	isLoggedIn: boolean | null;
	publicKey: string | null;
	game: Game | null;
	players: Map<number, Player>;
	items: Map<number, Item>;
  monsters: Map<number, Monster>;
	grid: Map<number, TileWithCoord>;
	lastMoveTimeStamp: number; // timestamp for next move
	actionLogs: ActionLogs;
	hoverTile: TileWithCoord | null;
	setHoverTile: (coord: Coord | null) => void;
	setIsLoggedIn: (s: {
		isLoggedIn: boolean | null;
		publicKey: string | null;
	}) => void;
	setGame: (game: Game | null) => void;

	addPlayer: (player: Player) => void;
	addItem: (item: Item, coord: Coord) => void;
	addMonster: (item: Monster, coord: Coord) => void;

	movePlayer: (id: number, coord: Coord) => void;
	pickupItem: (
		playerId: number,
		coord: Coord,
		itemEffect: ItemEffect,
	) => void;
	setGameState: (state: GameState) => void;
	setLastMoveTimeStamp: (time: number) => void;
	getPlayerById: (id: number) => Player | null;
	updateGrid: (viewportCoords: Coord[], newTiles: TileWithCoord[]) => void;
	addActionLog: (log: ActionLog) => void;
}

const initializeGrid = (
	size: number,
	config: Record<string, { terrainType: string }>,
) => {
	const grid = new Map();
	const waterCoordinatesBorderingLand = findBorderingWaterCoordinates(
		config as Grid,
	);
	for (let x = 0; x < size; x++) {
		for (let y = 0; y < size; y++) {
			const coordKey = coordToKey({ x, y });
			const tileConfigKey = `${x},${y}`;
			const tileConfig = config[tileConfigKey] || {};

			grid.set(coordKey, {
				coord: { x, y },
				terrainType: tileConfig.terrainType,
				isBorderingLand:
					waterCoordinatesBorderingLand.includes(tileConfigKey),
				entity_type: "None",
				fetchedAt: 0,
				isShown: false,
			});
		}
	}
	return grid;
};

const useStore = create<State>()(
	immer((set, get) => ({
		isLoggedIn: null,
		publicKey: null,
		game: null,
		gameState: GameState.LOADING,
		players: new Map<number, Player>(),
		items: new Map<number, Item>(),
		monsters: new Map<number, Monster>(),
		grid: initializeGrid(64, tileConfig),
		lastMoveTimeStamp: 0, // Store the last move timestamp
		actionLogs: [],
		hoverTile: null,
		setHoverTile: (coord: Coord | null) => {
			if (coord) {
				const gridItem = get().grid.get(coordToKey(coord));
				set({ hoverTile: gridItem });
			} else {
				set({ hoverTile: null });
			}
		},
		setIsLoggedIn: ({
			isLoggedIn,
			publicKey,
		}: {
			isLoggedIn: boolean | null;
			publicKey: string | null;
		}) => {
			set({ isLoggedIn });
			set({ publicKey });
		},

		setGame: (game: Game | null) => {
			set({ game });
		},

		addPlayer: (player) => {
			set((state) => {
				state.players.set(coordToKey(player.coord), player);
			});
		},
		addItem: (item, coord) => {
			set((state) => {
				state.items.set(coordToKey(coord), item);
			});
		},
		addMonster: (monster, coord) => {
			set((state) => {
				state.items.set(coordToKey(coord), monster);
			});
		},

		movePlayer: (id, coord) => {
			set((state) => {
				const player = state.players.get(id);
				if (player) {
					state.players.set(coordToKey(coord), player);
				}
			});
		},
		pickupItem: (playerId, coord, itemEffect) => {
			set((state) => {
				const coordKey = coordToKey(coord);
				const player = state.players.get(coordKey);
				const item = state.items.get(coordKey);
				if (player && item) {
					state.players.set(playerId, { ...player, ...itemEffect });
					state.items.delete(coordKey);
				} else {
					console.error("[pickupItem] player or item not found");
				}
			});
		},
		setGameState: (state: GameState) => {
			set({ gameState: state });
		},
		setLastMoveTimeStamp: (time: number) =>
			set({ lastMoveTimeStamp: time }),
		getPlayerById: (id: number) => {
			const players = get().players;
			let player: Player | null = null;

			players.forEach((value) => {
				if (value.id === id) {
					player = value;
				}
			});

			return player as Player | null;
		},
		updateGrid: (viewportCoords, newTiles) => {
			set((state) => {
				const newGrid = new Map(state.grid);

				const viewportCoordKeys = new Set(
					viewportCoords.map(coordToKey),
				);

				// Update the grid
				newGrid.forEach((value, key) => {
					// Check if the tile is in the viewport
					if (viewportCoordKeys.has(key)) {
						// Set isShown to true for tiles in the viewport
						newGrid.set(key, {
							...value,
							isShown: true,
						});
					} else {
						// Set isShown to false for tiles outside the viewport
						newGrid.set(key, {
							...value,
							entity_type: "None",
							isShown: false,
						});
					}
				});

				newTiles.forEach((tile) => {
					const coordKey = coordToKey(tile.coord);

					// Update the grid with the newly fetched tile value (overrides the isShown: false set above)
					if (newGrid.has(coordKey)) {
						const existingTile = newGrid.get(coordKey);
						newGrid.set(coordKey, {
							...existingTile,
							...tile,
							isShown: true,
						});
					}
				});

				state.grid = newGrid;
			});
		},
		addActionLog: (log) => {
			set((state) => {
				state.actionLogs.push(log);
			});
		},
	})),
);

export default useStore;
