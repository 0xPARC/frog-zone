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
			scene.load.image(phaserConfig.assetKeys.ice, "/assets/ICE.png");
			scene.load.image(phaserConfig.assetKeys.rock, "/assets/ROCK.png");
			scene.load.image(phaserConfig.assetKeys.sand, "/assets/SAND.png");
			scene.load.image(phaserConfig.assetKeys.water, "/assets/WATER.png");

			scene.load.image(phaserConfig.assetKeys.tileset, "/assets/tile.png");
			scene.load.image(phaserConfig.assetKeys.frog, "/assets/frog.png");
			scene.load.image(phaserConfig.assetKeys.item, "/assets/potion.png");
			scene.load.image(phaserConfig.assetKeys.arrow, "/assets/arrow.png");
			scene.load.image(phaserConfig.assetKeys.monster, "/assets/monster.png");
			scene.load.image(
				phaserConfig.assetKeys["frog blue"],
				"/assets/PLAYER_00_FROG_BLUE.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys["frog green"],
				"/assets/PLAYER_01_FROG_GREEN.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys["frog teal"],
				"/assets/PLAYER_02_FROG_TEAL.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys["frog pink"],
				"/assets/PLAYER_03_FROG_PINK.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys["blue potion"],
				"/assets/ITEM_00_BLUE_POTION.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys["red potion"],
				"/assets/ITEM_02_RED_POTION.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys["diamond sword"],
				"/assets/ITEM_05_DIAMOND_SWORD.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys["iron sword"],
				"/assets/ITEM_07_IRON_SWORD.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys["wooden sword"],
				"/assets/ITEM_09_WOODEN_SWORD.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys.dragon,
				"/assets/MONSTER_00_DRAGON.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys["prickly bush"],
				"/assets/MONSTER_01_PRICKLY_BUSH.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys.ogre,
				"/assets/MONSTER_10_HEAVY_ENEMY.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys.imp,
				"/assets/MONSTER_13_IMP.svg",
			);
			scene.load.image(
				phaserConfig.assetKeys.bat,
				"/assets/MONSTER_17_BAT.svg",
			);
			scene.load.audio(
				phaserConfig.assetKeys.sounds.background,
				"/assets/audio/background.m4a",
			);
			scene.load.audio(
				phaserConfig.assetKeys.sounds.click,
				"/assets/audio/click.wav",
			);
			scene.load.audio(
				phaserConfig.assetKeys.sounds.impact,
				"/assets/audio/impact.wav",
			);
			scene.load.audio(
				phaserConfig.assetKeys.sounds.powerup,
				"/assets/audio/powerup.wav",
			);
			scene.load.audio(
				phaserConfig.assetKeys.sounds.ready,
				"/assets/audio/ready.wav",
			);
			scene.load.audio(
				phaserConfig.assetKeys.sounds.success,
				"/assets/audio/success.m4a",
			);
		},
	});
	const config: Phaser.Types.Core.GameConfig = {
		type: Phaser.WEBGL,
		parent: "phaser-container",
		width: (window.innerWidth - 370) * window.devicePixelRatio,
		height: window.innerHeight * window.devicePixelRatio,
		backgroundColor: "#201e1e",
		pixelArt: true,
		scale: {
			mode: Phaser.Scale.CENTER_BOTH,
			zoom: 0.7 / window.devicePixelRatio,
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

	const playBackgroundMusic = () => {
		const sound = mainScene.sound.add(
			phaserConfig.assetKeys.sounds.background,
			{ loop: true },
		);
		sound.play();
	};

	return { phaserGame, mainScene, playBackgroundMusic, ...sceneContext };
};

export default createPhaserGame;
