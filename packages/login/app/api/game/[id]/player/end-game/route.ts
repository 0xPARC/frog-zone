import { NextResponse } from "next/server";
import prisma from "@/lib/prisma/init";

export async function OPTIONS() {
  return NextResponse.json(null, { status: 204 });
}

export async function POST(
  request: Request,
  { params }: { params: { id: string } },
) {
  const { id: gameId } = params;

  if (!gameId) {
    return NextResponse.json(
      { success: false, message: "gameId is required." },
      { status: 400 },
    );
  }

  try {
    const { machineId } = await request.json();

    if (!machineId) {
      return NextResponse.json(
        { success: false, message: "publicKey and machineId are required." },
        { status: 400 },
      );
    }

    const playerGame = await prisma.playerGame.findFirst({
      where: {
        machineId: machineId,
        game: {
          gameId: gameId,
        },
      },
    });

    if (!playerGame) {
      return NextResponse.json(
        { success: false, message: "PlayerGame record not found." },
        { status: 404 },
      );
    }
    const updatedPlayerGame = await prisma.playerGame.update({
      where: {
        id: playerGame.id,
      },
      data: {
        hasEndedGame: true,
      },
    });

    const remainingPlayersInGame = await prisma.playerGame.count({
      where: {
        gameId: playerGame.gameId,
        hasEndedGame: false,
      },
    });

    if (remainingPlayersInGame === 0) {
      // If all players have ended the game, mark the game as completed and wasAborted
      await prisma.game.update({
        where: { id: playerGame.gameId },
        data: { status: "completed", wasAborted: true, updatedAt: new Date() },
      });
    }

    return NextResponse.json(
      { success: true, playerGame: updatedPlayerGame },
      { status: 200 },
    );
  } catch (error) {
    return NextResponse.json(
      {
        success: false,
        message: `Failed to update player status: ${JSON.stringify(error)}`,
      },
      { status: 500 },
    );
  }
}
