import GraphQL

struct RuntimeSpec {
  struct Basic {
    struct Definition {
      struct BasicObject {
        var idNonNullArg: @escaping (_: (source: SourceSpec.BasicObject, args: (idNonNull: Any, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Any
        var stringNonNullArg: @escaping (_: (source: SourceSpec.BasicObject, args: (stringNonNull: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
        var intNonNullArg: @escaping (_: (source: SourceSpec.BasicObject, args: (intNonNull: Int, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Int
        var floatNonNullArg: @escaping (_: (source: SourceSpec.BasicObject, args: (floatNonNull: Float, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Float
        var booleanNonNullArg: @escaping (_: (source: SourceSpec.BasicObject, args: (booleanNonNull: Bool, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Bool
        var idArg: @escaping (_: (source: SourceSpec.BasicObject, args: (id: Any?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Any?
        var stringArg: @escaping (_: (source: SourceSpec.BasicObject, args: (string: String?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String?
        var intArg: @escaping (_: (source: SourceSpec.BasicObject, args: (int: Int?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Int?
        var floatArg: @escaping (_: (source: SourceSpec.BasicObject, args: (float: Float?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Float?
        var booleanArg: @escaping (_: (source: SourceSpec.BasicObject, args: (boolean: Bool?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Bool?
        var basicInputArg: @escaping (_: (source: SourceSpec.BasicObject, args: (basicInput: SourceSpec.BasicInput?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String?
        var basicInputArgWithDefault: @escaping (_: (source: SourceSpec.BasicObject, args: (basicInput: SourceSpec.BasicInput?, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String?
      }
      struct BasicInterface {
      }
      struct BasicInterfaceImpl {
      }
      struct BasicScalar {
        var serialize: @escaping (_ value: Any) throws -> GraphQL.Map
        var parseValue: @escaping (_ value: GraphQL.Map) throws -> GraphQL.Map
        var parseLiteral: @escaping (_ value: GraphQL.Value) throws -> GraphQL.Map
      }
      var BasicObject: BasicObject
      var BasicInterface: BasicInterface
      var BasicInterfaceImpl: BasicInterfaceImpl
      var BasicScalar: BasicScalar
    }
    struct Extension {
      struct BasicObject {
        var extendedFieldWithArg: @escaping (_: (source: SourceSpec.BasicObject, args: (idNonNull: Any, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Any
      }
      struct AnotherInterface {
      }
      struct BasicInterface {
        var extendedFieldWithArg: @escaping (_: (source: SourceSpec.BasicInterface, args: (idNonNull: Any, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Any
      }
      struct BasicInterfaceImpl {
        var extendedFieldWithArg: @escaping (_: (source: SourceSpec.BasicInterfaceImpl, args: (idNonNull: Any, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Any
      }
      struct AnotherType {
      }
      struct Query {
        var extendedHello: @escaping (_: (source: SourceSpec.Query, args: (name: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
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
        var echo: @escaping (_: (source: SourceSpec.GrandParent, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
        var echoSync: @escaping (_: (source: SourceSpec.GrandParent, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) throws -> String
      }
      struct Parent {
        var echo: @escaping (_: (source: SourceSpec.Parent, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
        var echoSync: @escaping (_: (source: SourceSpec.Parent, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
      }
      struct Child {
        var echo: @escaping (_: (source: SourceSpec.Child, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
        var echoSync: @escaping (_: (source: SourceSpec.Child, args: (message: String, _: ()), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> String
        var basicInterface: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> SourceSpec.BasicInterface?
        var basicInterfaceList: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicInterface?]?
        var basicInterfaceNonNullList: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicInterface?]
        var basicInterfaceNonNullListNonNullElement: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicInterface]
        var basicUnion: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> SourceSpec.BasicUnion?
        var basicUnionList: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicUnion?]?
        var basicUnionNonNullList: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicUnion?]
        var basicUnionNonNullListNonNullElement: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicUnion]
        var basicEnum: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> SourceSpec.BasicEnum?
        var basicEnumList: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicEnum?]?
        var basicEnumNonNullList: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicEnum?]
        var basicEnumNonNullListNonNullElement: @escaping (_: (source: SourceSpec.Child, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> [SourceSpec.BasicEnum]
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
      var serialize: @escaping (_ value: Any) throws -> GraphQL.Map
      var parseValue: @escaping (_ value: GraphQL.Map) throws -> GraphQL.Map
      var parseLiteral: @escaping (_ value: GraphQL.Value) throws -> GraphQL.Map
    }
    var Query: Query
    var Mutation: Mutation
    var DateTime: DateTime
  }
  struct source {
    struct source {
      struct Source {
        var a: @escaping (_: (source: SourceSpec.Source, args: (), context: Any, info: GraphQL.GraphQLResolveInfo)) async throws -> Int?
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