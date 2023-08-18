import SchemaBuilder from "@pothos/core"
import PrismaPlugin from "@pothos/plugin-prisma"
import SimpleObjectPlugin from "@pothos/plugin-simple-objects"
import ValidationPlugin from "@pothos/plugin-validation"
import RelayPlugin from "@pothos/plugin-relay"
import PrismaTypes from "@pothos/plugin-prisma/generated"
import { Group } from "@prisma/client"
import { prisma } from "../prisma"
import { GraphQLError } from "graphql"

type Builder = {
  PrismaTypes: PrismaTypes & { Group: { Shape: Group & { isActive: boolean } } }
}

export const builder = new SchemaBuilder<Builder>({
  plugins: [PrismaPlugin, SimpleObjectPlugin, RelayPlugin, ValidationPlugin],
  prisma: {
    client: prisma,
    exposeDescriptions: true
  },
  relayOptions: {
    clientMutationId: "omit",
    cursorType: "String"
  },
  validationOptions: {
    validationError: (error) => {
      return new GraphQLError("Validation Error", {
        extensions: error.formErrors
      })
    }
  }
})
