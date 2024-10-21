import {
	createCamera,
	createInput,
	getSceneLoadPromise,
} from "@smallbraingames/small-phaser";
import createTilemap from "./createTilemap";
import config from "./phaserConfig";
import { getPlayerId } from "../../../utils/getPlayerId";
import useStore from "../../store";

const tileHeight = config.tilemap.tileHeight;
const setupMainScene = async (scene: Phaser.Scene) => {
	const playerId = Number(getPlayerId());
	const player = useStore.getState().getPlayerById(playerId);
	await getSceneLoadPromise(scene);

	const tilemap = createTilemap(scene);
	const camera = createCamera(scene.cameras.main, 1, 1, 1, 1, 2);
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
