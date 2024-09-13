import GraphQL

struct RuntimeSpec {
  struct Basic {
    struct Definition {
      struct BasicObject {
        var idNonNullArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (idNonNull: Any, _: ()))) async throws -> Any
        var stringNonNullArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (stringNonNull: String, _: ()))) async throws -> String
        var intNonNullArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (intNonNull: Int, _: ()))) async throws -> Int
        var floatNonNullArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (floatNonNull: Float, _: ()))) async throws -> Float
        var booleanNonNullArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (booleanNonNull: Bool, _: ()))) async throws -> Bool
        var idArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (id: Any?, _: ()))) async throws -> Any?
        var stringArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (string: String?, _: ()))) async throws -> String?
        var intArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (int: Int?, _: ()))) async throws -> Int?
        var floatArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (float: Float?, _: ()))) async throws -> Float?
        var booleanArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (boolean: Bool?, _: ()))) async throws -> Bool?
        var basicInputArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (basicInput: SourceSpec.BasicInput?, _: ()))) async throws -> String?
        var basicInputArgWithDefault: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (basicInput: SourceSpec.BasicInput?, _: ()))) async throws -> String?
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
        var extendedFieldWithArg: (_: (src: SourceSpec.BasicObject, info: GraphQL.GraphQLResolveInfo, args: (idNonNull: Any, _: ()))) async throws -> Any
      }
      struct AnotherInterface {
      }
      struct BasicInterface {
        var extendedFieldWithArg: (_: (src: SourceSpec.BasicInterface, info: GraphQL.GraphQLResolveInfo, args: (idNonNull: Any, _: ()))) async throws -> Any
      }
      struct BasicInterfaceImpl {
        var extendedFieldWithArg: (_: (src: SourceSpec.BasicInterfaceImpl, info: GraphQL.GraphQLResolveInfo, args: (idNonNull: Any, _: ()))) async throws -> Any
      }
      struct AnotherType {
      }
      struct Query {
        var extendedHello: (_: (src: SourceSpec.Query, info: GraphQL.GraphQLResolveInfo, args: (name: String, _: ()))) async throws -> String
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
        var echo: (_: (src: SourceSpec.GrandParent, info: GraphQL.GraphQLResolveInfo, args: (message: String, _: ()))) async throws -> String
        var echoSync: (_: (src: SourceSpec.GrandParent, info: GraphQL.GraphQLResolveInfo, args: (message: String, _: ()))) throws -> String
      }
      struct Parent {
        var echo: (_: (src: SourceSpec.Parent, info: GraphQL.GraphQLResolveInfo, args: (message: String, _: ()))) async throws -> String
        var echoSync: (_: (src: SourceSpec.Parent, info: GraphQL.GraphQLResolveInfo, args: (message: String, _: ()))) async throws -> String
      }
      struct Child {
        var echo: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: (message: String, _: ()))) async throws -> String
        var echoSync: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: (message: String, _: ()))) async throws -> String
        var basicInterface: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> SourceSpec.BasicInterface?
        var basicInterfaceList: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> [SourceSpec.BasicInterface?]?
        var basicInterfaceNonNullList: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> [SourceSpec.BasicInterface?]
        var basicInterfaceNonNullListNonNullElement: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> [SourceSpec.BasicInterface]
        var basicUnion: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> SourceSpec.BasicUnion?
        var basicUnionList: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> [SourceSpec.BasicUnion?]?
        var basicUnionNonNullList: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> [SourceSpec.BasicUnion?]
        var basicUnionNonNullListNonNullElement: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> [SourceSpec.BasicUnion]
        var basicEnum: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> SourceSpec.BasicEnum?
        var basicEnumList: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> [SourceSpec.BasicEnum?]?
        var basicEnumNonNullList: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> [SourceSpec.BasicEnum?]
        var basicEnumNonNullListNonNullElement: (_: (src: SourceSpec.Child, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> [SourceSpec.BasicEnum]
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
        var a: (_: (src: SourceSpec.Source, info: GraphQL.GraphQLResolveInfo, args: ())) async throws -> Int?
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