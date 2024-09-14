import GraphQL

struct RuntimeSpec {
  struct Basic {
    struct Definition {
      struct BasicObject {
        var idNonNullArg: (_: (source: SourceSpec.BasicObject, args: (idNonNull: Any, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Any
        var stringNonNullArg: (_: (source: SourceSpec.BasicObject, args: (stringNonNull: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
        var intNonNullArg: (_: (source: SourceSpec.BasicObject, args: (intNonNull: Int, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Int
        var floatNonNullArg: (_: (source: SourceSpec.BasicObject, args: (floatNonNull: Float, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Float
        var booleanNonNullArg: (_: (source: SourceSpec.BasicObject, args: (booleanNonNull: Bool, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Bool
        var idArg: (_: (source: SourceSpec.BasicObject, args: (id: Any?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Any?
        var stringArg: (_: (source: SourceSpec.BasicObject, args: (string: String?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String?
        var intArg: (_: (source: SourceSpec.BasicObject, args: (int: Int?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Int?
        var floatArg: (_: (source: SourceSpec.BasicObject, args: (float: Float?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Float?
        var booleanArg: (_: (source: SourceSpec.BasicObject, args: (boolean: Bool?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Bool?
        var basicInputArg: (_: (source: SourceSpec.BasicObject, args: (basicInput: SourceSpec.BasicInput?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String?
        var basicInputArgWithDefault: (_: (source: SourceSpec.BasicObject, args: (basicInput: SourceSpec.BasicInput?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String?
      }
      struct BasicInterface {
      }
      struct BasicInterfaceImpl {
      }
      struct BasicScalar {
        var serialize: (_ value: Any) throws -> GraphQL.Map
        var parseValue: (_ value: GraphQL.Map) throws -> GraphQL.Map
        var parseLiteral: (_ value: GraphQL.Value) throws -> GraphQL.Map
      }
      var BasicObject: BasicObject
      var BasicInterface: BasicInterface
      var BasicInterfaceImpl: BasicInterfaceImpl
      var BasicScalar: BasicScalar
    }
    struct Extension {
      struct BasicObject {
        var extendedFieldWithArg: (_: (source: SourceSpec.BasicObject, args: (idNonNull: Any, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Any
      }
      struct AnotherInterface {
      }
      struct BasicInterface {
        var extendedFieldWithArg: (_: (source: SourceSpec.BasicInterface, args: (idNonNull: Any, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Any
      }
      struct BasicInterfaceImpl {
        var extendedFieldWithArg: (_: (source: SourceSpec.BasicInterfaceImpl, args: (idNonNull: Any, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Any
      }
      struct AnotherType {
      }
      struct Query {
        var extendedHello: (_: (source: SourceSpec.Query, args: (name: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
      }
      var BasicObject: BasicObject
      var AnotherInterface: AnotherInterface
      var BasicInterface: BasicInterface
      var BasicInterfaceImpl: BasicInterfaceImpl
      var AnotherType: AnotherType
      var Query: Query
    }
    var Definition: Definition
    var Extension: Extension
  }
  struct Deprecation {
    struct Definition {
      struct DeprecatedFieldObject {
      }
      var DeprecatedFieldObject: DeprecatedFieldObject
    }
    var Definition: Definition
  }
  struct Module_ {
    struct Module_ {
      struct module {
        struct ModuleB {
        }
        var ModuleB: ModuleB
      }
      var module: module
    }
    struct module {
      struct ModuleA {
      }
      var ModuleA: ModuleA
    }
    var Module_: Module_
    var module: module
  }
  struct Nested1 {
    struct Nested2 {
      struct nested2 {
        struct Nested2 {
        }
        var Nested2: Nested2
      }
      var nested2: nested2
    }
    struct nested1 {
      struct Nested1 {
      }
      var Nested1: Nested1
    }
    var Nested2: Nested2
    var nested1: nested1
  }
  struct Resolve {
    struct Schema {
      struct GrandParent {
        var echo: (_: (source: SourceSpec.GrandParent, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
        var echoSync: (_: (source: SourceSpec.GrandParent, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) throws -> String
      }
      struct Parent {
        var echo: (_: (source: SourceSpec.Parent, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
        var echoSync: (_: (source: SourceSpec.Parent, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
      }
      struct Child {
        var echo: (_: (source: SourceSpec.Child, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
        var echoSync: (_: (source: SourceSpec.Child, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
        var basicInterface: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> SourceSpec.BasicInterface?
        var basicInterfaceList: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicInterface?]?
        var basicInterfaceNonNullList: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicInterface?]
        var basicInterfaceNonNullListNonNullElement: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicInterface]
        var basicUnion: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> SourceSpec.BasicUnion?
        var basicUnionList: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicUnion?]?
        var basicUnionNonNullList: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicUnion?]
        var basicUnionNonNullListNonNullElement: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicUnion]
        var basicEnum: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> SourceSpec.BasicEnum?
        var basicEnumList: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicEnum?]?
        var basicEnumNonNullList: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicEnum?]
        var basicEnumNonNullListNonNullElement: (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicEnum]
      }
      var GrandParent: GrandParent
      var Parent: Parent
      var Child: Child
    }
    var Schema: Schema
  }
  struct graphql {
    struct GraphQLObject {
    }
    struct graphql {
    }
    struct typing {
    }
    var GraphQLObject: GraphQLObject
    var graphql: graphql
    var typing: typing
  }
  struct root {
    struct Query {
    }
    struct Mutation {
    }
    struct DateTime {
      var serialize: (_ value: Any) throws -> GraphQL.Map
      var parseValue: (_ value: GraphQL.Map) throws -> GraphQL.Map
      var parseLiteral: (_ value: GraphQL.Value) throws -> GraphQL.Map
    }
    var Query: Query
    var Mutation: Mutation
    var DateTime: DateTime
  }
  struct source {
    struct source {
      struct Source {
        var a: (_: (source: SourceSpec.Source, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Int?
      }
      var Source: Source
    }
    var source: source
  }
  var Basic: Basic
  var Deprecation: Deprecation
  var Module_: Module_
  var Nested1: Nested1
  var Resolve: Resolve
  var graphql: graphql
  var root: root
  var source: source
}