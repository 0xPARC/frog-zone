import { useEffect, useState, useRef, useCallback } from "react";
import { updateGameStatus } from "../../utils/updateGameStatus";
import { updatePlayer } from "../../utils/updatePlayer";
import useStore from "../store";

const TIME_UNTIL_AUTO_END = 300 * 1000;

export const GameTimer = ({ playerId }: { playerId: number }) => {
	const gameId = useStore.getState().game?.gameId as string;
	const publicKey = useStore.getState().publicKey as string;
	const player = useStore((state) => state.getPlayerById(playerId));

	const [countdown, setCountdown] = useState(TIME_UNTIL_AUTO_END / 1000);
	const countdownRef = useRef(countdown);
	countdownRef.current = countdown;

	const handleGameEnd = useCallback(async () => {
		await updatePlayer({
			score: player?.points || 0,
			publicKey,
			gameId,
		});
		await updateGameStatus({
			gameId,
			status: "completed",
		});
	}, [gameId, player?.points, publicKey]);

	useEffect(() => {
		const updatePlayerScore = () => {
			if (countdownRef.current % 5 === 0) {
				updatePlayer({
					score: player?.points || 0,
					publicKey,
					gameId,
				});
			}
		};

		const timer = setInterval(() => {
			setCountdown((prevCountdown) => {
				if (prevCountdown <= 1) {
					clearInterval(timer);
					handleGameEnd();
					return 0;
				}
				updatePlayerScore();
				return prevCountdown - 1;
			});
		}, 1000);

		return () => clearInterval(timer);
	}, [gameId, handleGameEnd, player?.points, publicKey]);

	const secondsRemaining = countdown.toFixed(0);

	const containerStyle = {
		...styles.container,
		backgroundColor: "rgba(0,0,0,0.8)",
		border: "1px solid #fff",
	};

	return (
		<div style={containerStyle}>
			{countdown > 0 ? (
				<p>
					<b>{secondsRemaining}</b> seconds left to defeat the DRAGON.
					Go NORTH!
				</p>
			) : (
				<p>GAME OVER</p>
			)}
		</div>
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
