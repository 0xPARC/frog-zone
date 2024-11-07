import { NextResponse } from "next/server";
import prisma from "@/lib/prisma/init";

export async function GET() {
  try {
    // Query to get the aggregated scores for each player
    const scoreboard = await prisma.playerGame.groupBy({
      by: ["publicKey"],
      _sum: { score: true },
      orderBy: { _sum: { score: "desc" } }, // Sort by highest scores
    });

    // Format the response to match the expected output structure
    const response = scoreboard.map((player) => ({
      publicKey: player.publicKey,
      score: player._sum.score,
    }));

    return NextResponse.json({
      success: true,
      players: response,
    });
  } catch (error) {
    console.error("Error retrieving scoreboard:", error);

    return NextResponse.json(
      { success: false, message: "Failed to retrieve scoreboard" },
      { status: 500 },
    );
  }
}
