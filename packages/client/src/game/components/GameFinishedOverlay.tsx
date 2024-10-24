import React, { useEffect, useState } from "react";
import useStore from "../store";
import { Button } from "../../components/Button";
import { Scoreboard } from "./Scoreboard";

const TIME_TO_AUTO_START_NEW_GAME = 60 * 1000;

export const GameFinishedOverlay: React.FC = () => {
	const wasAborted = useStore.getState().game?.wasAborted;
	const [countdown, setCountdown] = useState(
		TIME_TO_AUTO_START_NEW_GAME / 1000,
	);

	useEffect(() => {
		if (wasAborted) {
			handleNewGame();
		}
	}, [wasAborted]);

	const handleNewGame = async () => {
		useStore.getState().setGame(null);
		useStore.getState().setIsLoggedIn({
			isLoggedIn: false,
			publicKey: null,
		});
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
		position: "fixed" as "fixed",
		top: 0,
		left: 0,
		width: "100vw",
		height: "100vh",
		backgroundColor: "rgba(0, 0, 0, 1)",
		display: "flex",
		flexDirection: "column" as "column",
		justifyContent: "center",
		alignItems: "center",
		textAlign: "center" as "center",
		zIndex: 1000,
	},
	countdownMessage: {
		marginTop: "30px",
		color: "#999",
	},
};
