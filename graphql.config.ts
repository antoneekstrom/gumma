import type { IGraphQLConfig } from "graphql-config"
import { schema } from "./gumma-groups/src/schema"
import { printSchema } from "graphql"

export default {
  schema: printSchema(schema),
  documents: "./gumma-groups/**/*.ts"
} satisfies IGraphQLConfig