import type React from "react";
import ENTITIES_CONFIG from "../../const/entities.config";
import useStore from "../store";
import heart from "../../../public/assets/heart_cropped.png";
import sword from "../../../public/assets/sword_cropped.png";

type PlayerInfoProps = {
	playerId: number;
};

const styles = {
	infoBox: {
		position: "absolute" as const,
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
	const publicKey = useStore.getState().publicKey as string;

	if (!player) {
		return null;
	}

	return (
		<div style={styles.infoBox}>
			<h4>
        <b>Player: {ENTITIES_CONFIG.players[Number(playerId)].name}</b>
			</h4>
			<p>Your Semaphore ID: ${publicKey}</p>
			<p>SCORE: {player.points * 10}</p>
			<div style={{display: "flex", alignItems: "center"}}>
			<p style={{ marginRight: "5px" }}>HP:</p>
			{[...Array(player.hp)].map((_, i) =>
        <img  key={i} src={heart} style={{ width: '20px', height: '20px'}} />)}
			</div>
			<div style={{display: "flex", alignItems: "center"}}>
			<p style={{ marginRight: "5px" }}>ATK:</p>
			{[...Array(player.atk)].map((_, i) =>
        <img  key={i} src={sword} style={{ width: '20px', height: '20px'}} />)}
			</div>
			<p>
				x: {player.coord.x} y: {player.coord.y}
			</p>
		</div>
	);
};
