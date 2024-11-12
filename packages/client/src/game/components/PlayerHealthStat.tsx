import sword from "../../../public/assets/sword_cropped.png";
import heart from "../../../public/assets/heart_cropped.png";

const DEFAULT_TRUNC_AFTER = 15;

export enum PlayerHealthStatType {
	HP = "hp",
	ATK = "atk",
}

type PlayerHealthStatProps = {
	type: PlayerHealthStatType;
	value: number;
	truncateAt?: number;
};

const StatTypeToImage = {
	[PlayerHealthStatType.ATK]: sword,
	[PlayerHealthStatType.HP]: heart,
};

export const PlayerHealthStat = ({
	type,
	value,
	truncateAt = DEFAULT_TRUNC_AFTER,
}: PlayerHealthStatProps) => {
	return (
		<div style={{ display: "flex", alignItems: "center" }}>
			<p style={{ marginRight: "5px", textTransform: "uppercase" }}>
				{type}:
			</p>
			{value > 0 ? (
				[...Array(value)]
					.slice(0, truncateAt - 1)
					.map((_, i) => (
						<img
							key={i}
							src={StatTypeToImage[type]}
							style={{ width: "20px", height: "20px" }}
						/>
					))
			) : (
				<p>0</p>
			)}
			{value > truncateAt && (
				<p style={{ marginLeft: "5px" }}>+ {value - truncateAt} </p>
			)}
		</div>
	);
};
