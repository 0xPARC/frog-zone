import type React from "react";
import { useState } from "react";
import { updateGameStatus } from "../../utils/updateGameStatus";
import { updatePlayer } from "../../utils/updatePlayer";
import useStore from "../store";

interface WinGameButtonProps {
	gameId: string;
}

const WinGameButton: React.FC<WinGameButtonProps> = ({ gameId }) => {
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [successMessage, setSuccessMessage] = useState<string | null>(null);
	const publicKey = useStore.getState().publicKey as string;

	const handleWinGame = async () => {
		setLoading(true);
		setError(null);
		setSuccessMessage(null);
		await updatePlayer({
			score: 100,
			publicKey,
			gameId,
		});
		await updateGameStatus({
			gameId,
			status: "completed",
		});
		setLoading(false);
	};

	return (
		<div
			style={{
				position: "absolute",
				top: "10px",
				right: "380px",
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
					minWidth: "100px",
				}}
			>
				{loading ? "Updating..." : "Win Game"}
			</button>
			{error && <p style={{ color: "red" }}>{error}</p>}
			{successMessage && <p style={{ color: "green" }}>{successMessage}</p>}
		</div>
	);
};

export default WinGameButton;
