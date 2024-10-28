import React, { useState, useEffect } from "react";
import { Modal } from "../../components/Modal";
import { Button } from "../../components/Button";
import useStore from "../store";
import { requestEndGame } from "../../utils/requestEndGame";
import { getPlayerId } from "../../utils/getPlayerId";

// CTRL + Q triggers the modal. Allows users to quit the game, the game will end if all users quit

const quitModalStyles = {
	modalActions: {
		display: "flex",
		justifyContent: "space-between",
		marginTop: "20px",
		gap: "20px",
	},
	quitButton: {
		background: "red",
		color: "white",
		padding: "10px 20px",
		border: "none",
		cursor: "pointer",
	},
	cancelButton: {
		background: "grey",
		color: "white",
		padding: "10px 20px",
		border: "none",
		cursor: "pointer",
	},
};

export const QuitGameModal: React.FC = () => {
	const [isOpen, setIsOpen] = useState(false);
	const [requestedEndGame, setRequestedEndGame] = useState(false);
	const gameId = useStore.getState().game?.gameId;
	const machineId = getPlayerId() as string;

	const handleQuit = async () => {
		if (gameId) {
			setRequestedEndGame(true);
			const data = await requestEndGame({ gameId, machineId });
			if (!data.success) {
				setRequestedEndGame(false);
			}
		}
	};

	const handleKeyPress = (event: KeyboardEvent) => {
		if (event.ctrlKey && event.key === "q") {
			setIsOpen(true);
		}
	};

	useEffect(() => {
		window.addEventListener("keydown", handleKeyPress);
		return () => {
			window.removeEventListener("keydown", handleKeyPress);
		};
	}, []);

	return (
		<>
			<Modal
				isOpen={isOpen}
				onClose={() => setIsOpen(false)}
				title="Quit Game?"
				isCloseable={!requestedEndGame}
			>
				{!requestedEndGame ? (
					<>
						<p>Are you sure you want to quit the game?</p>
						<div style={quitModalStyles.modalActions}>
							<Button
								onClick={() => setIsOpen(false)}
								style={{ backgroundColor: "grey" }}
							>
								Cancel
							</Button>
							<Button
								onClick={handleQuit}
								style={{ backgroundColor: "red" }}
							>
								Quit
							</Button>
						</div>
					</>
				) : (
					<p>Quitting... </p>
				)}
			</Modal>
		</>
	);
};
