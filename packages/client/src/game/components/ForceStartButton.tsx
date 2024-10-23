import React, { useState } from "react";
import useStore from "../store";
import { updateGameStatus } from "../../utils/updateGameStatus";
import { Button } from "../../components/Button";

const ForceStartButton: React.FC = () => {
	const [forceStartRequested, setForceStartRequested] = useState(false);
	const gameId = useStore((state) => state.game?.gameId);
	const handleForceStart = async () => {
		if (gameId) {
			setForceStartRequested(true);
			await updateGameStatus({ gameId, status: "ongoing" });
		}
	};

	return (
		<Button onClick={handleForceStart}>
			{forceStartRequested ? "Starting..." : "Start Anyway"}
		</Button>
	);
};

export default ForceStartButton;
