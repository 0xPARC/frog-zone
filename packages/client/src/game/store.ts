export type Coord = { x: number; y: number };
import { enableMapSet } from "immer";
import { create } from "zustand";
enableMapSet();
import { coordToKey } from "@smallbraingames/small-phaser";
import { immer } from "zustand/middleware/immer";

export type Player = {
	id: number;
	hp: number;
	atk: number;
};

export type Item = {
	hp: number;
	atk: number;
};

type ItemEffect = {
	oldHp: number;
	oldAtk: number;
	newHp: number;
	newAtk: number;
};

interface State {
	players: Map<number, Player>;
	items: Map<number, Item>;

	addPlayer: (player: Player, coord: Coord) => void;
	addItem: (item: Item, coord: Coord) => void;

	movePlayer: (id: number, coord: Coord) => void;
	pickupItem: (playerId: number, coord: Coord, itemEffect: ItemEffect) => void;
}

const useStore = create<State>()(
	immer((set) => ({
		players: new Map(),
		items: new Map(),

		addPlayer: (player, coord) => {
			set((state) => {
				state.players.set(coordToKey(coord), player);
			});
		},
		addItem: (item, coord) => {
			set((state) => {
				state.items.set(coordToKey(coord), item);
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
	})),
);

export default useStore;
