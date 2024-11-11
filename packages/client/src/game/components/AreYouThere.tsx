import type React from "react";
import { useEffect, useState } from "react";
import { Button } from "../../components/Button";
import { updateGameStatus } from "../../utils/updateGameStatus";
import { updatePlayer } from "../../utils/updatePlayer";
import useStore from "../store";

const TIME_UNTIL_ARE_YOU_THERE = 30 * 1000;
const TIME_UNTIL_AUTO_END = 30 * 1000;

const modalStyles = {
	overlay: {
		position: "fixed" as const,
		top: 0,
		left: 0,
		right: 0,
		bottom: 0,
		background: "rgba(0, 0, 0, 0.5)",
		display: "flex",
		justifyContent: "center",
		alignItems: "center",
	},
	content: {
		background: "black",
		padding: "20px",
		borderRadius: "8px",
		maxWidth: "400px",
		textAlign: "center" as const,
	},
};

interface AreYouThereModalProps {
	onClose: () => void;
	onTimeout: () => void;
}

const AreYouThereModal: React.FC<AreYouThereModalProps> = ({
	onClose,
	onTimeout,
}) => {
	const [countdown, setCountdown] = useState(TIME_UNTIL_AUTO_END / 1000);

	useEffect(() => {
		const timer = setInterval(() => {
			setCountdown((prevCountdown) => {
				if (prevCountdown <= 1) {
					clearInterval(timer);
					onTimeout();
					return 0;
				}
				return prevCountdown - 1;
			});
		}, 1000);

		return () => clearInterval(timer);
	}, [onTimeout]);

	// Listen for keydown events to close the modal
	useEffect(() => {
		const handleKeyDown = () => {
			onClose();
		};

		window.addEventListener("keydown", handleKeyDown);

		return () => {
			window.removeEventListener("keydown", handleKeyDown);
		};
	}, [onClose]);

	return (
		<div style={modalStyles.overlay}>
			<div style={modalStyles.content}>
				<h2>Are you there?</h2>
				<p>
					Please confirm within {countdown} seconds, or the game will
					automatically end.
				</p>
				<Button onClick={onClose}>Yes, I'm here</Button>
			</div>
		</div>
	);
};

export const AreYouThere: React.FC = () => {
	const publicKey = useStore.getState().publicKey as string;
	const lastUpdateTime = useStore.getState().game?.updatedAt as string;
	const gameId = useStore.getState().game?.gameId as string;
	const timerActive = Boolean(lastUpdateTime);
	const [showModal, setShowModal] = useState(false);

	const handleGameEnd = async () => {
		await updateGameStatus({
			gameId,
			status: "completed",
		});
		console.log("GAME ENDED due to inactivity.");
	};

	useEffect(() => {
		const lastUpdate = new Date(lastUpdateTime).getTime();
		const now = Date.now();
		const timeElapsed = now - lastUpdate;

		if (timerActive && timeElapsed > TIME_UNTIL_ARE_YOU_THERE) {
			setShowModal(true);
		} else {
			const remainingTime = TIME_UNTIL_ARE_YOU_THERE - timeElapsed;
			const timer = setTimeout(() => {
				if (timerActive) {
					setShowModal(true);
				}
			}, remainingTime);

			return () => {
				setShowModal(false);
				clearTimeout(timer);
			};
		}
	}, [lastUpdateTime, timerActive]);

	const closeModal = async () => {
		setShowModal(false);
		await updatePlayer({
			gameId: useStore.getState().game?.gameId as string,
			publicKey,
		});
	};

	return (
		<>
			{showModal && (
				<AreYouThereModal onClose={closeModal} onTimeout={handleGameEnd} />
			)}
		</>
	);
};
