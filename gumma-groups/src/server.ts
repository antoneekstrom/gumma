import { createYoga } from "graphql-yoga"
import { createServer } from "node:http"
import { schema } from "./schema"

const PORT = 3000

export function run() {
  const yoga = createYoga({
    schema,
    graphiql: true,
  })
  
  const server = createServer(yoga)
  
  server.listen(PORT, () => {
    console.log(`Visit http://localhost:${PORT}/graphql`)
  })
}