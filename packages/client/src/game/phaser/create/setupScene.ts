import {
	createCamera,
	createInput,
	getSceneLoadPromise,
} from "@smallbraingames/small-phaser";
import createTilemap from "./createTilemap";
import config from "./phaserConfig";

const tileHeight = config.tilemap.tileHeight;
const setupMainScene = async (scene: Phaser.Scene) => {
	await getSceneLoadPromise(scene);

	const tilemap = createTilemap(scene);
	const camera = createCamera(scene.cameras.main, 1, 1, 1, 1, 2);
	camera.centerOn(tileHeight, tileHeight);
	const input = createInput(scene.input);

	return {
		tilemap,
		camera,
		input,
	};
};

export default setupMainScene;
