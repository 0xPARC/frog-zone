import React from "react";
import useStore, { Player } from "../store";

type PlayerInfoProps = {
  playerId: number;
};

const styles = {
  infoBox: {
    position: "absolute" as "absolute",
    top: "10px",
    left: "10px",
    backgroundColor: "rgba(0,0,0,0.5)",
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
  const players = useStore((state) => state.players);

  let player: Player | null = null;
  players.forEach((value) => {
    if (value.id === playerId) {
      player = value;
    }
  });

  if (!player) {
    return null;
  }

  return (
    <div style={styles.infoBox}>
      <h4>
        <b>Player {player.id}</b>
      </h4>
      <p>HP: {player.hp}</p>
      <p>ATK: {player.atk}</p>
    </div>
  );
};
