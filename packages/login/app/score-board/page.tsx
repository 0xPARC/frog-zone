"use client";

import { useSearchParams } from "next/navigation";
import { Suspense, useEffect, useState } from "react";

interface Player {
  publicKey: string;
  score: number;
}

function Scoreboard() {
  const searchParams = useSearchParams();
  const queryPublicKey = searchParams.get("publicKey");

  const [players, setPlayers] = useState<Player[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchScores = async () => {
      try {
        const response = await fetch("/api/scoreboard");
        if (!response.ok) {
          throw new Error("Failed to fetch scoreboard data");
        }
        const data = await response.json();
        setPlayers(data.players);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Unknown error");
      } finally {
        setIsLoading(false);
      }
    };

    fetchScores();
  }, []);

  const truncatePublicKey = (key: string): string => {
    return key.length > 10 ? `${key.slice(0, 6)}...${key.slice(-3)}` : key;
  };

  if (isLoading)
    return <p className="text-center p-4">Loading all scores...</p>;
  if (error)
    return <p className="text-center p-4">Error loading all scores: {error}</p>;

  // hack: compare only the start ofthe string to avoid encoding issues with // and other character commbos
  const highlightedIndex = players.findIndex(
    (player) =>
      player.publicKey.substring(0, 7) === queryPublicKey?.substring(0, 7),
  );
  const notFound = highlightedIndex < 0;
  const lastIndex = players.length - 1;

  return (
    <div className="bg-black text-white min-h-screen p-8">
      <h4 className="font-bold text-lg mb-6 text-center">Station Scoreboard</h4>
      <table className="w-full">
        <thead>
          <tr>
            <th className="border-b border-gray-700 p-4 text-left">
              Player Public Key
            </th>
            <th className="border-b border-gray-700 p-4 text-left">Score</th>
          </tr>
        </thead>
        <tbody>
          {/* If no queryPublicKey, display all players */}
          {!queryPublicKey &&
            players.map((player, index) => (
              <tr
                key={index}
                className={`${
                  index === 0 ? "bg-white text-black" : "even:bg-gray-900"
                }`}
              >
                <td className="p-4">
                  {index + 1}. {truncatePublicKey(player.publicKey)}
                  {index === 0 && " 🏆"}
                </td>
                <td className="p-4">{player.score}</td>
              </tr>
            ))}

          {/* match for public key not found */}
          {queryPublicKey && notFound && (
            <>
              {players.slice(0, 2).map((player, index) => (
                <tr key={index} className="even:bg-gray-900">
                  <td className="p-4">
                    {index + 1}. {truncatePublicKey(player.publicKey)}
                    {index === 0 && " 🏆"}
                  </td>
                  <td className="p-4">{player.score}</td>
                </tr>
              ))}
              <tr>
                <td className="p-4 text-center" colSpan={2}>
                  ...
                </td>
              </tr>
              {players[lastIndex] && (
                <tr
                  className={`${
                    highlightedIndex === lastIndex
                      ? "bg-yellow-500 text-black"
                      : "even:bg-gray-900"
                  }`}
                >
                  <td className="p-4">
                    {lastIndex + 1}.{" "}
                    {truncatePublicKey(players[lastIndex].publicKey)}
                  </td>
                  <td className="p-4">{players[lastIndex].score}</td>
                </tr>
              )}
            </>
          )}

          {/* If a queryPublicKey is provided, show selected rows */}
          {queryPublicKey && players[0] && !notFound && (
            <>
              {/* Show the first player */}
              <tr
                className={`${
                  highlightedIndex === 0 ? "bg-yellow-500 text-black" : ""
                }`}
              >
                <td className="p-4">
                  1. {truncatePublicKey(players[0].publicKey)} 🏆
                  {highlightedIndex === 0 && " (you)"}
                </td>
                <td className="p-4">{players[0].score}</td>
              </tr>
              {/* Show ellipsis if the highlighted player isn't the first player */}
              {highlightedIndex > 2 && (
                <tr>
                  <td className="p-4 text-center" colSpan={2}>
                    ...
                  </td>
                </tr>
              )}

              {/* Show the player before, highlighted player, and player after */}
              {highlightedIndex > 0 && (
                <>
                  {highlightedIndex > 1 && (
                    <tr className="even:bg-gray-900">
                      <td className="p-4">
                        {highlightedIndex}.{" "}
                        {truncatePublicKey(
                          players[highlightedIndex - 1].publicKey,
                        )}
                      </td>
                      <td className="p-4">
                        {players[highlightedIndex - 1].score}
                      </td>
                    </tr>
                  )}

                  <tr className="bg-yellow-500 text-black">
                    <td className="p-4">
                      {highlightedIndex + 1}.{" "}
                      {truncatePublicKey(players[highlightedIndex].publicKey)}
                      {" (you)"}
                    </td>
                    <td className="p-4">{players[highlightedIndex].score}</td>
                  </tr>

                  {highlightedIndex < lastIndex && (
                    <tr className="even:bg-gray-900">
                      <td className="p-4">
                        {highlightedIndex + 2}.{" "}
                        {truncatePublicKey(
                          players[highlightedIndex + 1].publicKey,
                        )}
                      </td>
                      <td className="p-4">
                        {players[highlightedIndex + 1].score}
                      </td>
                    </tr>
                  )}
                </>
              )}

              {/* Show ellipsis if the highlighted player is not second-to-last or last */}
              {highlightedIndex < lastIndex - 1 && (
                <tr>
                  <td className="p-4 text-center" colSpan={2}>
                    ...
                  </td>
                </tr>
              )}

              {/* Show the last player only if it’s not the highlighted player */}
              {highlightedIndex !== lastIndex && players[lastIndex] && (
                <tr
                  className={`${
                    highlightedIndex === lastIndex
                      ? "bg-yellow-500 text-black"
                      : "even:bg-gray-900"
                  }`}
                >
                  <td className="p-4">
                    {lastIndex + 1}.{" "}
                    {truncatePublicKey(players[lastIndex].publicKey)}
                    {" (you)"}
                  </td>
                  <td className="p-4">{players[lastIndex].score}</td>
                </tr>
              )}
            </>
          )}
        </tbody>
      </table>
    </div>
  );
}
export default function ScoreboardPage() {
  return (
    <Suspense
      fallback={<p className="text-center p-4">Loading all scores...</p>}
    >
      <Scoreboard />
    </Suspense>
  );
}
