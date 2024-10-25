import { PrismaClient } from "@prisma/client";

let prisma: PrismaClient;

if (process.env.NODE_ENV === "production") {
  prisma = new PrismaClient();
} else {
  // @ts-expect-error avoid global type error
  if (!global.prisma) {
    // @ts-expect-error avoid global type error
    global.prisma = new PrismaClient();
  }
  // @ts-expect-error avoid global type error
  prisma = global.prisma;
}

export default prisma;
