import GraphQL



struct ResolverSpec {
  struct Basic {
    struct Definition {
      protocol BasicObject {
        func idNonNullArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, idNonNull: Any) throws -> Any
        func stringNonNullArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, stringNonNull: String) throws -> String
        func intNonNullArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, intNonNull: Int) throws -> Int
        func floatNonNullArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, floatNonNull: Float) throws -> Float
        func booleanNonNullArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, booleanNonNull: Bool) throws -> Bool
        func idArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, id: Any?) throws -> Any?
        func stringArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, string: String?) throws -> String?
        func intArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, int: Int?) throws -> Int?
        func floatArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, float: Float?) throws -> Float?
        func booleanArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, boolean: Bool?) throws -> Bool?
        func basicInputArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, basicInput: SourceSpec.BasicInput?) throws -> String?
        func basicInputArgWithDefault(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, basicInput: SourceSpec.BasicInput?) throws -> String?
      }
      protocol BasicInterface {
      }
      protocol BasicInterfaceImpl {
      }
      protocol BasicScalar {
        func serialize(_ value: Any) throws -> GraphQL.Map
        func parseValue(_ value: GraphQL.Map) throws -> GraphQL.Map
        func parseLiteral(_ value: GraphQL.Value) throws -> GraphQL.Map
      }
    }
    struct Extension {
      protocol BasicObject {
        func extendedFieldWithArg(obj: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, idNonNull: Any) throws -> Any
      }
      protocol AnotherInterface {
      }
      protocol BasicInterface {
        func extendedFieldWithArg(obj: SourceSpec.BasicInterface, info: GraphQL.GraphQLResolveInfo, idNonNull: Any) throws -> Any
      }
      protocol BasicInterfaceImpl {
        func extendedFieldWithArg(obj: SourceSpec.BasicInterfaceImpl, info: GraphQL.GraphQLResolveInfo, idNonNull: Any) throws -> Any
      }
      protocol AnotherType {
      }
      protocol Query {
        func extendedHello(obj: SourceSpec.Query, info: GraphQL.GraphQLResolveInfo, name: String) throws -> String
      }
    }
  }
  struct Deprecation {
    struct Definition {
      protocol DeprecatedFieldObject {
      }
    }
  }
  struct Module {
    struct Module {
      struct module {
        protocol ModuleB {
        }
      }
    }
    struct module {
      protocol ModuleA {
      }
    }
  }
  struct Nested1 {
    struct Nested2 {
      struct nested2 {
        protocol Nested2 {
        }
      }
    }
    struct nested1 {
      protocol Nested1 {
      }
    }
  }
  struct Resolve {
    struct Schema {
      protocol GrandParent {
        func echo(obj: SourceSpec.GrandParent, info: GraphQL.GraphQLResolveInfo, message: String) throws -> String
        func echoSync(obj: SourceSpec.GrandParent, info: GraphQL.GraphQLResolveInfo, message: String) async throws -> String
      }
      protocol Parent {
        func echo(obj: SourceSpec.Parent, info: GraphQL.GraphQLResolveInfo, message: String) throws -> String
        func echoSync(obj: SourceSpec.Parent, info: GraphQL.GraphQLResolveInfo, message: String) throws -> String
      }
      protocol Child {
        func echo(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, message: String) throws -> String
        func echoSync(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, message: String) throws -> String
        func basicInterface(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> SourceSpec.BasicInterface?
        func basicInterfaceList(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> [SourceSpec.BasicInterface?]?
        func basicInterfaceNonNullList(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> [SourceSpec.BasicInterface?]
        func basicInterfaceNonNullListNonNullElement(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> [SourceSpec.BasicInterface]
        func basicUnion(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> SourceSpec.BasicUnion?
        func basicUnionList(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> [SourceSpec.BasicUnion?]?
        func basicUnionNonNullList(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> [SourceSpec.BasicUnion?]
        func basicUnionNonNullListNonNullElement(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> [SourceSpec.BasicUnion]
        func basicEnum(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> SourceSpec.BasicEnum?
        func basicEnumList(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> [SourceSpec.BasicEnum?]?
        func basicEnumNonNullList(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> [SourceSpec.BasicEnum?]
        func basicEnumNonNullListNonNullElement(obj: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo) throws -> [SourceSpec.BasicEnum]
      }
    }
  }
  struct graphql {
    protocol GraphQLObject {
    }
    protocol graphql {
    }
    protocol typing {
    }
  }
  struct root {
    protocol Query {
    }
    protocol Mutation {
    }
  }
  struct source {
    struct source {
      protocol Source {
        func a(obj: SourceSpec.Source, info: GraphQL.GraphQLResolveInfo) throws -> Int?
      }
    }
  }
}