import { coordToKey, getCenterPixelCoord } from "@smallbraingames/small-phaser";
import { debounce, debounceTime, distinct } from "rxjs";
import { type Api, Direction } from "../createApi";
import type { EventStream } from "../createEventStream";
import type { PhaserGame } from "../phaser/create/createPhaserGame";
import type { Coord } from "../store";
import phaserConfig from "./create/phaserConfig";
import { getPlayerId } from "../../utils/getPlayerId";
import { completedMoveAnimation } from "../../utils/animations";

const syncPhaser = (eventStream$: EventStream, game: PhaserGame, api: Api) => {
	const players = new Map<number, Phaser.GameObjects.Image>();
	const items = new Map<number, Phaser.GameObjects.Image>();
	let moveMarker: Phaser.GameObjects.Image | null = null;

	const selectedPlayerId = Number(getPlayerId());

	const addPlayer = (coord: Coord): Phaser.GameObjects.Image => {
		const pixelCoord = getCenterPixelCoord(
			coord,
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight
		);
		const go = game.mainScene.add.image(
			pixelCoord.x,
			pixelCoord.y,
			phaserConfig.assetKeys.frog
		);
		go.setSize(phaserConfig.tilemap.tileWidth, phaserConfig.tilemap.tileHeight);
		go.setDisplaySize(
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight
		);
		go.setDepth(1);
		return go;
	};

	const handleMovePlayer = async (direction: Direction) => {
		const selectedPlayer = players.get(selectedPlayerId);
		if (!selectedPlayer) return;

		const tileWidth = phaserConfig.tilemap.tileWidth;
		const tileHeight = phaserConfig.tilemap.tileHeight;

		let newX = selectedPlayer.x;
		let newY = selectedPlayer.y;

		switch (direction) {
			case Direction.LEFT:
				newX -= tileWidth;
				break;
			case Direction.RIGHT:
				newX += tileWidth;
				break;
			case Direction.UP:
				newY -= tileHeight;
				break;
			case Direction.DOWN:
				newY += tileHeight;
				break;
		}

		// Add the marker at the new position
		const nextMoveMarker = game.mainScene.add.image(
			newX,
			newY,
			phaserConfig.assetKeys.arrow
		);
		nextMoveMarker.setSize(tileWidth, tileHeight);
		nextMoveMarker.setDisplaySize(tileWidth * 0.7, tileHeight * 0.7);
		nextMoveMarker.setDepth(0);

		const rotation = {
			[Direction.LEFT]: Math.PI,
			[Direction.RIGHT]: 0,
			[Direction.UP]: (3 * Math.PI) / 2,
			[Direction.DOWN]: Math.PI / 2,
		};

		nextMoveMarker.setRotation(rotation[direction]);
		moveMarker = nextMoveMarker;

		await api.move(selectedPlayerId, direction);
	};

	const addItem = (coord: Coord): Phaser.GameObjects.Image => {
		const pixelCoord = getCenterPixelCoord(
			coord,
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight
		);
		const go = game.mainScene.add.image(
			pixelCoord.x,
			pixelCoord.y,
			phaserConfig.assetKeys.item
		);
		go.setSize(phaserConfig.tilemap.tileWidth, phaserConfig.tilemap.tileHeight);
		go.setDisplaySize(
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight
		);
		return go;
	};

	eventStream$.subscribe((event) => {
		const type = event[0];
		switch (type) {
			case "PlayerAdd": {
				const args = event[1];
				const [player, coord] = args;
				console.log("player add", coord);
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
						phaserConfig.tilemap.tileHeight
					);
					const x = pixelCoord.x;
					const y = pixelCoord.y;
					game.mainScene.tweens.add({
						targets: player,
						x,
						y,
						duration: 200,
						ease: "Power2",
						onComplete: () => {
							if (playerId === selectedPlayerId) {
								game.mainScene.cameras.main.centerOn(x, y);
								completedMoveAnimation(player);
								if (moveMarker) {
									moveMarker.destroy();
									moveMarker = null;
								}
							}
						},
					});
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

	game.input.keyboard$.pipe(debounceTime(500)).subscribe(async (key) => {
		if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.LEFT) {
			console.log("players left", players.get(selectedPlayerId));
			handleMovePlayer(Direction.LEFT);
		} else if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.RIGHT) {
			handleMovePlayer(Direction.RIGHT);
		} else if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.UP) {
			handleMovePlayer(Direction.UP);
		} else if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.DOWN) {
			handleMovePlayer(Direction.DOWN);
		}
	});
};

export default syncPhaser;
