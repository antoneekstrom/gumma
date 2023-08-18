import { z } from "zod"
import { prisma } from "../../prisma"
import { builder } from "../builder"
import dayjs from "dayjs"
import { Member } from "./Member"

prisma.$extends({
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

/// OBJECT

const Group = builder.prismaObject("Group", {
  fields: (t) => ({
    id: t.exposeString("id"),
    type: t.exposeString("type"),
    name: t.exposeString("name", { nullable: true }),
    members: t.relation("members"),
    activeStart: t.string({
      resolve: ({ activeStart }) => activeStart.toISOString()
    }),
    activeEnd: t.string({
      resolve: ({ activeEnd }) => activeEnd.toISOString()
    }),
    isActive: t.boolean({
      resolve: ({ isActive }) => isActive
    }),
  })
})

/// QUERIES

builder.queryFields((t) => ({
  groups: t.prismaField({
    type: ["Group"],
    args: {
      isActive: t.arg({
        type: "Boolean",
        required: false
      })
    },
    resolve: (query, _, { isActive }) => {
      
      if (isActive === undefined) {
        return prisma.group.findMany({
          ...query,
          where: {}
        })
      }

      const where = {
        activeStart: {
          lte: new Date(Date.now()),
        },
        activeEnd: {
          gt: new Date(Date.now())
        }
      }

      return prisma.group.findMany({
        ...query,
        where: isActive ? where : { NOT: where }
      })
    }
  })
}))

/// MUTATIONS

const CreateGroup = builder.simpleObject(
  "CreateGroup", {
    fields: (t) => ({
      success: t.boolean(),
      group: t.field({
        type: Group
      })
    })
  }
)

const CreateGroupInput = builder.inputType(
  "CreateGroupInput", {
    fields: (t) => ({
      activeStart: t.string({
        required: true,
        defaultValue: dayjs().toISOString(),
        validate: {
          schema: z.string().datetime()
        }
      }),
      activeEnd: t.string({
        required: true,
        validate: {
          schema: z.string().datetime()
        }
      }),
      name: t.string({
        required: true
      }),
    }),
    validate: [({ activeEnd, activeStart }) => {
      return dayjs(activeEnd).isAfter(activeStart)
    }, {
      message: "activeEnd date must be after activeStart date"
    }],
  }
)

builder.mutationField(
  "createGroup",
  (t) => t.field({
    type: CreateGroup,
    nullable: true,
    args: {
      input: t.arg({
        type: CreateGroupInput,
        required: true
      })
    },
    resolve: async (_, { input: { activeStart, activeEnd, name } }) => ({
      success: true,
      group: await prisma.group.create({
        data: {
          activeStart: new Date(activeStart!),
          activeEnd: new Date(activeEnd!),
          name
        }
      })
    })
  })
)

const AddPersonToGroup = builder.simpleObject(
  "AddPersonToGroup", {
    fields: (t) => ({
      success: t.boolean(),
      member: t.field({
        type: Member
      })
    })
  }
)

builder.mutationField(
  "addPersonToGroup",
  (t) => t.field({
    type: AddPersonToGroup,
    nullable: true,
    args: {
      personId: t.arg({
        type: "String",
        required: true
      }),
      groupId: t.arg({
        type: "String",
        required: true
      })
    },
    resolve: async (_, { personId, groupId }) => ({
      success: true,
      member: await prisma.member.create({
        data: {
          groupId,
          personId
        },
        include: {
          group: true
        }
      })
    })
  })
)