import { PLAYER_CONFIG } from "./../../../player.config";
import { coordToKey, getCenterPixelCoord } from "@smallbraingames/small-phaser";
import { debounce, debounceTime, distinct } from "rxjs";
import { type Api, Direction } from "../createApi";
import type { PhaserGame } from "../phaser/create/createPhaserGame";
import type { Coord } from "../store";
import phaserConfig from "./create/phaserConfig";
import { getPlayerId } from "../../utils/getPlayerId";
import { completedMoveAnimation } from "../../utils/animations";
import { getTilesAroundPlayer } from "../../utils/getTilesAroundPlayer";
import useStore from "../store";

const MOVE_DEBOUNCE_TIME = 1000;

const syncPhaser = async (game: PhaserGame, api: Api) => {
	const players = new Map<number, Phaser.GameObjects.Image>();
	const items = new Map<number, Phaser.GameObjects.Image>();
	let moveMarker: Phaser.GameObjects.Image | null = null;

	const selectedPlayerId = Number(getPlayerId());

	const drawTilesAroundPlayer = async ({ coord }: { coord: Coord }) => {
		const tiles = await getTilesAroundPlayer({
			playerId: selectedPlayerId,
			coord,
		});

		tiles.forEach((tile) => {
			game.tilemap.removeFogAt(tile.coord);
			if (tile.entity_type.val === "Item") {
				const itemGameObject = addItem(tile.coord);
				items.set(tile.entity_id.val, itemGameObject);
			}
			if (tile.entity_type.val === "Player") {
				const id = tile.entity_id.val;
				const playerGameObject = addPlayer({
					playerId: id,
					coord: tile.coord,
				});
				players.set(id, playerGameObject);
				useStore.getState().addPlayer({
					id,
					hp: tile.hp.val,
					atk: tile.atk.val,
					coord: tile.coord,
				});
			}
			if (tile.entity_type.val === "None") {
				// remove image at coord
				const id = coordToKey(tile.coord);
				const image = players.get(id) || items.get(id);
				if (image) {
					image.destroy();
				}
			}
		});
	};

	const addPlayer = ({
		playerId,
		coord,
	}: {
		playerId: number;
		coord: Coord;
	}): Phaser.GameObjects.Image => {
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
		go.setSize(
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		go.setDisplaySize(
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		go.setDepth(1);
		// if (selectedPlayerId === playerId) {
		// 	// Define the triangle's points
		// 	const triangleSize = 10; // Adjust this size as needed
		// 	const triangleX = go.x;
		// 	const triangleY = 25; // Position above the image

		// 	// Add the triangle above the image
		// 	const triangle = game.mainScene.add.triangle(
		// 		triangleX,
		// 		triangleY,
		// 		0,
		// 		triangleSize, // Point 1 (top)
		// 		-triangleSize,
		// 		-triangleSize, // Point 2 (bottom left)
		// 		triangleSize,
		// 		-triangleSize, // Point 3 (bottom right)
		// 		0xffd700, // Yellow color in hex
		// 	);

		// 	// Set the origin to center the triangle
		// 	triangle.setDepth(2);
		// 	triangle.setOrigin(0.5, 0.5);
		// }
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
			phaserConfig.assetKeys.arrow,
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

		const moveResponse = await api.move(selectedPlayerId, direction);
		if (moveResponse?.my_new_coords) {
			const newCoord = {
				x: moveResponse.my_new_coords.x.val,
				y: moveResponse.my_new_coords.y.val,
			};
			drawTilesAroundPlayer({
				coord: newCoord,
			});
			if (selectedPlayer) {
				const pixelCoord = getCenterPixelCoord(
					newCoord,
					phaserConfig.tilemap.tileWidth,
					phaserConfig.tilemap.tileHeight,
				);
				const x = pixelCoord.x;
				const y = pixelCoord.y;
				game.mainScene.tweens.add({
					targets: selectedPlayer,
					x,
					y,
					duration: 200,
					ease: "Power2",
					onComplete: () => {
						game.mainScene.cameras.main.centerOn(x, y);
						completedMoveAnimation(selectedPlayer);
						if (moveMarker) {
							moveMarker.destroy();
							moveMarker = null;
						}
					},
				});
			}
		}
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
		go.setSize(
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		go.setDisplaySize(
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		return go;
	};

	// draw initial tiles around player
	drawTilesAroundPlayer({ coord: PLAYER_CONFIG[selectedPlayerId] });

	game.input.keyboard$
		.pipe(debounceTime(MOVE_DEBOUNCE_TIME))
		.subscribe(async (key) => {
			if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.LEFT) {
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
