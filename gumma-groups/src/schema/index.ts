import { builder } from "./builder"
import "./objects/Person"
import "./objects/Group"
import "./objects/Member"

builder.queryType()
builder.mutationType()

export const schema = builder.toSchema()
