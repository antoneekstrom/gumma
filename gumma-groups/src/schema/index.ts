import SchemaBuilder from "@pothos/core"
import { Group, PrismaClient } from "@prisma/client"
import PrismaPlugin from "@pothos/plugin-prisma"
import PrismaTypes from "@pothos/plugin-prisma/generated"

const prisma = new PrismaClient()

const builder = new SchemaBuilder<{ PrismaTypes: PrismaTypes }>({
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
    members: t.relation("members")
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
      resolve: (query) => prisma.group.findMany({
        ...query
      })
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
      resolve: async () => {
        const result = await prisma.group.create({
          data: {
            type: "COMMITTEE"
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

export const schema = builder.toSchema()
