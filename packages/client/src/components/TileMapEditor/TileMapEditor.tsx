import { useEffect, useState } from "react";
import tileMapConfig from "../../const/tile.config.json";
import { Button } from "../Button";
import { TerrainType } from "../../game/store";
import {
	findBorderCoordinates,
	Grid,
} from "../../utils/findBorderCoords";

/* TileMapEditor allows us to easily visualize and edit the game map.
Clicking on the tiles toggles them between LAND and WATER.
Exporting the JSON exports a the json to paste into the tile.config.json */

// Edit these to visualize player or item locations on the game map
const ITEM_COORDS = ["1,8", "2,9", "3,3", "1,1"];
const PLAYER_COORDS = ["1,2", "1,3", "4,5", "5,6"];

export const TileMapEditor = () => {
	const [gridData, setGridData] = useState(tileMapConfig);
	const [isVisible, setIsVisible] = useState(false);
	const waterCoordinatesBorderingLand = findBorderCoordinates(
		gridData as Grid,
	);

	const toggleTerrain = (x: number, y: number) => {
		const key = `${x},${y}`;
		setGridData((prevData) => ({
			...prevData,
			[key]: {
				terrainType:
					prevData[key as keyof typeof prevData].terrainType ===
					TerrainType.GRASS
						? TerrainType.WATER
						: TerrainType.GRASS,
			},
		}));
	};

	const exportJson = () => {
		const jsonString = JSON.stringify(gridData, null, 2);
		const blob = new Blob([jsonString], { type: "application/json" });
		const url = URL.createObjectURL(blob);

		const a = document.createElement("a");
		a.href = url;
		a.download = "terrain-grid.json";
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
		URL.revokeObjectURL(url);
	};

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.ctrlKey && event.key === "e") {
			event.preventDefault();
			setIsVisible((prev) => !prev);
		}
	};

	useEffect(() => {
		window.addEventListener("keydown", handleKeyDown);
		return () => {
			window.removeEventListener("keydown", handleKeyDown);
		};
	}, []);

	return (
		<>
			{isVisible && (
				<div
					style={{
						position: "absolute" as "absolute",
						top: 0,
						left: 0,
						minWidth: "100vw",
						minHeight: "100vh",
						backgroundColor: "rgba(0, 0, 0, 1)",
						alignItems: "center",
						textAlign: "center" as "center",
						zIndex: 1000,
					}}
				>
					<Button
						onClick={exportJson}
						style={{ marginTop: "20px", marginBottom: "20px" }}
					>
						Export JSON
					</Button>
					<div
						className="grid"
						style={{
							display: "grid",
							gridTemplateColumns: "repeat(32, 1.5vw)",
							gridGap: "2px",
						}}
					>
						{Object.keys(gridData).map((key) => {
							const [x, y] = key.split(",").map(Number);
							const terrainType =
								gridData[key as keyof typeof gridData]
									.terrainType;
							return (
								<div
									key={key}
									onClick={() => toggleTerrain(x, y)}
									style={{
										width: "1.5vw",
										height: "1.5vw",
										backgroundColor:
											terrainType === "LAND"
												? "green"
												: waterCoordinatesBorderingLand.includes(
														key,
												  )
												? "#03a9f4"
												: "blue",
										display: "flex",
										alignItems: "center",
										justifyContent: "center",
										color: "white",
										cursor: "pointer",
									}}
								>
									{PLAYER_COORDS.includes(key)
										? "P"
										: ITEM_COORDS.includes(key)
										? "I"
										: ""}
								</div>
							);
						})}
					</div>
				</div>
			)}
		</>
	);
};
