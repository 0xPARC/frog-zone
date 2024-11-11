import {
	createCamera,
	createInput,
	getSceneLoadPromise,
} from "@smallbraingames/small-phaser";
import { getPlayerId } from "../../../utils/getPlayerId";
import useStore from "../../store";
import createTilemap from "./createTilemap";
import config from "./phaserConfig";

const tileHeight = config.tilemap.tileHeight;
const setupMainScene = async (scene: Phaser.Scene) => {
	const playerId = Number(getPlayerId());
	const player = useStore.getState().getPlayerById(playerId);
	await getSceneLoadPromise(scene);

	const tilemap = createTilemap(scene);
	const camera = createCamera(scene.cameras.main, 0.6, 0.6, 1, 1, 2);
	camera.setZoom(0.6);
	const x = player?.coord.x || 1;
	const y = player?.coord.y || 1;
	camera.centerOn(tileHeight * x, tileHeight * y);
	const input = createInput(scene.input);

	return {
		tilemap,
		camera,
		input,
	};
};

export default setupMainScene;
