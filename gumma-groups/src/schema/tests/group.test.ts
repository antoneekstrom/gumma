import { describe, expect, test } from "vitest"
import { parse } from "graphql"
import { buildHTTPExecutor } from "@graphql-tools/executor-http"
import { createYoga } from "graphql-yoga"
import { ZodRawShape, z } from "zod"
import { schema } from ".."

export const group = z.object({
  id: z.string().cuid().optional()
})

// wraps as data property in zod object
const resultObject = <T extends ZodRawShape>(result: T) => z.object({
  data: z.object(result)
})

describe("group", async () => {
  const yoga = createYoga({
    schema,
  })
  
  const executor = buildHTTPExecutor({
    fetch: yoga.fetch
  })

  test("create group", async () => {
    const document = parse(/* GraphQL */`
      mutation {
        createGroup {
          success
          group {
            id
          }
        }
      }
    `)

    const schema = resultObject({
      createGroup: z.object({
        success: z.boolean().optional(),
        group
      })
    })

    expect(schema.parse(await executor({ document }))).toBeTypeOf("object")
  })

  test("get all groups", async () => {
    const document = parse(/* GraphQL */`
      query {
        groups {
          id
        }
      }
    `)

    const schema = resultObject({
      groups: z.array(group).optional()
    })

    expect(schema.parse(await executor({ document }))).toBeTypeOf("object")
  })
})