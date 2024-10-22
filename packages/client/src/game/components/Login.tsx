import React, { useEffect, useState } from "react";
import { QRCodeSVG } from "qrcode.react";
import { getPlayerId } from "../../utils/getPlayerId";
import {
	fetchMachineStatus,
	MachineStatusResponse,
} from "../../utils/fetchMachineStatus";
import { LOGIN_SERVER_URL } from "../../const/env.const";
import useStore from "../store";
import { fetchGame } from "../../utils/fetchGame";

export const Login: React.FC = () => {
	const playerId = getPlayerId();
	const loginUrl = `${LOGIN_SERVER_URL}/login/${playerId}`;
	const isLoggedIn = useStore((state) => state.isLoggedIn);
	const gameId = useStore((state) => state.game?.gameId);

	useEffect(() => {
		const checkStatus = async () => {
			try {
				if (playerId && !isLoggedIn) {
					const data: MachineStatusResponse =
						await fetchMachineStatus({
							playerId,
						});
					console.log("MACHINE STATUS", data);
					if (data.isLoggedIn && data.game) {
						useStore.getState().setIsLoggedIn(data.isLoggedIn);
						useStore.getState().setGame(data.game);
					} else {
						useStore.getState().setIsLoggedIn(false);
					}
				} else if (playerId && isLoggedIn && gameId) {
					const data = await fetchGame({ gameId });
					console.log("GAME STATUS", data.game);
					useStore.getState().setGame(data.game);
				}
			} catch {
				useStore.getState().setIsLoggedIn(false);
			}
		};

		// Start polling every 1 second (1000 milliseconds)
		const intervalId = setInterval(checkStatus, 1000);

		// Cleanup polling when component unmounts
		return () => clearInterval(intervalId);
	}, [playerId, isLoggedIn, gameId]);

	if (isLoggedIn === true) {
		return null;
	}

	if (isLoggedIn === null) {
		return (
			<div style={styles.overlay}>
				<p>Loading...</p>
			</div>
		);
	}
	return (
		<div style={styles.overlay}>
			<div>
				<h1>Welcome to FROG ZONE!</h1>
				<p>Scan the QR code to login</p>
			</div>
			<div style={styles.qrContainer}>
				<QRCodeSVG value={loginUrl} size={300} />
			</div>
			<div>
				<p>
					Or follow the{" "}
					<a
						href={loginUrl}
						target="_blank"
						style={{ color: "#0099e0" }}
					>
						link
					</a>
				</p>
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
	qrContainer: {
		backgroundColor: "#fff",
		padding: "20px",
		marginTop: "20px",
		borderRadius: "10px",
		boxShadow: "0 4px 10px rgba(0, 0, 0, 0.3)",
	},
};
