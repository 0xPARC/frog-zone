import { useEffect, useState } from "react";
import useStore, { NEXT_MOVE_TIME_MILLIS } from "../store";

export const MoveCountdownTimer = () => {
	const lastMoveTime = useStore((state) => state.lastMoveTimeStamp);
	const [timeRemaining, setTimeRemaining] = useState(0);
	const [shake, setShake] = useState(false);

	useEffect(() => {
		const updateCountdown = () => {
			if (lastMoveTime) {
				const now = Date.now();
				const timeSinceMove = now - lastMoveTime;
				const remainingTime = Math.max(
					0,
					NEXT_MOVE_TIME_MILLIS - timeSinceMove,
				);

				setTimeRemaining(remainingTime);
			}
		};

		const intervalId = setInterval(updateCountdown, 100);
		return () => clearInterval(intervalId);
	}, [lastMoveTime]);

	useEffect(() => {
		const handleKeyPress = (event: KeyboardEvent) => {
			if (
				event.keyCode === 37 ||
				event.keyCode === 38 ||
				event.keyCode === 39 ||
				event.keyCode === 40
			) {
				if (timeRemaining > 0) {
					setShake(true);
					setTimeout(() => setShake(false), 300);
				}
			}
		};

		window.addEventListener("keydown", handleKeyPress);

		return () => {
			window.removeEventListener("keydown", handleKeyPress);
		};
	}, [timeRemaining]);

	const secondsRemaining = (timeRemaining / 1000).toFixed(1);

	const containerStyle = {
		...styles.container,
		backgroundColor:
			timeRemaining > 0
				? "rgba(255, 100, 0, 0.4)"
				: "rgba(0, 128, 0, 0.4)",
		animation: shake ? "shake 0.3s" : "none",
	};

	return (
		<>
			<div style={containerStyle}>
				{timeRemaining > 0 ? (
					<p>
						Next move available in: <b>{secondsRemaining}</b>{" "}
						seconds
					</p>
				) : (
					<p>You can move now!</p>
				)}
			</div>

			<style>{`
				@keyframes shake {
					0% {
						transform: translateX(0);
					}
					25% {
						transform: translateX(-10px);
					}
					50% {
						transform: translateX(10px);
					}
					75% {
						transform: translateX(-10px);
					}
					100% {
						transform: translateX(0);
					}
				}
			`}</style>
		</>
	);
};

const styles = {
	container: {
		position: "fixed" as "fixed",
		bottom: "20px",
		right: "20px",
		padding: "10px 20px",
		borderRadius: "8px",
		color: "white",
		fontSize: "14px",
		zIndex: 1000,
	},
};
