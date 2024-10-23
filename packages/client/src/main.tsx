import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import createApi from "./game/createApi.ts";
import createPhaserGame from "./game/phaser/create/createPhaserGame.ts";
import syncPhaser from "./game/phaser/syncPhaser.ts";

const rootElement: HTMLElement | null = document.getElementById("root");

async function initGame() {
	const game = await createPhaserGame();
	const api = createApi();
	syncPhaser(game, api);

	if (!rootElement) {
		throw new Error("[main] root element not found");
	}
	createRoot(rootElement).render(<App />);
}

initGame().catch((error) => {
	console.error("[main] Failed to initialize the game:", error);
});
