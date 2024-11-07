import {
	getCenterPixelCoord,
	pixelCoordToTileCoord,
} from "@smallbraingames/small-phaser";
import { completedMoveAnimation } from "../../utils/animations";
import { fetchPlayer } from "../../utils/fetchPlayer";
import { getPlayerId } from "../../utils/getPlayerId";
import { type Api, Direction } from "../createApi";
import type { PhaserGame } from "../phaser/create/createPhaserGame";
import type { Coord, TileWithCoord } from "../store";
import useStore, {
	GameState,
	NEXT_MOVE_TIME_MILLIS,
} from "../store";
import { createTileFetcher } from "./create/createTileFetcher";
import phaserConfig from "./create/phaserConfig";
import { updatePlayer } from "../../utils/updatePlayer";
import { debounceTime } from "rxjs/internal/operators/debounceTime";
import ENTITIES_CONFIG from "../../const/entities.config";

const ENABLE_KEYBOARD_NAV = true;
const ARROW_ALPHA_WHILE_MOVE_UNAVAILABLE = 0.6;

type StoredImage = {
	image: Phaser.GameObjects.Image;
	coord: Coord;
};

type EntityId = number;

const syncPhaser = async (game: PhaserGame, api: Api) => {
	const players = new Map<EntityId, StoredImage>();
	let selectedPlayerImg: Phaser.GameObjects.Image | null = null;
	const monsters = new Map<EntityId, StoredImage>();
	const items = new Map<EntityId, StoredImage>();
	const selectedPlayerId = Number(getPlayerId());
	const player = await fetchPlayer(selectedPlayerId);
	const initialPlayerCoord = {
		x: player?.player_data?.loc.x,
		y: player?.player_data?.loc.y,
	};
	const tileWidth = phaserConfig.tilemap.tileWidth;
	const tileHeight = phaserConfig.tilemap.tileHeight;
	const addActionLog = useStore.getState().addActionLog;
	const directionArrows: Record<Direction, Phaser.GameObjects.Image | null> = {
		[Direction.DOWN]: null,
		[Direction.LEFT]: null,
		[Direction.RIGHT]: null,
		[Direction.UP]: null,
	};
	const isPreviousMovePending = false;

	const isMoveAvailable = () => {
		const lastMoveTime = useStore.getState().lastMoveTimeStamp;
		const now = Date.now();
		const canMove =
			!lastMoveTime || now - lastMoveTime >= NEXT_MOVE_TIME_MILLIS;
		return canMove && !isPreviousMovePending;
	};

	const destroyImageAtTileCoord = ({ tileCoord }: { tileCoord: Coord }) => {
		items.forEach((item, key) => {
			if (item.coord.x === tileCoord.x && item.coord.y === tileCoord.y) {
				item.image.destroy();
				items.delete(key);
				addActionLog({
					message: `item consumed: ${
						ENTITIES_CONFIG.items[key]?.name ?? "UNKNOWN"
					}`,
					color: "turquoise",
				});
			}
		});
		players.forEach((item, key) => {
			if (item.coord.x === tileCoord.x && item.coord.y === tileCoord.y) {
				item.image.destroy();
				players.delete(key);
			}
		});
		monsters.forEach((item, key) => {
			if (item.coord.x === tileCoord.x && item.coord.y === tileCoord.y) {
				item.image.destroy();
				monsters.delete(key);
			}
		});
	};

	const drawTiles = ({
		tiles,
		viewportCoords,
	}: {
		tiles: TileWithCoord[];
		viewportCoords: Coord[];
	}) => {
		useStore.getState().updateGrid(viewportCoords, tiles);
		const grid = useStore.getState().grid;

		grid.forEach((tile) => {
			if (tile.isShown) {
				game.tilemap.removeFogAt(tile.coord);
				if (!tile.fetchedAt) {
					game.tilemap.putFogAt(tile.coord, 0.3);
				}
				// if the tile was last fetched more than 3 seconds ago
				if (tile.fetchedAt && tile.fetchedAt < Date.now() - 2500) {
					game.tilemap.putFogAt(tile.coord, 0.1);
				}
			} else {
				game.tilemap.putFogAt(tile.coord, tile.fetchedAt ? 0.7 : 1);
				destroyImageAtTileCoord({ tileCoord: tile.coord });
			}
			if (tile.entity_type && tile.entity_id !== undefined) {
				if (tile.entity_type === "Item") {
					const id = tile.entity_id;
					const item = items.get(tile.entity_id);
					// check to see we didn't already add this item, in this specific spot
					if (
						!item ||
						(item.coord.x !== tile.coord.x &&
							item.coord.y !== tile.coord.y)
					) {
						// destroy the old item since its location changed
						if (item) {
							item.image.destroy();
						}
						const itemGameObject = addItem(
							tile.coord,
							ENTITIES_CONFIG.items[id]?.assetKey ??
								phaserConfig.assetKeys.item,
						);
						items.set(id, {
							image: itemGameObject,
							coord: tile.coord,
						});
					}
				}
				if (tile.entity_type === "Monster") {
					const id = tile.entity_id;
					const monster = monsters.get(tile.entity_id);
					// check to see we didn't already add this item, in this specific spot
					if (
						!monster ||
						(monster.coord.x !== tile.coord.x &&
							monster.coord.y !== tile.coord.y)
					) {
						// destroy the old monster since its location changed
						if (monster) {
							monster.image.destroy();
						}
						const monsterGameObject = addMonster(
							tile.coord,
							ENTITIES_CONFIG.monsters[id]?.assetKey ??
								phaserConfig.assetKeys.monster,
						);
						monsters.set(id, {
							image: monsterGameObject,
							coord: tile.coord,
						});
					}
				}
				if (tile.entity_type === "Player") {
					const id = tile.entity_id;
					destroyImageAtTileCoord({
						tileCoord: tile.coord,
					});
					// selectedPlayer is handled in drawSelectedPlayer for more control/ faster updates
					if (id !== selectedPlayerId) {
						const player = players.get(id);
						// check to see we didn't already add a player with this same id and in the same spot
						if (
							!player ||
							(player.coord.x !== tile.coord.x &&
								player.coord.y !== tile.coord.y)
						) {
							// destroy the old player, since their location has changed
							if (player) {
								player.image.destroy();
							}
							const playerGameObject = addPlayer({
								coord: tile.coord,
								assetKey:
									ENTITIES_CONFIG.players[id]?.assetKey ??
									phaserConfig.assetKeys.frog,
							});
							players.set(tile.entity_id, {
								image: playerGameObject,
								coord: tile.coord,
							});
						}
					}
					useStore.getState().addPlayer({
						id,
						hp: tile.hp,
						atk: tile.atk,
						coord: tile.coord,
					});
				}
				if (tile.entity_type === "None") {
					destroyImageAtTileCoord({
						tileCoord: tile.coord,
					});
				}
			}
		});

		// reveals the tiles only once drawTiles is called when the game start up
		const phaserContainer = document?.getElementById("phaser-container");
		if (phaserContainer && phaserContainer.style.visibility !== "visible") {
			phaserContainer.style.visibility = "visible";
		}
	};

	const tileFetcher = createTileFetcher({
		initialCoordinate: initialPlayerCoord,
		batchSize: 5,
		playerId: selectedPlayerId,
		onSuccessfulFetch: drawTiles,
	});

	const addPlayer = ({
		coord,
		showArrows,
		assetKey,
	}: {
		coord: Coord;
		showArrows?: boolean;
		assetKey: string;
	}): Phaser.GameObjects.Image => {
		const pixelCoord = getCenterPixelCoord(
			coord,
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		const go = game.mainScene.add.image(
			pixelCoord.x,
			pixelCoord.y,
			assetKey,
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
		if (showArrows) {
			drawArrowsAroundPlayer(go);
		}
		return go;
	};

	const positionCamera = (coord: Coord) => {
		const pixelCoord = getCenterPixelCoord(
			coord,
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		const x = pixelCoord.x;
		const y = pixelCoord.y;
		game.mainScene.cameras.main.pan(x, y);
	};

	const drawSelectedPlayer = (coord: Coord) => {
		if (selectedPlayerImg) {
			selectedPlayerImg?.destroy();
		}
		const playerGameObject = addPlayer({
			coord: coord,
			showArrows: true,
			assetKey: ENTITIES_CONFIG.players[selectedPlayerId].assetKey,
		});
		selectedPlayerImg = playerGameObject;
		game.camera.phaserCamera.startFollow(selectedPlayerImg);
	};

	const getNextPxCoord = (
		playerImg: Phaser.GameObjects.Image,
		direction: Direction,
		offset: number = tileWidth,
	) => {
		let newX = playerImg?.x;
		let newY = playerImg?.y;

		switch (direction) {
			case Direction.LEFT:
				newX -= offset;
				break;
			case Direction.RIGHT:
				newX += offset;
				break;
			case Direction.UP:
				newY -= offset;
				break;
			case Direction.DOWN:
				newY += offset;
				break;
		}
		return { x: newX, y: newY };
	};

	const isValidTile = (tileCoord: { x: number; y: number }) => {
		const key = `${tileCoord.x},${tileCoord.y}`;
		const grid = useStore.getState().grid;
		const tile = grid.get(key);
		if (tile?.terrainType) return ["GRASS", "ICE", "SAND"].includes(tile?.terrainType);
    return false;
	};

	const drawArrowsAroundPlayer = (playerImg: Phaser.GameObjects.Image) => {
		const rotation = {
			[Direction.LEFT]: Math.PI,
			[Direction.RIGHT]: 0,
			[Direction.UP]: (3 * Math.PI) / 2,
			[Direction.DOWN]: Math.PI / 2,
		};
		const directions = [
			Direction.LEFT,
			Direction.RIGHT,
			Direction.UP,
			Direction.DOWN,
		];
		directions.forEach((direction) => {
			if (directionArrows[direction]) {
				// clear old arrows
				directionArrows[direction].destroy();
				directionArrows[direction] = null;
			}
			const newPxCoord = getNextPxCoord(
				playerImg,
				direction,
				tileWidth * 0.72,
			);
			const arrow = game.mainScene.add
				.image(newPxCoord.x, newPxCoord.y, phaserConfig.assetKeys.arrow)
				.setInteractive();
			arrow.setAlpha(ARROW_ALPHA_WHILE_MOVE_UNAVAILABLE);
			arrow.setSize(tileWidth, tileHeight);
			arrow.setDisplaySize(tileWidth * 0.6, tileHeight * 0.6);
			arrow.setDepth(4);
			arrow.setRotation(rotation[direction]);
			arrow.on("pointerdown", () => {
				if (isMoveAvailable()) {
					handleMovePlayer(direction);
				}
			});
			directionArrows[direction] = arrow;
		});
	};

	const handleMovePlayer = async (direction: Direction) => {
		if (!selectedPlayerImg) return;

		// record a move was made
		useStore.getState().setLastMoveTimeStamp(Date.now());
		// stop the fetcher so we can show the pending move
		tileFetcher.stop();

		Object.keys(directionArrows).forEach((key) => {
			const d = key as Direction;
			if (directionArrows[d] && d !== direction) {
				directionArrows[d].setAlpha(ARROW_ALPHA_WHILE_MOVE_UNAVAILABLE);
			}
		});

		if (directionArrows[direction]) {
			directionArrows[direction].setTint(0xfeb437);
		}

		addActionLog({
			message: `move ${direction.toUpperCase()} requested`,
			color: "darkorange",
		});

		const moveResponse = await api.move(selectedPlayerId, direction);
		directionArrows[direction]?.clearTint();
		directionArrows[direction]?.setAlpha(
			ARROW_ALPHA_WHILE_MOVE_UNAVAILABLE,
		);
		if (moveResponse?.my_new_coords) {
			const x = moveResponse.my_new_coords.x;
			const y = moveResponse.my_new_coords.y;
			addActionLog({
				message: `move ${direction.toUpperCase()} received: ${JSON.stringify(
					{
						response: "success",
						x,
						y,
					},
				)}`,
				color: "limegreen",
			});
			const newCoord = {
				x,
				y,
			};
			positionCamera(newCoord);
			drawSelectedPlayer(newCoord);
			completedMoveAnimation(selectedPlayerImg);
			tileFetcher.updateCoordinates(newCoord);
			const publicKey = useStore.getState().publicKey as string;
			const gameId = useStore.getState().game?.gameId as string;
			updatePlayer({
				publicKey,
				score: Math.floor(Math.random() * 89), // TODO: implement real score, for now this is random between 0 - 88
				gameId,
			});
		} else {
			console.error("MOVE FAILED", {
				moveResponse,
				selectedPlayerId,
				direction,
			});
			addActionLog({
				message: `move ${direction.toUpperCase()} received: ${JSON.stringify(
					{
						response: "failure",
					},
				)}`,
				color: "red",
			});
		}
	};

	const addItem = (
		coord: Coord,
		assetKey: string,
	): Phaser.GameObjects.Image => {
		const pixelCoord = getCenterPixelCoord(
			coord,
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		const go = game.mainScene.add.image(
			pixelCoord.x,
			pixelCoord.y,
			assetKey,
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

	const addMonster = (
		coord: Coord,
		assetKey: string,
	): Phaser.GameObjects.Image => {
		const pixelCoord = getCenterPixelCoord(
			coord,
			phaserConfig.tilemap.tileWidth,
			phaserConfig.tilemap.tileHeight,
		);
		const go = game.mainScene.add.image(
			pixelCoord.x,
			pixelCoord.y,
			assetKey,
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

	const drawTerrain = () => {
		const grid = useStore.getState().grid;
		grid.forEach((tile) => {
      game.tilemap.putTileWithTerrainAt(tile.coord, tile.terrainType);
		});
	};

	game.mainScene.time.addEvent({
		delay: 200,
		loop: true,
		callback: () => {
			if (isMoveAvailable()) {
				Object.keys(directionArrows).forEach((key) => {
					const d = key as Direction;
					if (directionArrows[d]) {
						directionArrows[d].setAlpha(1);
					}
				});
			}
		},
	});

	const setupGame = () => {
		drawTerrain();
		drawSelectedPlayer(initialPlayerCoord);
		tileFetcher.start();

		if (ENABLE_KEYBOARD_NAV) {
			game.input.keyboard$.pipe(debounceTime(200)).subscribe((key) => {
				if (!isMoveAvailable()) {
					return;
				}

				// Handle directional input
				if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.LEFT) {
					handleMovePlayer(Direction.LEFT);
				} else if (
					key.keyCode === Phaser.Input.Keyboard.KeyCodes.RIGHT
				) {
					handleMovePlayer(Direction.RIGHT);
				} else if (key.keyCode === Phaser.Input.Keyboard.KeyCodes.UP) {
					handleMovePlayer(Direction.UP);
				} else if (
					key.keyCode === Phaser.Input.Keyboard.KeyCodes.DOWN
				) {
					handleMovePlayer(Direction.DOWN);
				}
			});
		}
		useStore.getState().setGameState(GameState.READY);
		addActionLog({
			message: "welcome to FROG ZONE",
			color: "limegreen",
		});
	};
	setupGame();
};

export default syncPhaser;
