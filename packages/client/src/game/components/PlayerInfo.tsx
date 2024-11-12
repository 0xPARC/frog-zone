import type React from "react";
import ENTITIES_CONFIG from "../../const/entities.config";
import useStore from "../store";
import { PlayerHealthStat, PlayerHealthStatType } from "./PlayerHealthStat";

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
		maxWidth: "360px",
	},
};

export const PlayerInfo: React.FC<PlayerInfoProps> = ({ playerId }) => {
	const player = useStore((state) => state.getPlayerById(playerId));
	const publicKey = useStore.getState().publicKey as string;
	const atk = Number(player?.atk ?? 0);
	const hp = Number(player?.hp ?? 0);

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
			<PlayerHealthStat type={PlayerHealthStatType.HP} value={hp} />
			<PlayerHealthStat type={PlayerHealthStatType.ATK} value={atk} />
			<p>
				x: {player.coord.x} y: {player.coord.y}
			</p>
		</div>
	);
};
