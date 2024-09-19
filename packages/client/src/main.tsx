import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import createApi from "./game/createApi.ts";
import createEventStream from "./game/createEventStream.ts";
import createPhaserGame from "./game/phaser/create/createPhaserGame.ts";
import syncPhaser from "./game/phaser/syncPhaser.ts";
import syncStore from "./game/syncStore.ts";

const root = document.getElementById("root");
if (!root) {
	throw new Error("[main] root element not found");
}
const game = await createPhaserGame();
const eventStream$ = createEventStream();
const api = createApi();
syncPhaser(eventStream$, game, api);
syncStore(eventStream$);

createRoot(root).render(<App />);
