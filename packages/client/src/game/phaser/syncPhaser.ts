import {
	coordToKey,
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
	TerrainType,
} from "../store";
import { createTileFetcher } from "./create/createTileFetcher";
import phaserConfig from "./create/phaserConfig";
import { updatePlayer } from "../../utils/updatePlayer";
import { debounceTime } from "rxjs/internal/operators/debounceTime";

const syncPhaser = async (game: PhaserGame, api: Api) => {
	const players = new Map<number, Phaser.GameObjects.Image>();
	let selectedPlayerImg: Phaser.GameObjects.Image | null = null;
	const items = new Map<number, Phaser.GameObjects.Image>();
	const selectedPlayerId = Number(getPlayerId());
	const player = await fetchPlayer(selectedPlayerId);
	const initialPlayerCoord = {
		x: player?.player_data?.loc.x,
		y: player?.player_data?.loc.y,
	};
	let moveMarker: Phaser.GameObjects.Image | null = null;
	const tileWidth = phaserConfig.tilemap.tileWidth;
	const tileHeight = phaserConfig.tilemap.tileHeight;

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
				const id = coordToKey(tile.coord);
				const image = players.get(id) || items.get(id);
				if (image) {
					image.destroy();
				}
			}
			if (tile.entity_type && tile.entity_id !== undefined) {
				if (tile.entity_type === "Item") {
					const itemGameObject = addItem(tile.coord);
					items.set(tile.entity_id, itemGameObject);
				}
				if (tile.entity_type === "Player") {
					const id = tile.entity_id;
					if (id !== selectedPlayerId) {
						const playerImg = players.get(id);
						if (playerImg) {
							playerImg.destroy();
						}
						const playerGameObject = addPlayer({
							playerId: id,
							coord: tile.coord,
						});
						players.set(id, playerGameObject);
					}
					useStore.getState().addPlayer({
						id,
						hp: tile.hp,
						atk: tile.atk,
						coord: tile.coord,
					});
				}
				if (tile.entity_type === "None") {
					// remove image at coord
					const id = coordToKey(tile.coord);
					const image = players.get(id) || items.get(id);
					if (image) {
						image.destroy();
					}
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
		game.mainScene.cameras.main.centerOn(x, y);
	};

	const drawSelectedPlayer = (coord: Coord) => {
		if (selectedPlayerImg) {
			selectedPlayerImg?.destroy();
		}
		const playerGameObject = addPlayer({
			playerId: selectedPlayerId,
			coord: coord,
		});
		selectedPlayerImg = playerGameObject;
		game.camera.phaserCamera.startFollow(selectedPlayerImg);
	};

	const getNextPxCoord = (
		playerImg: Phaser.GameObjects.Image,
		direction: Direction,
	) => {
		let newX = playerImg?.x;
		let newY = playerImg?.y;

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
		return { x: newX, y: newY };
	};

	const isValidTile = (tileCoord: { x: number; y: number }) => {
		const key = coordToKey(tileCoord);
		const grid = useStore.getState().grid;
		const tile = grid.get(key);
		return tile?.terrainType === TerrainType.LAND;
	};

	const handleMovePlayer = async (direction: Direction) => {
		if (!selectedPlayerImg) return;

		const newPxCoord = getNextPxCoord(selectedPlayerImg, direction);
		// prevent the user from moving to an invalid tile, like into water
		if (
			!isValidTile(
				pixelCoordToTileCoord(newPxCoord, tileWidth, tileHeight),
			)
		) {
			return;
		}

		// record a move was made
		useStore.getState().setLastMoveTimeStamp(Date.now());
		// stop the fetcher so we can show the pending move
		tileFetcher.stop();

		// Add the marker at the new position
		const nextMoveMarker = game.mainScene.add.image(
			newPxCoord.x,
			newPxCoord.y,
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
				x: moveResponse.my_new_coords.x,
				y: moveResponse.my_new_coords.y,
			};
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
		}
		if (moveMarker) {
			moveMarker.destroy();
			moveMarker = null;
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

	const drawTerrain = () => {
		const grid = useStore.getState().grid;
		grid.forEach((tile) => {
			if (tile.terrainType === TerrainType.LAND) {
				game.tilemap.putLandAt(tile.coord);
			}
			if (tile.terrainType === TerrainType.WATER) {
				if (tile.isBorderingLand) {
					game.tilemap.putShallowWaterAt(tile.coord);
				} else {
					game.tilemap.putWaterAt(tile.coord);
				}
			}
		});
	};

	const setupGame = () => {
		drawTerrain();
		drawSelectedPlayer(initialPlayerCoord);
		tileFetcher.start();

		game.input.keyboard$.pipe(debounceTime(200)).subscribe((key) => {
			const lastMoveTime = useStore.getState().lastMoveTimeStamp;
			const now = Date.now();
			const canMove =
				!lastMoveTime || now - lastMoveTime >= NEXT_MOVE_TIME_MILLIS;
			const isPreviousMovePending = Boolean(moveMarker);

			if (!canMove || isPreviousMovePending) {
				return;
			}

			// Handle directional input
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
		useStore.getState().setGameState(GameState.READY);
	};
	setupGame();
};

export default syncPhaser;
