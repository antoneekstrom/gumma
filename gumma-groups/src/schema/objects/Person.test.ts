import { describe, expect, test } from "vitest"
import { z } from "zod"
import { gql } from "../../graphql/__generated__"
import { execute } from "../../test"

describe("person", async () => {
  test("get person by cid", async () => {
    const document = gql(`
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

    const result = await execute(document)
    expect(z.array(z.any()).parse(result.data?.people.edges)).toBeTypeOf("object")
  })
})
