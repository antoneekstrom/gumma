import { Group } from "@prisma/client"
import { prisma } from "../../prisma"
import { builder } from "../builder"

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
      group: t.field({ type: Group })
    })
  }
)

builder.mutationField(
  "createGroup",
  (t) => t.field({
    type: CreateGroup,
    args: {
      activeStart: t.arg({
        type: "String",
        required: true,
        defaultValue: new Date(Date.now()).toISOString()
      }),
      activeEnd: t.arg({
        type: "String",
        required: true
      }),
      name: t.arg({
        type: "String",
        required: true
      }),
    },
    resolve: async (_, { activeStart, activeEnd, name }) => ({
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