import React from "react";
import useStore from "../store";

const NewGameButton: React.FC = () => {
	const handleNewGame = async () => {
		useStore.getState().setGame(null);
		useStore.getState().setIsLoggedIn(false);
		window.location.reload();
	};

	return (
		<button
			onClick={handleNewGame}
			style={{
				background: "blue",
				color: "white",
				fontFamily: "monospace",
				borderRadius: "5px",
				padding: "10px 15px",
				border: "none",
			}}
		>
			Start New Game
		</button>
	);
};

export default NewGameButton;
