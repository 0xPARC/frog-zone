import { PlayerInfo } from "./game/components/PlayerInfo";
import { Login } from "./game/components/Login";
import { getPlayerId } from "./utils/getPlayerId";
import { MoveCountdownTimer } from "./game/components/MoveCountdown";
import WinGameButton from "./game/components/WinGameButton";
import useStore from "./game/store";
import { GameFinishedOverlay } from "./game/components/GameFinishedOverlay";
import { WaitingForPlayersOverlay } from "./game/components/WaitingForPlayersOverlay";

const MIN_PLAYERS = 4;
const MIN_PLAYERS_TO_FORCE_START = 1;
function App() {
	const playerId = getPlayerId();
	const gameId = useStore((state) => state.game?.gameId);
	const gameStatus = useStore((state) => state.game?.status);
	const gameMachines = useStore((state) => state.game?.machines);
	const forceStart = useStore((state) => state.forceStart);
	return (
		<div>
			<Login />
			<PlayerInfo playerId={Number(playerId)} />
			<MoveCountdownTimer />
			<div
				style={{
					position: "absolute",
					top: "10px",
					right: "10px",
					minWidth: "200px",
				}}
			>
				{gameId && gameStatus === "ongoing" && (
					<WinGameButton gameId={gameId} />
				)}
			</div>
			{gameId &&
				gameStatus === "ongoing" &&
				gameMachines &&
				gameMachines?.length < MIN_PLAYERS &&
				!forceStart && (
					<WaitingForPlayersOverlay
						allowForceStart={
							gameMachines.length >= MIN_PLAYERS_TO_FORCE_START
						}
					/>
				)}
			{gameId && gameStatus === "completed" && <GameFinishedOverlay />}
		</div>
	);
}

export default App;
