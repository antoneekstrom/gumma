import { describe, expect, test } from "vitest"
import { createYoga } from "graphql-yoga"
import { schema } from ".."
import { buildHTTPExecutor } from "@graphql-tools/executor-http"
import { parse } from "graphql"
import { z } from "zod"

describe("person", async () => {
  const yoga = createYoga({
    schema,
  })
  
  const executor = buildHTTPExecutor({
    fetch: yoga.fetch
  })

  test("get person by cid", async () => {
    const document = parse(/* GraphQL */`
      query GetPeople {
        people {
          edges {
            node {
              cid
            }
          }
        }
      }
    `)

    const result = await executor({ document }) as any

    console.log(result)
  
    expect(z.array(z.any()).parse(result.data.people.edges)).toBeTypeOf("object")
  })
})
