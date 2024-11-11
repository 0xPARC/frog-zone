import { useEffect, useState } from "react";
import { updateGameStatus } from "../../utils/updateGameStatus";
import { updatePlayer } from "../../utils/updatePlayer";
import useStore from "../store";

const TIME_UNTIL_AUTO_END = 10 * 1000;

export const GameTimer = ({playerId}: {playerId: number}) => {
  const gameId = useStore.getState().game?.gameId as string;
  const [countdown, setCountdown] = useState(TIME_UNTIL_AUTO_END / 1000);
  const publicKey = useStore.getState().publicKey as string;
  const player = useStore((state) => state.getPlayerById(playerId));

  useEffect(() => {
    const handleGameEnd = async () => {
      await updatePlayer({
  			score: player?.points || 0,
  			publicKey,
  			gameId,
  		});
  		await updateGameStatus({
  			gameId,
  			status: "completed",
  		});
  	};

		const timer = setInterval(async () => {
		  updatePlayer({
  			score: player?.points || 0,
  			publicKey,
  			gameId,
  		});
			setCountdown((prevCountdown) => {
				if (prevCountdown <= 1) {
					clearInterval(timer);
					handleGameEnd();
					return 0;
				}
				return prevCountdown - 1;
			});
		}, 1000);

		return () => clearInterval(timer);
	}, [gameId, player, publicKey]);

	const secondsRemaining = (countdown).toFixed(0);

	const containerStyle = {
		...styles.container,
		backgroundColor: "rgba(0,0,0,0.8)",
		border: "1px solid #fff",
	};

	return (
		<>
			<div style={containerStyle}>
				{countdown > 0 ? (
					<p>
						<b>{secondsRemaining}</b> seconds left to defeat the DRAGON. Go NORTH!
					</p>
				) : (
					<p>GAME OVER</p>
				)}
			</div>
		</>
	);
};

const styles = {
	container: {
		position: "fixed" as const,
		bottom: "20px",
		left: "20px",
		padding: "10px 20px",
		color: "white",
		fontSize: "14px",
		zIndex: 1000,
	},
};
