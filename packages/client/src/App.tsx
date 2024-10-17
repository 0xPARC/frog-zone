import { PlayerInfo } from "./game/components/PlayerInfo";
import { Login } from "./game/components/Login";
import { getPlayerId } from "./utils/getPlayerId";
import { MoveCountdownTimer } from "./game/components/MoveCountdown";

function App() {
	const playerId = getPlayerId();
	return (
		<div>
			<Login />
			<PlayerInfo playerId={Number(playerId)} />
			<MoveCountdownTimer />
		</div>
	);
}

export default App;
