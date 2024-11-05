import React from "react";
import useStore from "../store";
import ENTITIES_CONFIG from "../../const/entities.config";

type PlayerInfoProps = {
	playerId: number;
};

const styles = {
	infoBox: {
		position: "absolute" as "absolute",
		top: "10px",
		left: "10px",
		backgroundColor: "rgba(0,0,0,0.8)",
		fontFamily: "monospace",
		color: "#fff",
		paddingLeft: "10px",
		paddingRight: "10px",
		border: "1px solid #fff",
		zIndex: 10,
		minWidth: "150px",
	},
};

export const PlayerInfo: React.FC<PlayerInfoProps> = ({ playerId }) => {
	const player = useStore((state) => state.getPlayerById(playerId));

	if (!player) {
		return null;
	}

	return (
		<div style={styles.infoBox}>
			<h4>
				<b>Player: {ENTITIES_CONFIG.players[Number(playerId)].name}</b>
			</h4>
			<p>HP: {player.hp}</p>
			<p>ATK: {player.atk}</p>
			<p>
				x: {player.coord.x} y: {player.coord.y}
			</p>
		</div>
	);
};
