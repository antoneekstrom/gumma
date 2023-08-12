import { describe, expect, test } from "vitest"
import { parse } from "graphql"
import { buildHTTPExecutor } from "@graphql-tools/executor-http"
import { createYoga } from "graphql-yoga"
import { ZodRawShape, z } from "zod"
import { schema } from ".."

export const person = z.object({
  cid: z.string(),
  id: z.string().cuid(),
  groups: z.array(
    z.object({
      id: z.string().cuid()
    })
  ).optional()
})

// wraps as data property in zod object
const resultObject = <T extends ZodRawShape>(result: T) => z.object({
  data: z.object(result)
})

describe("person", async () => {
  const yoga = createYoga({
    schema,
  })
  
  const executor = buildHTTPExecutor({
    fetch: yoga.fetch
  })

  test("get groups from person", async () => {
    const document = parse(/* GraphQL */`
      query {
        person(cid: "antoneks") {
          cid
          id
          groups {
            id
          }
        }
      }
    `)

    const schema = resultObject({ person })
    expect(schema.parse(await executor({ document }))).toBeTypeOf("object")
  })

  test("get person by cid", async () => {
    const document = parse(/* GraphQL */`
      query {
        person(cid: "antoneks") {
          id
          cid
        }
      }
    `)
  
    const schema = resultObject({
      person
    })

    expect(schema.parse(await executor({ document }))).toBeTypeOf("object")
  })
})
