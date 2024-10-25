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
    const { publicKey, ...rest } = await request.json();

    if (!publicKey) {
      return NextResponse.json(
        { success: false, message: "publicKey is required." },
        { status: 400 },
      );
    }

    const playerGame = await prisma.playerGame.findFirst({
      where: {
        game: {
          gameId: gameId,
        },
        publicKey: publicKey,
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
        ...rest,
        updatedAt: new Date(),
        game: {
          update: {
            updatedAt: new Date(),
          },
        },
      },
    });

    return NextResponse.json(
      { success: true, playerGame: updatedPlayerGame },
      { status: 200 },
    );
  } catch (error) {
    return NextResponse.json(
      {
        success: false,
        message: `Failed to update player score: ${JSON.stringify(error)}`,
      },
      { status: 500 },
    );
  }
}
