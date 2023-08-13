import SchemaBuilder from "@pothos/core"
import { Group, PrismaClient } from "@prisma/client"
import PrismaPlugin from "@pothos/plugin-prisma"
import PrismaTypes from "@pothos/plugin-prisma/generated"

const prisma = new PrismaClient().$extends({
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

const builder = new SchemaBuilder<{ PrismaTypes: PrismaTypes & { Group: { Shape: Group & { isActive: boolean } } } }>({
  plugins: [PrismaPlugin],
  prisma: {
    client: prisma,
    exposeDescriptions: true,
    filterConnectionTotalCount: true
  },
})

builder.prismaObject("Person", {
  fields: (t) => ({
    id: t.exposeID("id"),
    cid: t.exposeID("cid"),
    groups: t.relation("groups")
  })
})
  
builder.prismaObject("Group", {
  fields: (t) => ({
    id: t.exposeString("id"),
    activeStart: t.string({
      resolve: ({ activeStart }) => activeStart.toISOString()
    }),
    activeEnd: t.string({
      resolve: ({ activeEnd }) => activeEnd.toISOString()
    }),
    isActive: t.boolean({
      resolve: ({ isActive }) => isActive
    }),
    members: t.relation("members"),
  })
})
  
builder.queryType({
  fields: (t) => ({
    person: t.prismaField({
      type: "Person",
      args: {
        cid: t.arg({
          type: "String",
          required: true
        })
      },
      resolve: async (query, _, { cid }) => prisma.person.findUniqueOrThrow({
        ...query,
        where: {
          cid
        }
      })
    }),
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
  }),
})
  
  type MutationResult = {
    success: boolean
  }
  
const CreateGroup = builder.objectRef<MutationResult & Group>("CreateGroupResult").implement({
  fields: (t) => ({
    success: t.exposeBoolean("success"),
    group: t.prismaField({
      type: "Group",
      resolve: (query, { id }) => prisma.group.findUniqueOrThrow({
        ...query,
        where: {
          id
        }
      })
    })
  })
})
  
builder.mutationType({
  fields: (t) => ({
    createGroup: t.field({
      type: CreateGroup,
      args: {
        activeStart: t.arg({
          type: "String",
          required: false
        }),
        activeEnd: t.arg({
          type: "String",
          required: false
        })
      },
      resolve: async (_, { activeStart, activeEnd }) => {
        const result = await prisma.group.create({
          data: {
            type: "COMMITTEE",
            activeStart: new Date(activeStart!),
            activeEnd: new Date(activeEnd!),
          }
        })
        return {
          success: true,
          ...result
        }
      }
    })
  })
})

export default builder.toSchema()
