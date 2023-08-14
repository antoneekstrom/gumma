import { prisma } from "../../prisma"
import { builder } from "../builder"

/// OBJECT

const Person = builder.prismaObject("Person", {
  fields: (t) => ({
    id: t.exposeID("id"),
    cid: t.exposeID("cid"),
    groups: t.relation("groups"),
  })
})

/// QUERIES

builder.queryField(
  "person",
  (t) => t.prismaField({
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
)

builder.queryField(
  "people",
  (t) => t.prismaConnection({
    type: "Person",
    cursor: "id",
    resolve: async (query) => prisma.person.findMany({ ...query })
  })
)

/// MUTATIONS

const CreatePerson = builder.simpleObject(
  "CreatePerson", {
    fields: (t) => ({
      success: t.boolean(),
      person: t.field({ type: Person })
    })
  }
)

builder.mutationField(
  "createPerson",
  (t) => t.field({
    type: CreatePerson,
    args: {
      cid: t.arg({
        type: "String",
        required: true
      })
    },
    resolve: async (_, { cid }) => ({
      success: true,
      person: await prisma.person.create({
        data: {
          cid
        }
      })
    })
  })
)