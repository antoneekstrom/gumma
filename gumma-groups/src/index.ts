import { createYoga } from "graphql-yoga"
import { createServer } from "node:http"
import { schema } from "./schema"
import "./prisma"

const PORT = 3000

const yoga = createYoga({
  schema,
  graphiql: true,
})

const server = createServer(yoga)

async function startServer() {
  server.listen(PORT, () => {
    console.log(`Visit http://localhost:${PORT}/graphql`)
  })
}

async function killServer() {
  server.close((err) => {
    if (err) {
      startServer()
    }
  })
}

if (import.meta.hot) {
  startServer()
  import.meta.hot.on("vite:beforeFullReload", () => {
    killServer()
  })
}