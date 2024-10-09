import { PlayerInfo } from "./game/components/PlayerInfo";
import { getPlayerId } from "./utils/getPlayerId";

function App() {
  const playerId = getPlayerId();
  return (
    <div>
      <PlayerInfo playerId={Number(playerId)} />
    </div>
  );
}

export default App;
