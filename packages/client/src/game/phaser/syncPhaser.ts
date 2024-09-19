import { coordToKey, getCenterPixelCoord } from "@smallbraingames/small-phaser";
import { debounce, debounceTime, distinct } from "rxjs";
import { type Api, Direction } from "../createApi";
import type { EventStream } from "../createEventStream";
import type { PhaserGame } from "../phaser/create/createPhaserGame";
import type { Coord } from "../store";
import phaserConfig from "./create/phaserConfig";

const syncPhaser = (eventStream$: EventStream, game: PhaserGame, api: Api) => {
	const players = new Map<number, Phaser.GameObjects.Image>();
	const items = new Map<number, Phaser.GameObjects.Image>();

	const selectedPlayerId = 1;

	const addPlayer = (coord: Coord): Phaser.GameObjects.Image => {
		const pixelCoord = getCenterPixelCoord(
			coord,
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		const go = game.mainScene.add.image(
			pixelCoord.x,
			pixelCoord.y,
			phaserConfig.assetKeys.frog,
		);
		go.setSize(phaserConfig.tilemap.tileWidth, phaserConfig.tilemap.tileHeight);
		go.setDisplaySize(
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		return go;
	};

	const addItem = (coord: Coord): Phaser.GameObjects.Image => {
		const pixelCoord = getCenterPixelCoord(
			coord,
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		const go = game.mainScene.add.image(
			pixelCoord.x,
			pixelCoord.y,
			phaserConfig.assetKeys.item,
		);
		go.setSize(phaserConfig.tilemap.tileWidth, phaserConfig.tilemap.tileHeight);
		go.setDisplaySize(
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		return go;
	};

	eventStream$.subscribe((event) => {
		const type = event[0];
		switch (type) {
			case "PlayerAdd": {
				const args = event[1];
				const [player, coord] = args;
				const playerGameObject = addPlayer(coord);
				players.set(player.id, playerGameObject);
				break;
			}
			case "PlayerMove": {
				const args = event[1];
				const [playerId, coord] = args;
				const player = players.get(playerId);
				if (player) {
					const pixelCoord = getCenterPixelCoord(
						coord,
						phaserConfig.tilemap.tileWidth,
						phaserConfig.tilemap.tileHeight,
					);
					player.setPosition(pixelCoord.x, pixelCoord.y);
				}
				break;
			}
			case "ItemAdd": {
				const args = event[1];
				const [, coord] = args;
				const itemGameObject = addItem(coord);
				const id = coordToKey(coord);
				items.set(id, itemGameObject);
				break;
			}
			case "ItemPickup": {
				const args = event[1];
				const [, , , , , , coord] = args;
				const item = items.get(coordToKey(coord));
				if (item) {
					item.destroy();
				}
				break;
			}
		}
	});

	game.input.keyboard$.pipe(debounceTime(500)).subscribe((key) => {
		if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.LEFT) {
			api.move(selectedPlayerId, Direction.LEFT);
		} else if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.RIGHT) {
			api.move(selectedPlayerId, Direction.RIGHT);
		} else if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.UP) {
			api.move(selectedPlayerId, Direction.UP);
		} else if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.DOWN) {
			api.move(selectedPlayerId, Direction.DOWN);
		}
	});
};

export default syncPhaser;
