import React, { useState } from "react";
import { updateGameStatus } from "../../utils/updateGameStatus";
import useStore from "../store";
import { fetchGame } from "../../utils/fetchGame";

interface WinGameButtonProps {
	gameId: string;
}

const WinGameButton: React.FC<WinGameButtonProps> = ({ gameId }) => {
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [successMessage, setSuccessMessage] = useState<string | null>(null);

	const handleWinGame = async () => {
		setLoading(true);
		setError(null);
		setSuccessMessage(null);
		const data = await updateGameStatus({
			gameId,
			status: "completed",
		});
		if (data.success && data.game) {
			useStore.getState().setGame(data.game);
		}
		setLoading(false);
	};

	return (
		<div
			style={{
				position: "absolute",
				top: "10px",
				right: "10px",
			}}
		>
			<button
				onClick={handleWinGame}
				disabled={loading}
				style={{
					background: "green",
					color: "white",
					fontFamily: "monospace",
					borderRadius: "5px",
					padding: "10px 15px",
					border: "none",
				}}
			>
				{loading ? "Updating..." : "Win Game"}
			</button>
			{error && <p style={{ color: "red" }}>{error}</p>}
			{successMessage && (
				<p style={{ color: "green" }}>{successMessage}</p>
			)}
		</div>
	);
};

export default WinGameButton;
