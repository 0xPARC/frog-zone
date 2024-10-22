import { PlayerInfo } from "./game/components/PlayerInfo";
import { Login } from "./game/components/Login";
import { getPlayerId } from "./utils/getPlayerId";
import { MoveCountdownTimer } from "./game/components/MoveCountdown";
import WinGameButton from "./game/components/WinGameButton";
import useStore from "./game/store";
import { GameFinishedOverlay } from "./game/components/GameFinishedOverlay";
import { WaitingForPlayersOverlay } from "./game/components/WaitingForPlayersOverlay";
import { useReward } from "react-rewards";
import { useEffect } from "react";

const MIN_PLAYERS = 4;
const MIN_PLAYERS_TO_FORCE_START = 1;
function App() {
	const { reward: confettiReward, isAnimating: isConfettiAnimating } =
		useReward("confettiReward", "confetti", {
			elementCount: 100,
			elementSize: 12,
			spread: 200,
			angle: -90,
		});
	const playerId = getPlayerId();
	const gameId = useStore((state) => state.game?.gameId);
	const gameStatus = useStore((state) => state.game?.status);
	const gameMachines = useStore((state) => state.game?.machines);
	useEffect(() => {
		if (gameStatus === "ongoing") {
			confettiReward();
		}
	}, [gameStatus]);
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
				{/* <button onClick={() => confettiReward()}>Confetti</button> */}
			</div>
			{gameId &&
				gameStatus === "waiting_for_players" &&
				gameMachines &&
				gameMachines?.length < MIN_PLAYERS && (
					<WaitingForPlayersOverlay
						allowForceStart={
							gameMachines.length >= MIN_PLAYERS_TO_FORCE_START
						}
						minPlayers={MIN_PLAYERS}
						numPlayers={gameMachines.length}
					/>
				)}
			{gameId && gameStatus === "completed" && <GameFinishedOverlay />}
			<div
				id="confettiReward"
				style={{ position: "absolute", top: "-60px", left: "50%" }}
			/>
		</div>
	);
}

export default App;
