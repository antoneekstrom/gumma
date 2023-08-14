import { PrismaClient } from "@prisma/client"

export const prisma = new PrismaClient().$extends({
  result: {
    group: {
      isActive: {
        compute({ activeStart, activeEnd }) {
          return Date.now() > activeStart.getTime() && Date.now() < activeEnd.getTime()
        },
      }
    }
  }
})