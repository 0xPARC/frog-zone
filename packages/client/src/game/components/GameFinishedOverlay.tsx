import React from "react";
import NewGameButton from "./NewGameButton";

export const GameFinishedOverlay: React.FC = () => {
	return (
		<div style={styles.overlay}>
			<div>
				<h1>Thank you for playing FROG ZONE!</h1>
			</div>
			<div>
				<NewGameButton />
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
};
