import useStore from "../store"

const Dead = ({playerId}: {playerId: number})  => {

  const player = useStore((state) => state.getPlayerById(playerId));
  const isDead = player && player.hp === 0;
  console.log(isDead);
  return (
    <div style={{position: "absolute", backgroundColor: "rgb(35, 35, 35, 0.3)", display: "flex", width: (isDead ? "100%" : "0px"), height: (isDead ? "100%" : "0px"), justifyContent: "center", alignItems: "center"}}>
      <div style={{ position: "absolute", backgroundColor: "grey", zIndex: 15, width: (isDead ? "800px" : "0px"), height: (isDead ? "600px" : "0px"), display: "flex", alignItems: "center", justifyContent: "center", overflow: "hidden" }} >
        You are slain!
      </div>
    </div>
  )
}

export default Dead;
