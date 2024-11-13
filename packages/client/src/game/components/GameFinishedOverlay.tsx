import type React from "react";
import { useEffect, useState } from "react";
import { Button } from "../../components/Button";
import { getPlayerId } from "../../utils/getPlayerId";
import { resetGame } from "../../utils/resetGame";
import useStore from "../store";
import { Scoreboard } from "./Scoreboard";
import { LOGIN_SERVER_URL } from "../../const/env.const";

const TIME_TO_AUTO_START_NEW_GAME = 60 * 1000;

export const GameFinishedOverlay: React.FC = () => {
	const wasAborted = useStore.getState().game?.wasAborted;
	const publicKey = useStore.getState().publicKey as string;
	const [countdown, setCountdown] = useState(
		TIME_TO_AUTO_START_NEW_GAME / 1000,
	);
	const playerId = getPlayerId() as string;

	useEffect(() => {
		if (wasAborted) {
			handleNewGame();
		}
	}, [wasAborted]);

	const handleNewGame = async () => {
		resetGame({ playerId });
		window.location.reload();
	};

	useEffect(() => {
		const countdownInterval = setInterval(() => {
			setCountdown((prevCountdown) => prevCountdown - 1);
		}, 1000);

		const autoStartTimer = setTimeout(() => {
			handleNewGame();
		}, TIME_TO_AUTO_START_NEW_GAME);

		return () => {
			clearInterval(countdownInterval);
			clearTimeout(autoStartTimer);
		};
	}, []);

	return (
		<div style={styles.overlay}>
			<Scoreboard />
			<iframe
				src={`${LOGIN_SERVER_URL}/score-board?publicKey=${encodeURIComponent(
					publicKey,
				)}`}
				style={{ border: "none", width: "700px", height: "500px" }}
			/>
			<div>
				<Button onClick={handleNewGame}>Start New Game Now</Button>
				<p style={styles.countdownMessage}>
					A new game will start automatically in{" "}
					<strong>{countdown}</strong> seconds.
				</p>
			</div>
		</div>
	);
};

const styles = {
	overlay: {
		position: "fixed" as const,
		top: 0,
		left: 0,
		width: "100vw",
		height: "100vh",
		backgroundColor: "rgba(0, 0, 0, 1)",
		display: "flex",
		flexDirection: "column" as const,
		justifyContent: "center",
		alignItems: "center",
		textAlign: "center" as const,
		zIndex: 1000,
	},
	countdownMessage: {
		marginTop: "30px",
		color: "#999",
	},
};
