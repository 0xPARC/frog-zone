import { NextResponse } from "next/server";
import prisma from "@/lib/prisma/init";

export async function POST(request: Request) {
  try {
    const { machineId, publicKey } = await request.json();

    if (!machineId || !publicKey) {
      return NextResponse.json(
        {
          success: false,
          message: "machineId and publicKey are required.",
        },
        { status: 400 },
      );
    }

    // Create a player record in case we want them later
    const player = await prisma.player.upsert({
      where: { publicKey },
      update: { publicKey },
      create: { publicKey },
    });

    // Upsert operation to login the player to the machine
    const upsertedMachine = await prisma.machine.upsert({
      where: { machineId },
      update: { publicKey },
      create: { machineId, publicKey },
    });

    let activeGame = await prisma.game.findFirst({
      where: {
        status: "waiting_for_players",
        machines: {
          some: {},
        },
      },
      include: {
        machines: true,
      },
      orderBy: {
        createdAt: "desc", // Get the most recent game
      },
    });

    if (activeGame && activeGame.machines.length < 4) {
      // If there is an active game and it's not full, add the player to this game
      await prisma.machine.update({
        where: { id: upsertedMachine.id },
        data: {
          gameId: activeGame.id,
        },
      });
      if (activeGame.machines.length === 3) {
        // if this is the 4th player set the game's status to ongoing
        await prisma.game.update({
          where: { gameId: activeGame.gameId },
          data: {
            status: "ongoing",
          },
        });
      }
    } else {
      const newGame = await prisma.game.create({
        data: {
          gameId: `game_${Date.now()}`,
          machines: {
            connect: { id: upsertedMachine.id },
          },
        },
        include: {
          machines: true,
        },
      });

      activeGame = newGame;
    }

    await prisma.playerGame.create({
      data: {
        game: { connect: { id: activeGame.id } },
        player: { connect: { id: player.id } },
        machineId: machineId,
        publicKey: publicKey,
      },
    });

    return NextResponse.json({
      success: true,
      message: "Player logged in and assigned to a game",
      machine: upsertedMachine,
      game: activeGame,
    });
  } catch (error) {
    console.error("Error saving to database:", error);

    return NextResponse.json(
      { success: false, message: "Failed to save data" },
      { status: 500 },
    );
  }
}
