import { NextResponse } from "next/server";
import prisma from "@/lib/prisma/init";

export async function OPTIONS() {
  return NextResponse.json(null, { status: 204 });
}

export async function GET(
  request: Request,
  { params }: { params: { id?: string } },
) {
  const machineId = params?.id;

  if (!machineId) {
    return NextResponse.json(
      {
        success: false,
        message: "machineId is required.",
      },
      { status: 400 },
    );
  }

  try {
    const ongoingGame = await prisma.game.findFirst({
      where: {
        status: {
          not: "completed",
        },
      },
      orderBy: {
        createdAt: "desc",
      },
      include: {
        machines: true,
      },
    });

    const machine = ongoingGame?.machines.find(
      (machine) => machine.machineId === machineId,
    );

    if (!machine || !machine.publicKey) {
      return NextResponse.json(
        {
          success: true,
          message: ongoingGame
            ? "Machine is not logged in the ongoing game"
            : "No ongoing game",
          isLoggedIn: false,
          game: ongoingGame,
          publicKey: null,
        },
        { status: 200 },
      );
    }

    return NextResponse.json(
      {
        success: true,
        message: "Machine and ongoing game found",
        isLoggedIn: true,
        game: ongoingGame,
        publicKey: machine.publicKey,
      },
      { status: 200 },
    );
  } catch (error) {
    console.error("Error checking machineId and ongoing game:", error);
    return NextResponse.json(
      {
        success: false,
        message: `Failed to check machineId and ongoing game: ${JSON.stringify(
          error,
        )}`,
      },
      { status: 500 },
    );
  }
}
