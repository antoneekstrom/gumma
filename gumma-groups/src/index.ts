import { createYoga } from "graphql-yoga"
import { createServer } from "node:http"
import schema from "./schema"

const PORT = 3000

createServer(
  createYoga({
    schema,
    graphiql: true,
  })).listen(PORT, () => {
  console.log(`Visit http://localhost:${PORT}/graphql`)
})
