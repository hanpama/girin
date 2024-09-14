import GraphQL

func buildSchema(config: RuntimeSpec) throws -> GraphQLSchema {
    lazy var BasicScalarDefinition = try! GraphQL.GraphQLScalarType(
        name: "BasicScalar",
        description: "BasicScalar description\n",
        serialize: config.Basic.Definition.BasicScalar.serialize,
        parseValue: config.Basic.Definition.BasicScalar.parseValue,
        parseLiteral: config.Basic.Definition.BasicScalar.parseLiteral
    )
    lazy var DateTimeDefinition = try! GraphQL.GraphQLScalarType(
        name: "DateTime",
        serialize: config.root.DateTime.serialize,
        parseValue: config.root.DateTime.parseValue,
        parseLiteral: config.root.DateTime.parseLiteral
    )
    lazy var BasicEnumDefinition = try! GraphQL.GraphQLEnumType(
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
    lazy var BasicInputDefinition = try! GraphQL.GraphQLInputObjectType(
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
    lazy var AnotherInterfaceDefinition = try! GraphQL.GraphQLInterfaceType(
        name: "AnotherInterface",
        fields: [
            "anotherField": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLID),
                resolve: nil
            )
        ]
    )
    lazy var BasicInterfaceDefinition = try! GraphQL.GraphQLInterfaceType(
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
                    let function = config.Basic.Extension.BasicInterface.extendedFieldWithArg
                    struct Args: Decodable {
                        var idNonNull: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
    lazy var GrandParentDefinition = try! GraphQL.GraphQLInterfaceType(
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
                    let function = config.Resolve.Schema.GrandParent.echo
                    struct Args: Decodable {
                        var message: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                    let function = config.Resolve.Schema.GrandParent.echoSync
                    struct Args: Decodable {
                        var message: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
                    return eventLoopGroup.next().makeSucceededFuture(
                        try function(source, (args.message, ()), context, info)
                    )
                }
            )
        ]
    )
    lazy var ParentDefinition = try! GraphQL.GraphQLInterfaceType(
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
                    let function = config.Resolve.Schema.Parent.echo
                    struct Args: Decodable {
                        var message: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                    let function = config.Resolve.Schema.Parent.echoSync
                    struct Args: Decodable {
                        var message: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (args.message, ()), context, info)
                    }
                }
            )
        ]
    )
    lazy var BasicObjectDefinition = try! GraphQL.GraphQLObjectType(
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
                        defaultValue: Map.string("default")
                    ),
                ],
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.BasicObject
                    let function = config.Basic.Definition.BasicObject.idNonNullArg
                    struct Args: Decodable {
                        var idNonNull: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                        defaultValue: Map.string("default")
                    ),
                ],
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.BasicObject
                    let function = config.Basic.Definition.BasicObject.stringNonNullArg
                    struct Args: Decodable {
                        var stringNonNull: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                        defaultValue: Map.int(1)
                    ),
                ],
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.BasicObject
                    let function = config.Basic.Definition.BasicObject.intNonNullArg
                    struct Args: Decodable {
                        var intNonNull: Int
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                        defaultValue: Map.double(1)
                    ),
                ],
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.BasicObject
                    let function = config.Basic.Definition.BasicObject.floatNonNullArg
                    struct Args: Decodable {
                        var floatNonNull: Float
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                        defaultValue: Map.bool(true)
                    ),
                ],
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.BasicObject
                    let function = config.Basic.Definition.BasicObject.booleanNonNullArg
                    struct Args: Decodable {
                        var booleanNonNull: Bool
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                    let function = config.Basic.Definition.BasicObject.idArg
                    struct Args: Decodable {
                        var id: String?
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                    let function = config.Basic.Definition.BasicObject.stringArg
                    struct Args: Decodable {
                        var string: String?
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                    let function = config.Basic.Definition.BasicObject.intArg
                    struct Args: Decodable {
                        var int: Int?
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                    let function = config.Basic.Definition.BasicObject.floatArg
                    struct Args: Decodable {
                        var float: Float?
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                    let function = config.Basic.Definition.BasicObject.booleanArg
                    struct Args: Decodable {
                        var boolean: Bool?
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                    let function = config.Basic.Definition.BasicObject.basicInputArg
                    struct Args: Decodable {
                        var basicInput: SourceSpec.BasicInput?
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                        defaultValue: .dictionary(["basicScalar": Map.string("123"), "boolean": Map.bool(true), "float": Map.double(1), "id": Map.string("default"), "int": Map.int(1), "string": Map.string("default")])
                    ),
                ],
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.BasicObject
                    let function = config.Basic.Definition.BasicObject.basicInputArgWithDefault
                    struct Args: Decodable {
                        var basicInput: SourceSpec.BasicInput?
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                    let function = config.Basic.Extension.BasicObject.extendedFieldWithArg
                    struct Args: Decodable {
                        var idNonNull: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (args.idNonNull, ()), context, info)
                    }
                }
            )
        ]
    )
    lazy var BasicInterfaceImplDefinition = try! GraphQL.GraphQLObjectType(
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
                    let function = config.Basic.Extension.BasicInterfaceImpl.extendedFieldWithArg
                    struct Args: Decodable {
                        var idNonNull: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
    lazy var AnotherTypeDefinition = try! GraphQL.GraphQLObjectType(
        name: "AnotherType",
        fields: [
            "anotherField": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLID),
                resolve: nil
            )
        ]
    )
    lazy var DeprecatedFieldObjectDefinition = try! GraphQL.GraphQLObjectType(
        name: "DeprecatedFieldObject",
        fields: [
            "deprecatedField": GraphQL.GraphQLField(
                type: GraphQLString,
                deprecationReason: "No longer supported",
                resolve: nil
            )
        ]
    )
    lazy var ModuleBDefinition = try! GraphQL.GraphQLObjectType(
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
    lazy var ModuleADefinition = try! GraphQL.GraphQLObjectType(
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
    lazy var Nested2Definition = try! GraphQL.GraphQLObjectType(
        name: "Nested2",
        fields: [
            "value": GraphQL.GraphQLField(
                type: GraphQLString,
                resolve: nil
            )
        ]
    )
    lazy var Nested1Definition = try! GraphQL.GraphQLObjectType(
        name: "Nested1",
        fields: [
            "nested2": GraphQL.GraphQLField(
                type: GraphQLTypeReference("Nested2"),
                resolve: nil
            )
        ]
    )
    lazy var ChildDefinition = try! GraphQL.GraphQLObjectType(
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
                    let function = config.Resolve.Schema.Child.echo
                    struct Args: Decodable {
                        var message: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
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
                    let function = config.Resolve.Schema.Child.echoSync
                    struct Args: Decodable {
                        var message: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (args.message, ()), context, info)
                    }
                }
            ),
            "basicInterface": GraphQL.GraphQLField(
                type: GraphQLTypeReference("BasicInterface"),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicInterface
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicInterfaceList": GraphQL.GraphQLField(
                type: GraphQLList(GraphQLTypeReference("BasicInterface")),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicInterfaceList
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicInterfaceNonNullList": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicInterface"))),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicInterfaceNonNullList
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicInterfaceNonNullListNonNullElement": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicInterface")))),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicInterfaceNonNullListNonNullElement
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicUnion": GraphQL.GraphQLField(
                type: GraphQLTypeReference("BasicUnion"),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicUnion
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicUnionList": GraphQL.GraphQLField(
                type: GraphQLList(GraphQLTypeReference("BasicUnion")),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicUnionList
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicUnionNonNullList": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicUnion"))),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicUnionNonNullList
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicUnionNonNullListNonNullElement": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicUnion")))),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicUnionNonNullListNonNullElement
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicEnum": GraphQL.GraphQLField(
                type: GraphQLTypeReference("BasicEnum"),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicEnum
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicEnumList": GraphQL.GraphQLField(
                type: GraphQLList(GraphQLTypeReference("BasicEnum")),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicEnumList
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicEnumNonNullList": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicEnum"))),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicEnumNonNullList
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            ),
            "basicEnumNonNullListNonNullElement": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicEnum")))),
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Child
                    let function = config.Resolve.Schema.Child.basicEnumNonNullListNonNullElement
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
    lazy var GraphQLObjectDefinition = try! GraphQL.GraphQLObjectType(
        name: "GraphQLObject",
        fields: [
            "foo": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLString),
                resolve: nil
            )
        ]
    )
    lazy var graphqlDefinition = try! GraphQL.GraphQLObjectType(
        name: "graphql",
        fields: [
            "bar": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLString),
                resolve: nil
            )
        ]
    )
    lazy var typingDefinition = try! GraphQL.GraphQLObjectType(
        name: "typing",
        fields: [
            "baz": GraphQL.GraphQLField(
                type: GraphQLNonNull(GraphQLString),
                resolve: nil
            )
        ]
    )
    lazy var QueryDefinition = try! GraphQL.GraphQLObjectType(
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
                    let function = config.Basic.Extension.Query.extendedHello
                    struct Args: Decodable {
                        var name: String
                    }
                    let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (args.name, ()), context, info)
                    }
                }
            )
        ]
    )
    lazy var MutationDefinition = try! GraphQL.GraphQLObjectType(
        name: "Mutation",
        fields: [
            "version": GraphQL.GraphQLField(
                type: GraphQLString,
                resolve: nil
            )
        ]
    )
    lazy var SourceDefinition = try! GraphQL.GraphQLObjectType(
        name: "Source",
        fields: [
            "a": GraphQL.GraphQLField(
                type: GraphQLInt,
                resolve: { source, args, context, eventLoopGroup, info in
                    let source = source as! SourceSpec.Source
                    let function = config.source.source.Source.a
                    return eventLoopGroup.next().makeFutureWithTask {
                        return try await function(source, (()), context, info)
                    }
                }
            )
        ]
    )
    lazy var BasicUnionDefinition = try! GraphQL.GraphQLUnionType(
        name: "BasicUnion",
        types: [
            BasicObjectDefinition,
            BasicInterfaceImplDefinition,
            AnotherTypeDefinition,
        ]
    )
    return try! GraphQL.GraphQLSchema(
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
