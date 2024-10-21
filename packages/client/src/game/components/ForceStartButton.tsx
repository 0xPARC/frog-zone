import React from "react";
import useStore from "../store";

const ForceStartButton: React.FC = () => {
	const handleForceStart = async () => {
		useStore.getState().setForceStart(true);
	};

	return (
		<button
			onClick={handleForceStart}
			style={{
				background: "blue",
				color: "white",
				fontFamily: "monospace",
				borderRadius: "5px",
				padding: "10px 15px",
				border: "none",
			}}
		>
			Start Anyway
		</button>
	);
};

export default ForceStartButton;
