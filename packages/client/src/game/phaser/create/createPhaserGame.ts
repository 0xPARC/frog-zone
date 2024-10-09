import {
	createPhaserScene,
	createPhaserGame as createSmallPhaserGame,
	resizePhaserGame,
} from "@smallbraingames/small-phaser";
import phaserConfig from "./phaserConfig";
import setupMainScene from "./setupScene";

const MAIN_SCENE_KEY = "MAIN";

export type PhaserGame = Awaited<ReturnType<typeof createPhaserGame>>;

const createPhaserGame = async () => {
	const MainScene = createPhaserScene({
		key: MAIN_SCENE_KEY,
		preload: (scene: Phaser.Scene) => {
			scene.load.image(phaserConfig.assetKeys.tileset, "/assets/tile.png");
			scene.load.image(phaserConfig.assetKeys.frog, "/assets/frog.png");
			scene.load.image(phaserConfig.assetKeys.item, "/assets/potion.png");
			scene.load.image(phaserConfig.assetKeys.arrow, "/assets/arrow.png");
		},
	});
	const config: Phaser.Types.Core.GameConfig = {
		type: Phaser.WEBGL,
		parent: "phaser-container",
		width: window.innerWidth * window.devicePixelRatio,
		height: window.innerHeight * window.devicePixelRatio,
		pixelArt: true,
		scale: {
			mode: Phaser.Scale.NONE,
			zoom: 1 / window.devicePixelRatio,
		},
		scene: [MainScene],
		dom: {
			createContainer: true,
		},
	};

	const phaserGame = await createSmallPhaserGame(config);
	resizePhaserGame(phaserGame.game);
	const mainScene = phaserGame.scenes[MAIN_SCENE_KEY];
	mainScene.load.setCORS("anonymous");
	mainScene.sound.unlock();
	mainScene.sound.pauseOnBlur = false;
	const sceneContext = await setupMainScene(phaserGame.scenes[MAIN_SCENE_KEY]);
	return { phaserGame, mainScene, ...sceneContext };
};

export default createPhaserGame;
