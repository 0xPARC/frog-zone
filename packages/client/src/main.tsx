import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import createApi from "./game/createApi.ts";
import createPhaserGame from "./game/phaser/create/createPhaserGame.ts";
import syncPhaser from "./game/phaser/syncPhaser.ts";

const root = document.getElementById("root");
if (!root) {
	throw new Error("[main] root element not found");
}
const game = await createPhaserGame();
const api = createApi();
syncPhaser(game, api);

createRoot(root).render(<App />);
