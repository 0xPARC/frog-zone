import type React from "react";
import ENTITIES_CONFIG from "../../const/entities.config";
import useStore from "../store";
import heart from "../../../public/assets/heart_cropped.png";
import sword from "../../../public/assets/sword_cropped.png";

const styles = {
	infoBox: {
		position: "absolute" as const,
		top: "260px",
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
	Monster: "monster",
};

export const TileInfo: React.FC = () => {
	const hoverTile = useStore((state) => state.hoverTile);

	if (!hoverTile) {
		return null;
	}

	return (
		<div style={styles.infoBox}>
			<h4>Tile Details</h4>
			{hoverTile.isShown ? (
				<>
					<div style={styles.tileInfo}>
						<p>
							x: {hoverTile.coord.x} y: {hoverTile.coord.y}
						</p>
						<p>contains: {entityTypeToInfoMap[hoverTile.entity_type]}</p>
						<p>terrain: {hoverTile.terrainType}</p>
					</div>
					{(hoverTile.entity_type === "Player") && (
						<>
							<h4>Player Details</h4>
							<p>
								Name:{" "}
								{ENTITIES_CONFIG["players"][hoverTile.entity_id]?.name ??
									"UNKNOWN"}
							</p>
							<p>
								Description:{" "}
								{ENTITIES_CONFIG["players"][hoverTile.entity_id]?.description ??
									"???"}
							</p>
							<div style={{display: "flex", alignItems: "center"}}>
							<p style={{ marginRight: "5px" }}>HP:</p>
							{[...Array(hoverTile.hp)].map((_, i) =>
              <img  key={i} src={heart} style={{ width: '20px', height: '20px'}} />)}
							</div>
							<div style={{display: "flex", alignItems: "center"}}>
							<p style={{ marginRight: "5px" }}>ATK:</p>
							{[...Array(hoverTile.atk)].map((_, i) =>
              <img  key={i} src={sword} style={{ width: '20px', height: '20px'}} />)}
							</div>
						</>
					)}
					{(hoverTile.entity_type === "Item") && (
						<>
							<h4>Item Details</h4>
							<p>
								Name:{" "}
								{ENTITIES_CONFIG["items"][hoverTile.entity_id]?.name ??
									"UNKNOWN"}
							</p>
							<p>
								Description:{" "}
								{ENTITIES_CONFIG["items"][hoverTile.entity_id]?.description ??
									"???"}
							</p>
						</>
					)}
					{(hoverTile.entity_type === "Monster") && (
						<>
							<h4>Monster Details</h4>
							<p>
								Name:{" "}
								{ENTITIES_CONFIG["monsters"][hoverTile.entity_id]?.name ??
									"UNKNOWN"}
							</p>
							<p>
								Description:{" "}
								{ENTITIES_CONFIG["monsters"][hoverTile.entity_id]
									?.description ??
									"Yikes... We don't know anything about this one!"}
							</p>


							<div style={{display: "flex", alignItems: "center"}}>
							<p style={{ marginRight: "5px" }}>HP:</p>
							{[...Array(hoverTile.hp)].map((_, i) =>
              <img  key={i} src={heart} style={{ width: '20px', height: '20px'}} />)}
							</div>
							<div style={{display: "flex", alignItems: "center"}}>
							<p style={{ marginRight: "5px" }}>ATK:</p>
							{[...Array(hoverTile.atk)].map((_, i) =>
              <img  key={i} src={sword} style={{ width: '20px', height: '20px'}} />)}
							</div>
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
