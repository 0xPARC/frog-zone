import { NextResponse } from "next/server";
import prisma from "@/lib/prisma/init";

export async function OPTIONS() {
  return NextResponse.json(null, { status: 204 });
}

export async function GET(
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
    const game = await prisma.game.findUnique({
      where: { gameId },
      include: {
        machines: true,
        players: true,
      },
    });

    if (!game) {
      return NextResponse.json(
        { success: false, message: "Game not found." },
        { status: 404 },
      );
    }

    return NextResponse.json({ success: true, game }, { status: 200 });
  } catch (error) {
    return NextResponse.json(
      {
        success: false,
        message: `Failed to fetch game: ${JSON.stringify(error)}`,
      },
      { status: 500 },
    );
  }
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
    const { status } = await request.json();

    if (!status) {
      return NextResponse.json(
        { success: false, message: "Status is required." },
        { status: 400 },
      );
    }

    const updatedGame = await prisma.game.update({
      where: { gameId },
      data: { status },
    });

    return NextResponse.json(
      { success: true, game: updatedGame },
      { status: 200 },
    );
  } catch (error) {
    return NextResponse.json(
      {
        success: false,
        message: `Failed to fetch game: ${JSON.stringify(error)}`,
      },
      { status: 500 },
    );
  }
}
