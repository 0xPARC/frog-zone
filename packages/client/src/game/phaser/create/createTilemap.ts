import { createTilemap as createPhaserTilemap } from "@smallbraingames/small-phaser";
import type { Coord } from "../../store";
import config from "./phaserConfig";

const createTilemap = (scene: Phaser.Scene) => {
	const {
		tilemap: { tileWidth, tileHeight, gridSize },
		assetKeys: { tileset: tilesetAssetKey },
	} = config;
	const tilemap = createPhaserTilemap(scene, tileWidth, tileHeight, gridSize);

	const tileset = tilemap.addTilesetImage(
		tilesetAssetKey,
		tilesetAssetKey,
		tileWidth,
		tileHeight,
	);
	if (!tileset) {
		throw Error("[createTilemap] tileset is null");
	}
	const startX = -gridSize / 2;
	const startY = startX;
	const layer = tilemap.createBlankLayer(
		tilesetAssetKey,
		tileset,
		startX * tileWidth,
		startY * tileHeight,
		gridSize,
		gridSize,
	);

	if (!layer) {
		throw Error("[createTilemap] layer is null");
	}

	const putTileAt = (tile: number, tileCoord: Coord) => {
		layer.putTileAt(
			tile,
			tileCoord.x + gridSize / 2,
			tileCoord.y + gridSize / 2,
		);
	};

	const removeTileAt = (tileCoord: Coord) => {
		layer.removeTileAt(
			tileCoord.x + gridSize / 2,
			tileCoord.y + gridSize / 2,
		);
	};

	const getTileAt = (tileCoord: Coord) => {
		return layer.getTileAt(
			tileCoord.x + gridSize / 2,
			tileCoord.y + gridSize / 2,
		);
	};

	// Create a fog map to store the fog overlays for each tile
	const fogMap: { [key: string]: Phaser.GameObjects.Graphics } = {};

	const putFogAt = (tileCoord: Coord, opacity: number = 0.5) => {
		const tileX =
			(tileCoord.x + gridSize / 2) * tileWidth + startX * tileWidth;
		const tileY =
			(tileCoord.y + gridSize / 2) * tileHeight + startY * tileHeight;

		const key = `${tileCoord.x},${tileCoord.y}`;
		// if fog already exists, remove it
		if (fogMap[key]) {
			removeFogAt(tileCoord);
		}
		const fogOverlay = scene.add.graphics();
		fogOverlay.fillStyle(0xffffff, opacity);
		fogOverlay.fillRect(tileX, tileY, tileWidth, tileHeight);
		fogMap[key] = fogOverlay;
	};

	const removeFogAt = (tileCoord: Coord) => {
		const key = `${tileCoord.x},${tileCoord.y}`;
		const fogOverlay = fogMap[key];
		if (fogOverlay) {
			fogOverlay.destroy(); // Destroy the fog graphics for this tile
			delete fogMap[key]; // Remove the entry from the map
		}
	};

	for (let x = 0; x < gridSize; x++) {
		for (let y = 0; y < gridSize; y++) {
			putTileAt(0, { x, y });
		}
	}

	return {
		tilemap,
		layer,
		putTileAt,
		removeTileAt,
		getTileAt,
		putFogAt,
		removeFogAt,
		...config.tilemap,
	};
};

export default createTilemap;
