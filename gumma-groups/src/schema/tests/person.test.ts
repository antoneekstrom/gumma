import { describe, expect, test } from "vitest"
import { createYoga } from "graphql-yoga"
import { z } from "zod"
import schema from ".."
import { gql } from "../../graphql/__generated__"
import { ExecutionResult, print } from "graphql"
import { TypedDocumentNode } from "@graphql-typed-document-node/core"

export const person = z.object({
  cid: z.string(),
  id: z.string().cuid(),
  groups: z.array(
    z.object({
      id: z.string().cuid()
    })
  ).optional()
})

describe("person", async () => {
  const yoga = createYoga({
    schema,
  })
  
  async function executeOperation<TResult, TVariables>(
    operation: TypedDocumentNode<TResult, TVariables>,
    ...[variables]: TVariables extends Record<string, never> ? [] : [TVariables]
  ): Promise<ExecutionResult<TResult>> {
    const response = await Promise.resolve(
      yoga.fetch("http://yoga/graphql", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json"
        },
        body: JSON.stringify({
          query: print(operation),
          variables: variables ?? undefined
        })
      })
    )
    return await response.json()
  }

  test("get groups from person", async () => {
    const document = gql(`
      query TestGetGroupsFromPerson {
        person(cid: "antoneks") {
          cid
          id
          groups {
            id
          }
        }
      }
    `)

    expect(person.parse((await executeOperation(document)).data?.person)).toBeTypeOf("object")
  })

  test("get person by cid", async () => {
    const document = gql(`
      query TestGetPersonByCid {
        person(cid: "antoneks") {
          id
          cid
        }
      }
    `)

    expect(person.parse((await executeOperation(document)).data?.person)).toBeTypeOf("object")
  })
})
