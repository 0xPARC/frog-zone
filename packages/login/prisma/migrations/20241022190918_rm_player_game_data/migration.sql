/*
  Warnings:

  - You are about to drop the `Player` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `PlayerGame` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "PlayerGame" DROP CONSTRAINT "PlayerGame_gameId_fkey";

-- DropForeignKey
ALTER TABLE "PlayerGame" DROP CONSTRAINT "PlayerGame_playerId_fkey";

-- DropTable
DROP TABLE "Player";

-- DropTable
DROP TABLE "PlayerGame";
