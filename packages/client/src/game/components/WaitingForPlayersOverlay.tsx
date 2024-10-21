import React from "react";
import ForceStartButton from "./ForceStartButton";

type WaitingForPlayersOverlayProps = {
	allowForceStart: boolean;
};

export const WaitingForPlayersOverlay: React.FC<
	WaitingForPlayersOverlayProps
> = ({ allowForceStart }: WaitingForPlayersOverlayProps) => {
	return (
		<div style={styles.overlay}>
			<div>
				<h1>Waiting for other players...</h1>
				<p>
					Please wait for other players to sign into to the machines
				</p>
			</div>
			<div>{allowForceStart && <ForceStartButton />}</div>
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
		backgroundColor: "rgba(0, 0, 0, 0.7)",
		display: "flex",
		flexDirection: "column" as "column",
		justifyContent: "center",
		alignItems: "center",
		textAlign: "center" as "center",
		zIndex: 1000,
	},
};
