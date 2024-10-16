import {
	createCamera,
	createInput,
	getSceneLoadPromise,
} from "@smallbraingames/small-phaser";
import createTilemap from "./createTilemap";
import config from "./phaserConfig";
import { PLAYER_CONFIG } from "../../../../player.config";
import { getPlayerId } from "../../../utils/getPlayerId";

const tileHeight = config.tilemap.tileHeight;
const setupMainScene = async (scene: Phaser.Scene) => {
	const playerId = Number(getPlayerId());
	await getSceneLoadPromise(scene);

	const tilemap = createTilemap(scene);
	const camera = createCamera(scene.cameras.main, 1, 1, 1, 1, 2);
	const x = PLAYER_CONFIG[playerId].x || 1;
	const y = PLAYER_CONFIG[playerId].y || 1;
	camera.centerOn(tileHeight * x, tileHeight * y);
	const input = createInput(scene.input);

	return {
		tilemap,
		camera,
		input,
	};
};

export default setupMainScene;
