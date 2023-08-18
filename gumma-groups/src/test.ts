import { createYoga } from "graphql-yoga"
import { ExecutionResult, print } from "graphql"
import { TypedDocumentNode } from "@graphql-typed-document-node/core"
import { schema } from "./schema"

const yoga = createYoga({
  schema,
})

export async function execute<TResult, TVariables>(
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
