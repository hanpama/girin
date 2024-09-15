import GraphQL

public struct Runtime {
    public let schema: GraphQLSchema
    public struct Wiring {
        struct Basic {
            struct Definition {
                struct BasicObject {
                    var idNonNullArg: (_ source: SourceSpec.BasicObject, _ args: (idNonNull: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
                    var stringNonNullArg: (_ source: SourceSpec.BasicObject, _ args: (stringNonNull: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
                    var intNonNullArg: (_ source: SourceSpec.BasicObject, _ args: (intNonNull: Int, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> Int
                    var floatNonNullArg: (_ source: SourceSpec.BasicObject, _ args: (floatNonNull: Float, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> Float
                    var booleanNonNullArg: (_ source: SourceSpec.BasicObject, _ args: (booleanNonNull: Bool, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> Bool
                    var idArg: (_ source: SourceSpec.BasicObject, _ args: (id: String?, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String?
                    var stringArg: (_ source: SourceSpec.BasicObject, _ args: (string: String?, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String?
                    var intArg: (_ source: SourceSpec.BasicObject, _ args: (int: Int?, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> Int?
                    var floatArg: (_ source: SourceSpec.BasicObject, _ args: (float: Float?, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> Float?
                    var booleanArg: (_ source: SourceSpec.BasicObject, _ args: (boolean: Bool?, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> Bool?
                    var basicInputArg: (_ source: SourceSpec.BasicObject, _ args: (basicInput: SourceSpec.BasicInput?, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String?
                    var basicInputArgWithDefault: (_ source: SourceSpec.BasicObject, _ args: (basicInput: SourceSpec.BasicInput?, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String?
                }
                struct BasicInterface {
                }
                struct BasicInterfaceImpl {
                }
                var BasicObject: BasicObject
                var BasicInterface: BasicInterface
                var BasicInterfaceImpl: BasicInterfaceImpl
            }
            struct Extension {
                struct BasicObject {
                    var extendedFieldWithArg: (_ source: SourceSpec.BasicObject, _ args: (idNonNull: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
                }
                struct AnotherInterface {
                }
                struct BasicInterface {
                    var extendedFieldWithArg: (_ source: SourceSpec.BasicInterface, _ args: (idNonNull: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
                }
                struct BasicInterfaceImpl {
                    var extendedFieldWithArg: (_ source: SourceSpec.BasicInterfaceImpl, _ args: (idNonNull: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
                }
                struct AnotherType {
                }
                struct Query {
                    var extendedHello: (_ source: SourceSpec.Query, _ args: (name: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
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
                    var echo: (_ source: SourceSpec.GrandParent, _ args: (message: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
                    var echoSync: (_ source: SourceSpec.GrandParent, _ args: (message: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) throws -> String
                }
                struct Parent {
                    var echo: (_ source: SourceSpec.Parent, _ args: (message: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
                    var echoSync: (_ source: SourceSpec.Parent, _ args: (message: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
                }
                struct Child {
                    var echo: (_ source: SourceSpec.Child, _ args: (message: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
                    var echoSync: (_ source: SourceSpec.Child, _ args: (message: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> String
                    var basicInterface: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.BasicInterface?
                    var basicInterfaceList: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> [SourceSpec.BasicInterface?]?
                    var basicInterfaceNonNullList: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> [SourceSpec.BasicInterface?]
                    var basicInterfaceNonNullListNonNullElement: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> [SourceSpec.BasicInterface]
                    var basicUnion: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.BasicUnion?
                    var basicUnionList: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> [SourceSpec.BasicUnion?]?
                    var basicUnionNonNullList: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> [SourceSpec.BasicUnion?]
                    var basicUnionNonNullListNonNullElement: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> [SourceSpec.BasicUnion]
                    var basicEnum: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.BasicEnum?
                    var basicEnumList: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> [SourceSpec.BasicEnum?]?
                    var basicEnumNonNullList: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> [SourceSpec.BasicEnum?]
                    var basicEnumNonNullListNonNullElement: (_ source: SourceSpec.Child, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> [SourceSpec.BasicEnum]
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
            var Query: Query
            var Mutation: Mutation
        }
        struct source {
            struct source {
                struct Source {
                    var a: (_ source: SourceSpec.Source, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> Int?
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
    init(wiring: Wiring, encoder: GraphQL.MapEncoder, decoder: GraphQL.MapDecoder) {
        let BasicScalarDefinition = try! GraphQL.GraphQLScalarType(
            name: "BasicScalar",
            description: "BasicScalar description\n",
            serialize: { value in
                guard let value = value as? Encodable else {
                    throw GraphQLError(message: "BasicScalar cannot represent value: \(value)")
                }
                return try encoder.encode(value)
            }
        )
        let DateTimeDefinition = try! GraphQL.GraphQLScalarType(
            name: "DateTime",
            serialize: { value in
                guard let value = value as? Encodable else {
                    throw GraphQLError(message: "DateTime cannot represent value: \(value)")
                }
                return try encoder.encode(value)
            }
        )
        let BasicEnumDefinition = try! GraphQL.GraphQLEnumType(
            name: "BasicEnum",
            values: [
                "ENUM_VALUE_1": GraphQL.GraphQLEnumValue(
                    value: "ENUM_VALUE_1",
                    description: "ENUM_VALUE_1 description\n"
                ),
                "ENUM_VALUE_2": GraphQL.GraphQLEnumValue(
                    value: "ENUM_VALUE_2"
                ),
                "EXTENDED_VALUE": GraphQL.GraphQLEnumValue(
                    value: "EXTENDED_VALUE",
                    description: "EXTENDED_VALUE description\n"
                )
            ]
        )
        let BasicInputDefinition = try! GraphQL.GraphQLInputObjectType(
            name: "BasicInput",
            fields: [
                "idNonNull": .init(
                    type: GraphQLNonNull(GraphQLID),
                    description: "idNonNull description\n"
                ),
                "stringNonNull": .init(
                    type: GraphQLNonNull(GraphQLString)
                ),
                "intNonNull": .init(
                    type: GraphQLNonNull(GraphQLInt)
                ),
                "floatNonNull": .init(
                    type: GraphQLNonNull(GraphQLFloat)
                ),
                "booleanNonNull": .init(
                    type: GraphQLNonNull(GraphQLBoolean)
                ),
                "basicScalarNotNull": .init(
                    type: GraphQLNonNull(GraphQLTypeReference("BasicScalar"))
                ),
                "id": .init(
                    type: GraphQLID
                ),
                "string": .init(
                    type: GraphQLString
                ),
                "int": .init(
                    type: GraphQLInt
                ),
                "float": .init(
                    type: GraphQLFloat
                ),
                "boolean": .init(
                    type: GraphQLBoolean
                ),
                "basicScalar": .init(
                    type: GraphQLTypeReference("BasicScalar")
                ),
                "extendedField": .init(
                    type: GraphQLNonNull(GraphQLID),
                    description: "extendedField description\n"
                ),
            ]
        )
        let AnotherInterfaceDefinition = try! GraphQL.GraphQLInterfaceType(
            name: "AnotherInterface",
            fields: [
                "anotherField": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    resolve: nil
                )
            ]
        )
        let BasicInterfaceDefinition = try! GraphQL.GraphQLInterfaceType(
            name: "BasicInterface",
            fields: [
                "interfaceField": GraphQL.GraphQLField(
                    type: GraphQLString,
                    resolve: nil
                ),
                "basicScalar": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("BasicScalar"),
                    resolve: nil
                ),
                "extendedField": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    resolve: nil
                ),
                "extendedFieldWithArg": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    args: [
                        "idNonNull": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLID)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicInterface
                        let function = wiring.Basic.Extension.BasicInterface.extendedFieldWithArg
                        struct Args: Decodable {
                            var idNonNull: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.idNonNull, ()), context, info)
                        }
                    }
                ),
                "anotherField": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    resolve: nil
                )
            ]
        )
        let GrandParentDefinition = try! GraphQL.GraphQLInterfaceType(
            name: "GrandParent",
            fields: [
                "echo": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    args: [
                        "message": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLString)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.GrandParent
                        let function = wiring.Resolve.Schema.GrandParent.echo
                        struct Args: Decodable {
                            var message: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.message, ()), context, info)
                        }
                    }
                ),
                "echoSync": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    args: [
                        "message": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLString)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.GrandParent
                        let function = wiring.Resolve.Schema.GrandParent.echoSync
                        struct Args: Decodable {
                            var message: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeSucceededFuture(
                            try function(source, (args.message, ()), context, info)
                        )
                    }
                )
            ]
        )
        let ParentDefinition = try! GraphQL.GraphQLInterfaceType(
            name: "Parent",
            interfaces: [
                GrandParentDefinition,
            ],
            fields: [
                "echo": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    args: [
                        "message": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLString)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Parent
                        let function = wiring.Resolve.Schema.Parent.echo
                        struct Args: Decodable {
                            var message: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.message, ()), context, info)
                        }
                    }
                ),
                "echoSync": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    args: [
                        "message": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLString)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Parent
                        let function = wiring.Resolve.Schema.Parent.echoSync
                        struct Args: Decodable {
                            var message: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.message, ()), context, info)
                        }
                    }
                )
            ]
        )
        let BasicObjectDefinition = try! GraphQL.GraphQLObjectType(
            name: "BasicObject",
            description: "BasicObject description\n",
            fields: [
                "idNonNull": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    resolve: nil
                ),
                "stringNonNull": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    resolve: nil
                ),
                "intNonNull": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLInt),
                    resolve: nil
                ),
                "floatNonNull": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLFloat),
                    resolve: nil
                ),
                "booleanNonNull": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLBoolean),
                    resolve: nil
                ),
                "id": GraphQL.GraphQLField(
                    type: GraphQLID,
                    description: "id description\n",
                    resolve: nil
                ),
                "string": GraphQL.GraphQLField(
                    type: GraphQLString,
                    description: "string description\n",
                    resolve: nil
                ),
                "int": GraphQL.GraphQLField(
                    type: GraphQLInt,
                    description: "int description\n",
                    resolve: nil
                ),
                "float": GraphQL.GraphQLField(
                    type: GraphQLFloat,
                    description: "float description\n",
                    resolve: nil
                ),
                "boolean": GraphQL.GraphQLField(
                    type: GraphQLBoolean,
                    description: "boolean description\n",
                    resolve: nil
                ),
                "idNonNullArg": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    args: [
                        "idNonNull": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLID),
                            description: "idNonNull description\n",
                            defaultValue: .string("default")
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.idNonNullArg
                        struct Args: Decodable {
                            var idNonNull: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.idNonNull, ()), context, info)
                        }
                    }
                ),
                "stringNonNullArg": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    args: [
                        "stringNonNull": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLString),
                            defaultValue: .string("default")
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.stringNonNullArg
                        struct Args: Decodable {
                            var stringNonNull: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.stringNonNull, ()), context, info)
                        }
                    }
                ),
                "intNonNullArg": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLInt),
                    args: [
                        "intNonNull": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLInt),
                            defaultValue: .int(1)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.intNonNullArg
                        struct Args: Decodable {
                            var intNonNull: Int
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.intNonNull, ()), context, info)
                        }
                    }
                ),
                "floatNonNullArg": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLFloat),
                    args: [
                        "floatNonNull": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLFloat),
                            defaultValue: .double(1)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.floatNonNullArg
                        struct Args: Decodable {
                            var floatNonNull: Float
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.floatNonNull, ()), context, info)
                        }
                    }
                ),
                "booleanNonNullArg": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLBoolean),
                    args: [
                        "booleanNonNull": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLBoolean),
                            defaultValue: .bool(true)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.booleanNonNullArg
                        struct Args: Decodable {
                            var booleanNonNull: Bool
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.booleanNonNull, ()), context, info)
                        }
                    }
                ),
                "idArg": GraphQL.GraphQLField(
                    type: GraphQLID,
                    args: [
                        "id": GraphQL.GraphQLArgument(
                            type: GraphQLID
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.idArg
                        struct Args: Decodable {
                            var id: String?
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.id, ()), context, info)
                        }
                    }
                ),
                "stringArg": GraphQL.GraphQLField(
                    type: GraphQLString,
                    args: [
                        "string": GraphQL.GraphQLArgument(
                            type: GraphQLString
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.stringArg
                        struct Args: Decodable {
                            var string: String?
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.string, ()), context, info)
                        }
                    }
                ),
                "intArg": GraphQL.GraphQLField(
                    type: GraphQLInt,
                    args: [
                        "int": GraphQL.GraphQLArgument(
                            type: GraphQLInt
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.intArg
                        struct Args: Decodable {
                            var int: Int?
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.int, ()), context, info)
                        }
                    }
                ),
                "floatArg": GraphQL.GraphQLField(
                    type: GraphQLFloat,
                    args: [
                        "float": GraphQL.GraphQLArgument(
                            type: GraphQLFloat
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.floatArg
                        struct Args: Decodable {
                            var float: Float?
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.float, ()), context, info)
                        }
                    }
                ),
                "booleanArg": GraphQL.GraphQLField(
                    type: GraphQLBoolean,
                    args: [
                        "boolean": GraphQL.GraphQLArgument(
                            type: GraphQLBoolean
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.booleanArg
                        struct Args: Decodable {
                            var boolean: Bool?
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.boolean, ()), context, info)
                        }
                    }
                ),
                "basicScalar": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("BasicScalar"),
                    resolve: nil
                ),
                "basicInputArg": GraphQL.GraphQLField(
                    type: GraphQLString,
                    args: [
                        "basicInput": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("BasicInput")
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.basicInputArg
                        struct Args: Decodable {
                            var basicInput: SourceSpec.BasicInput?
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.basicInput, ()), context, info)
                        }
                    }
                ),
                "basicInputArgWithDefault": GraphQL.GraphQLField(
                    type: GraphQLString,
                    args: [
                        "basicInput": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("BasicInput"),
                            defaultValue: .dictionary(["basicScalar": .string("123"), "boolean": .bool(true), "float": .double(1), "id": .string("default"), "int": .int(1), "string": .string("default")])
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Definition.BasicObject.basicInputArgWithDefault
                        struct Args: Decodable {
                            var basicInput: SourceSpec.BasicInput?
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.basicInput, ()), context, info)
                        }
                    }
                ),
                "extendedField": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    description: "extendedField description\n",
                    resolve: nil
                ),
                "extendedFieldWithArg": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    args: [
                        "idNonNull": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLID)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicObject
                        let function = wiring.Basic.Extension.BasicObject.extendedFieldWithArg
                        struct Args: Decodable {
                            var idNonNull: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.idNonNull, ()), context, info)
                        }
                    }
                )
            ]
        )
        let BasicInterfaceImplDefinition = try! GraphQL.GraphQLObjectType(
            name: "BasicInterfaceImpl",
            fields: [
                "interfaceField": GraphQL.GraphQLField(
                    type: GraphQLString,
                    resolve: nil
                ),
                "basicScalar": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("BasicScalar"),
                    resolve: nil
                ),
                "extendedField": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    description: "extendedField description\n",
                    resolve: nil
                ),
                "extendedFieldWithArg": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    args: [
                        "idNonNull": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLID)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.BasicInterfaceImpl
                        let function = wiring.Basic.Extension.BasicInterfaceImpl.extendedFieldWithArg
                        struct Args: Decodable {
                            var idNonNull: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.idNonNull, ()), context, info)
                        }
                    }
                ),
                "anotherField": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    resolve: nil
                )
            ],
            interfaces: [
                BasicInterfaceDefinition,
                AnotherInterfaceDefinition,
            ]
        )
        let AnotherTypeDefinition = try! GraphQL.GraphQLObjectType(
            name: "AnotherType",
            fields: [
                "anotherField": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    resolve: nil
                )
            ]
        )
        let DeprecatedFieldObjectDefinition = try! GraphQL.GraphQLObjectType(
            name: "DeprecatedFieldObject",
            fields: [
                "deprecatedField": GraphQL.GraphQLField(
                    type: GraphQLString,
                    deprecationReason: "No longer supported",
                    resolve: nil
                )
            ]
        )
        let ModuleBDefinition = try! GraphQL.GraphQLObjectType(
            name: "ModuleB",
            fields: [
                "id": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    resolve: nil
                ),
                "name": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    resolve: nil
                )
            ]
        )
        let ModuleADefinition = try! GraphQL.GraphQLObjectType(
            name: "ModuleA",
            fields: [
                "id": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    resolve: nil
                ),
                "name": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    resolve: nil
                )
            ]
        )
        let Nested2Definition = try! GraphQL.GraphQLObjectType(
            name: "Nested2",
            fields: [
                "value": GraphQL.GraphQLField(
                    type: GraphQLString,
                    resolve: nil
                )
            ]
        )
        let Nested1Definition = try! GraphQL.GraphQLObjectType(
            name: "Nested1",
            fields: [
                "nested2": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("Nested2"),
                    resolve: nil
                )
            ]
        )
        let ChildDefinition = try! GraphQL.GraphQLObjectType(
            name: "Child",
            fields: [
                "echo": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    args: [
                        "message": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLString)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.echo
                        struct Args: Decodable {
                            var message: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.message, ()), context, info)
                        }
                    }
                ),
                "echoSync": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    args: [
                        "message": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLString)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.echoSync
                        struct Args: Decodable {
                            var message: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.message, ()), context, info)
                        }
                    }
                ),
                "basicInterface": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("BasicInterface"),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicInterface
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicInterfaceList": GraphQL.GraphQLField(
                    type: GraphQLList(GraphQLTypeReference("BasicInterface")),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicInterfaceList
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicInterfaceNonNullList": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicInterface"))),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicInterfaceNonNullList
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicInterfaceNonNullListNonNullElement": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicInterface")))),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicInterfaceNonNullListNonNullElement
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicUnion": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("BasicUnion"),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicUnion
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicUnionList": GraphQL.GraphQLField(
                    type: GraphQLList(GraphQLTypeReference("BasicUnion")),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicUnionList
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicUnionNonNullList": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicUnion"))),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicUnionNonNullList
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicUnionNonNullListNonNullElement": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicUnion")))),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicUnionNonNullListNonNullElement
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicEnum": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("BasicEnum"),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicEnum
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicEnumList": GraphQL.GraphQLField(
                    type: GraphQLList(GraphQLTypeReference("BasicEnum")),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicEnumList
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicEnumNonNullList": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicEnum"))),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicEnumNonNullList
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                ),
                "basicEnumNonNullListNonNullElement": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicEnum")))),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Child
                        let function = wiring.Resolve.Schema.Child.basicEnumNonNullListNonNullElement
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                )
            ],
            interfaces: [
                GrandParentDefinition,
                ParentDefinition,
            ]
        )
        let GraphQLObjectDefinition = try! GraphQL.GraphQLObjectType(
            name: "GraphQLObject",
            fields: [
                "foo": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    resolve: nil
                )
            ]
        )
        let graphqlDefinition = try! GraphQL.GraphQLObjectType(
            name: "graphql",
            fields: [
                "bar": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    resolve: nil
                )
            ]
        )
        let typingDefinition = try! GraphQL.GraphQLObjectType(
            name: "typing",
            fields: [
                "baz": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    resolve: nil
                )
            ]
        )
        let QueryDefinition = try! GraphQL.GraphQLObjectType(
            name: "Query",
            fields: [
                "version": GraphQL.GraphQLField(
                    type: GraphQLString,
                    resolve: nil
                ),
                "extendedHello": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    args: [
                        "name": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLString)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Query
                        let function = wiring.Basic.Extension.Query.extendedHello
                        struct Args: Decodable {
                            var name: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.name, ()), context, info)
                        }
                    }
                )
            ]
        )
        let MutationDefinition = try! GraphQL.GraphQLObjectType(
            name: "Mutation",
            fields: [
                "version": GraphQL.GraphQLField(
                    type: GraphQLString,
                    resolve: nil
                )
            ]
        )
        let SourceDefinition = try! GraphQL.GraphQLObjectType(
            name: "Source",
            fields: [
                "a": GraphQL.GraphQLField(
                    type: GraphQLInt,
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Source
                        let function = wiring.source.source.Source.a
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (()), context, info)
                        }
                    }
                )
            ]
        )
        let BasicUnionDefinition = try! GraphQL.GraphQLUnionType(
            name: "BasicUnion",
            types: [
                BasicObjectDefinition,
                BasicInterfaceImplDefinition,
                AnotherTypeDefinition,
            ]
        )
        self.schema = try! GraphQL.GraphQLSchema(
            query: QueryDefinition,
            mutation: MutationDefinition,
            types: [
                BasicObjectDefinition,
                BasicInterfaceDefinition,
                BasicInterfaceImplDefinition,
                BasicUnionDefinition,
                BasicEnumDefinition,
                BasicInputDefinition,
                BasicScalarDefinition,
                AnotherInterfaceDefinition,
                AnotherTypeDefinition,
                DeprecatedFieldObjectDefinition,
                ModuleBDefinition,
                ModuleADefinition,
                Nested2Definition,
                Nested1Definition,
                GrandParentDefinition,
                ParentDefinition,
                ChildDefinition,
                GraphQLObjectDefinition,
                graphqlDefinition,
                typingDefinition,
                QueryDefinition,
                MutationDefinition,
                DateTimeDefinition,
                SourceDefinition,
            ]
        )
    }
}
