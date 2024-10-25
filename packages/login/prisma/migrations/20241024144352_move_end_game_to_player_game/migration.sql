/*
  Warnings:

  - You are about to drop the column `hasEndedGame` on the `Player` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Player" DROP COLUMN "hasEndedGame";

-- AlterTable
ALTER TABLE "PlayerGame" ADD COLUMN     "hasEndedGame" BOOLEAN NOT NULL DEFAULT false;
