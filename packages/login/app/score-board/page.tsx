"use client";

import { useEffect, useState } from "react";

interface Player {
  publicKey: string;
  score: number;
}

export default function Scoreboard() {
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

  if (isLoading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;

  return (
    <div className="bg-black text-white min-h-screen p-8">
      <h1 className="font-bold text-2xl mb-6">Scoreboard</h1>
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
          {players.map((player, index) => (
            <tr key={index} className="even:bg-gray-900">
              <td className="p-4">
                {index + 1}. {truncatePublicKey(player.publicKey)}
                {index === 0 && " 🏆"}
              </td>
              <td className="p-4">{player.score}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
