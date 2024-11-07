import type React from "react";
import ENTITIES_CONFIG from "../../const/entities.config";
import { getPlayerId } from "../../utils/getPlayerId";
import useStore from "../store";

export const Scoreboard: React.FC = () => {
	const players = useStore.getState().game?.players || [];
	const sortedPlayers = [...players].sort((a, b) => b.score - a.score);
	const selectedPlayerId = getPlayerId();

	return (
		<div>
			{sortedPlayers.length > 0 ? (
				<>
					{sortedPlayers[0].machineId === selectedPlayerId ? (
						<h1>Congratulations! You won!! 🎉</h1>
					) : (
						<h1>Thank you for playing FROG ZONE!</h1>
					)}
					<table style={styles.table}>
						<thead>
							<tr>
								<th></th>
								<th style={styles.th}>Player</th>
								<th style={styles.th}>Score</th>
							</tr>
						</thead>
						<tbody>
							{sortedPlayers.map((player, index) => (
								<tr
									key={player.publicKey}
									style={{
										color:
											player.machineId === selectedPlayerId
												? "yellow"
												: "white",
									}}
								>
									<td>{index + 1}.</td>
									<td style={styles.td}>
										{ENTITIES_CONFIG.players[Number(player.machineId)].name}
										{index === 0 && " 🏆"}
									</td>
									<td style={styles.td}>
										<span style={styles.score}>{player.score}</span>
									</td>
								</tr>
							))}
						</tbody>
					</table>
				</>
			) : (
				<p>No players found.</p>
			)}
		</div>
	);
};

const styles = {
	table: {
		width: "100%",
		borderCollapse: "collapse" as const,
		marginTop: "20px",
		minWidth: "400px",
		marginBottom: "40px",
	},
	th: {
		// border: "1px solid white",
		textAlign: "left" as const,
		padding: "8px",
	},
	td: {
		// border: "1px solid white",
		textAlign: "left" as const,
		padding: "8px",
	},
	score: {
		fontWeight: "bold",
	},
};
