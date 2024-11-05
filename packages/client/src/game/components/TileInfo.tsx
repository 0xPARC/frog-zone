import React from "react";
import useStore from "../store";
import ENTITIES_CONFIG from "../../const/entities.config";

const styles = {
	infoBox: {
		position: "absolute" as "absolute",
		top: "160px",
		left: "10px",
		backgroundColor: "rgba(0,0,0,0.8)",
		fontFamily: "monospace",
		color: "#fff",
		paddingLeft: "10px",
		paddingRight: "10px",
		border: "1px solid #fff",
		zIndex: 10,
		width: "250px",
	},
	tileInfo: {
		paddingBottom: "10px",
	},
};

const entityTypeToInfoMap: Record<string, string> = {
	None: "nothing",
	Player: "player",
	Item: "item",
};

export const TileInfo: React.FC = () => {
	const hoverTile = useStore((state) => state.hoverTile);

	if (!hoverTile) {
		return null;
	}

	return (
		<div style={styles.infoBox}>
			<h4>Tile Info</h4>
			{hoverTile.isShown ? (
				<>
					<div style={styles.tileInfo}>
						<p>
							x: {hoverTile.coord.x} y: {hoverTile.coord.y}
						</p>
						<p>
							contains:{" "}
							{entityTypeToInfoMap[hoverTile.entity_type]}
						</p>
						<p>terrain: {hoverTile.terrainType}</p>
					</div>
					{hoverTile.entity_type === "Player" && (
						<>
							<h4>Player Details</h4>
							<p>
								name:{" "}
								{ENTITIES_CONFIG["players"][hoverTile.entity_id]
									?.name ?? "UNKNOWN"}
							</p>
							<p>
								description:{" "}
								{ENTITIES_CONFIG["players"][hoverTile.entity_id]
									?.description ?? "???"}
							</p>
						</>
					)}
					{hoverTile.entity_type === "Item" && (
						<>
							<h4>Item Details</h4>
							<p>
								name:{" "}
								{ENTITIES_CONFIG["items"][hoverTile.entity_id]
									?.name ?? "UNKNOWN"}
							</p>
							<p>
								description:{" "}
								{ENTITIES_CONFIG["items"][hoverTile.entity_id]
									?.description ?? "???"}
							</p>
						</>
					)}
					{hoverTile.entity_type === "Monster" && (
						<>
							<h4>THERE'S A MONSTER !!</h4>
							<p>
								name:{" "}
								{ENTITIES_CONFIG["monsters"][
									hoverTile.entity_id
								]?.name ?? "UNKNOWN"}
							</p>
							<p>
								description:{" "}
								{ENTITIES_CONFIG["items"][hoverTile.entity_id]
									?.description ??
									"Yikes... We don't know anything about this one!"}
							</p>
						</>
					)}
				</>
			) : (
				<>
					<p>
						x: {hoverTile.coord.x} y: {hoverTile.coord.y}
					</p>
					<p>contains: ???</p>
					<p>terrain: ???</p>
				</>
			)}
		</div>
	);
};
