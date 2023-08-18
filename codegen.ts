import type { CodegenConfig } from "@graphql-codegen/cli"
import type { IGraphQLConfig } from "graphql-config"
import config from "./graphql.config"

export default {
  ...config,
  generates: {
    "./gumma-groups/src/graphql/__generated__/": {
      preset: "client",
      presetConfig: {
        gqlTagName: "gql"
      }
    },
  },
} satisfies IGraphQLConfig & CodegenConfig