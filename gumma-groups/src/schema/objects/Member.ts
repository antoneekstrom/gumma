import { builder } from "../builder"

/// OBJECT

export const Member = builder.prismaObject("Member", {
  fields: (t) => ({
    group: t.relation("group"),
    person: t.relation("person"),
  })
})
