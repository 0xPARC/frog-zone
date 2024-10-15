import { PlayerInfo } from "./game/components/PlayerInfo";
import { Login } from "./game/components/Login";
import { getPlayerId } from "./utils/getPlayerId";

function App() {
	const playerId = getPlayerId();
	return (
		<div>
			<Login />
			<PlayerInfo playerId={Number(playerId)} />
		</div>
	);
}

export default App;
