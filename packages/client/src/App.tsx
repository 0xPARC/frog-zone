import { GameFinishedOverlay } from "./game/components/GameFinishedOverlay";
import { Login } from "./game/components/Login";
import { MoveCountdownTimer } from "./game/components/MoveCountdown";
import { PlayerInfo } from "./game/components/PlayerInfo";
import { WaitingForPlayersOverlay } from "./game/components/WaitingForPlayersOverlay";
import WinGameButton from "./game/components/WinGameButton";
import useStore from "./game/store";
import { getPlayerId } from "./utils/getPlayerId";

import { AreYouThere } from "./game/components/AreYouThere";
import { EnterGameAnimation } from "./game/components/EnterGameAnimation";

const MIN_PLAYERS = 4;
const MIN_PLAYERS_TO_FORCE_START = 1;
function App() {
	const playerId = getPlayerId();
	const gameId = useStore((state) => state.game?.gameId);
	const gameStatus = useStore((state) => state.game?.status);
	const gameMachines = useStore((state) => state.game?.machines);
	const isLoggedIn = useStore((state) => state.isLoggedIn);
	return (
		<div>
			<Login />
			{isLoggedIn && (
				<>
					<PlayerInfo playerId={Number(playerId)} />
					<MoveCountdownTimer />

					{gameId && gameStatus === "ongoing" && (
						<>
							<EnterGameAnimation />
							<div
								style={{
									position: "absolute",
									top: "10px",
									right: "10px",
									minWidth: "200px",
								}}
							>
								<WinGameButton gameId={gameId} />
							</div>
							<AreYouThere />
						</>
					)}
					{/* <button onClick={() => confettiReward()}>Confetti</button> */}

					{gameId &&
						gameStatus === "waiting_for_players" &&
						gameMachines &&
						gameMachines?.length < MIN_PLAYERS && (
							<WaitingForPlayersOverlay
								allowForceStart={
									gameMachines.length >=
									MIN_PLAYERS_TO_FORCE_START
								}
								minPlayers={MIN_PLAYERS}
								numPlayers={gameMachines.length}
							/>
						)}
					{gameId && gameStatus === "completed" && (
						<GameFinishedOverlay />
					)}
				</>
			)}
		</div>
	);
}

export default App;
