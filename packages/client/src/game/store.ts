export type Coord = { x: number; y: number };
import { enableMapSet } from "immer";
import { create } from "zustand";
enableMapSet();
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
	isBorderingLand: boolean;
	coord: Coord;
	isShown?: boolean;
	fetchedAt: number;
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
	color: string; // hex color of the message
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
	grid: Map<string, TileWithCoord>; // Update to use string keys "x,y"
	lastMoveTimeStamp: number;
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
	const grid = new Map<string, TileWithCoord>();
	const waterCoordinatesBorderingLand = findBorderingWaterCoordinates(
		config as Grid,
	);

	for (let x = 0; x < size; x++) {
		for (let y = 0; y < size; y++) {
			const key = `${x},${y}`; // Use "x,y" as the key format
			const tileConfigKey = `${x},${y}`;
			const tileConfig = config[tileConfigKey] || {};

			grid.set(key, {
				coord: { x, y },
				terrainType: tileConfig.terrainType as TerrainType,
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
		lastMoveTimeStamp: 0,
		actionLogs: [],
		hoverTile: null,
		setHoverTile: (coord: Coord | null) => {
			if (coord) {
				const gridItem = get().grid.get(`${coord.x},${coord.y}`);
				set({ hoverTile: gridItem });
			} else {
				set({ hoverTile: null });
			}
		},
		setIsLoggedIn: ({ isLoggedIn, publicKey }) => {
			set({ isLoggedIn, publicKey });
		},
		setGame: (game) => set({ game }),
		addPlayer: (player) => {
			set((state) => {
				state.players.set(
					`${player.coord.x},${player.coord.y}`,
					player,
				);
			});
		},
		addItem: (item, coord) => {
			set((state) => {
				state.items.set(`${coord.x},${coord.y}`, item);
			});
		},
		addMonster: (monster, coord) => {
			set((state) => {
				state.monsters.set(`${coord.x},${coord.y}`, monster);
			});
		},
		movePlayer: (id, coord) => {
			set((state) => {
				const player = state.players.get(id);
				if (player) {
					state.players.set(`${coord.x},${coord.y}`, player);
				}
			});
		},
		pickupItem: (playerId, coord, itemEffect) => {
			set((state) => {
				const key = `${coord.x},${coord.y}`;
				const player = state.players.get(playerId);
				const item = state.items.get(key);
				if (player && item) {
					state.players.set(playerId, { ...player, ...itemEffect });
					state.items.delete(key);
				} else {
					console.error("[pickupItem] player or item not found");
				}
			});
		},
		setGameState: (state) => set({ gameState: state }),
		setLastMoveTimeStamp: (time) => set({ lastMoveTimeStamp: time }),
		getPlayerById: (id) => {
			const players = get().players;
			return (
				Array.from(players.values()).find(
					(player) => player.id === id,
				) || null
			);
		},
		updateGrid: (viewportCoords, newTiles) => {
			set((state) => {
				const newGrid = new Map(state.grid);

				const viewportKeys = new Set(
					viewportCoords.map((coord) => `${coord.x},${coord.y}`),
				);

				newGrid.forEach((value, key) => {
					if (viewportKeys.has(key)) {
						newGrid.set(key, { ...value, isShown: true });
					} else {
						newGrid.set(key, {
							...value,
							entity_type: "None",
							isShown: false,
						});
					}
				});

				newTiles.forEach((tile) => {
					const key = `${tile.coord.x},${tile.coord.y}`;
					if (newGrid.has(key)) {
						newGrid.set(key, {
							...newGrid.get(key),
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
