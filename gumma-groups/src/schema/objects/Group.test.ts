import { describe, expect, test } from "vitest"
import { gql } from "../../graphql/__generated__"
import { execute } from "../../test"
import dayjs from "dayjs"
import { prisma } from "../../prisma"

describe("group active period", async () => {
  test("add person existing to group", async () => {
    const document = gql(`
    mutation AddPersonToExistingGroupTest($personId: String!, $groupId: String!) {
      addPersonToGroup(personId: $personId, groupId: $groupId) {
        success
      }
    }
  `)

    const result = await execute(document, {
      personId: (await prisma.person.findFirstOrThrow()).id,
      groupId: (await prisma.group.findFirstOrThrow()).id
    })

    expect(result.data?.addPersonToGroup?.success).toEqual(true)
  })
  test("end before start date", async () => {
    const document = gql(`
      mutation EndBeforeStartTest($start: String!, $end: String!) {
        createGroup(input: {
          activeStart: $start,
          activeEnd: $end,
          name: "test"
        }) {
          success
        }
      }
    `)

    const result = await execute(document, {
      start: dayjs().toISOString(),
      end: dayjs().subtract(1, "year").toISOString()
    })

    expect(result.errors).toBeDefined()
  })
  test("invalid date format", async() => {
    const document = gql(`
    mutation InvalidDateFormatTest($end: String!) {
      createGroup(input: {
        activeEnd: $end,
        name: "test"
      }) {
        success
      }
    }
    `)
    const result = await execute(document, {
      end: "2023-08-14 16:50"
    })

    expect(result.errors).toBeDefined()
  })
})
