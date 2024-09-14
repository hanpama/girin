import GraphQL

func buildSchema(config: RuntimeSpec) throws -> GraphQLSchema {
  struct Types {
    let config: RuntimeSpec
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
              defaultValue: "default"
            ),
          ],
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
              var idNonNull: Any
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.idNonNullArg((source, (args.idNonNull, ()), context, info))
            }
          }
        ),
        "stringNonNullArg": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLString),
          args: [
            "stringNonNull": GraphQL.GraphQLArgument(
              type: GraphQLNonNull(GraphQLString),
              defaultValue: "default"
            ),
          ],
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
              var stringNonNull: String
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.stringNonNullArg((source, (args.stringNonNull, ()), context, info))
            }
          }
        ),
        "intNonNullArg": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLInt),
          args: [
            "intNonNull": GraphQL.GraphQLArgument(
              type: GraphQLNonNull(GraphQLInt),
              defaultValue: 1
            ),
          ],
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
              var intNonNull: Int
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.intNonNullArg((source, (args.intNonNull, ()), context, info))
            }
          }
        ),
        "floatNonNullArg": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLFloat),
          args: [
            "floatNonNull": GraphQL.GraphQLArgument(
              type: GraphQLNonNull(GraphQLFloat),
              defaultValue: 1
            ),
          ],
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
              var floatNonNull: Float
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.floatNonNullArg((source, (args.floatNonNull, ()), context, info))
            }
          }
        ),
        "booleanNonNullArg": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLBoolean),
          args: [
            "booleanNonNull": GraphQL.GraphQLArgument(
              type: GraphQLNonNull(GraphQLBoolean),
              defaultValue: true
            ),
          ],
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
              var booleanNonNull: Bool
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.booleanNonNullArg((source, (args.booleanNonNull, ()), context, info))
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
            struct Args: Decodable {
              var id: Any?
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.idArg((source, (args.id, ()), context, info))
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
            struct Args: Decodable {
              var string: String?
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.stringArg((source, (args.string, ()), context, info))
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
            struct Args: Decodable {
              var int: Int?
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.intArg((source, (args.int, ()), context, info))
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
            struct Args: Decodable {
              var float: Float?
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.floatArg((source, (args.float, ()), context, info))
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
            struct Args: Decodable {
              var boolean: Bool?
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.booleanArg((source, (args.boolean, ()), context, info))
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
            struct Args: Decodable {
              var basicInput: BasicInput?
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.basicInputArg((source, (args.basicInput, ()), context, info))
            }
          }
        ),
        "basicInputArgWithDefault": GraphQL.GraphQLField(
          type: GraphQLString,
          args: [
            "basicInput": GraphQL.GraphQLArgument(
              type: GraphQLTypeReference("BasicInput"),
              defaultValue: ["basicScalar": "123", "boolean": true, "float": 1, "id": "default", "int": 1, "string": "default"]
            ),
          ],
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
              var basicInput: BasicInput?
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Definition.BasicObject.basicInputArgWithDefault((source, (args.basicInput, ()), context, info))
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
            struct Args: Decodable {
              var idNonNull: Any
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Extension.BasicObject.extendedFieldWithArg((source, (args.idNonNull, ()), context, info))
            }
          }
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
            struct Args: Decodable {
              var idNonNull: Any
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Extension.BasicInterface.extendedFieldWithArg((source, (args.idNonNull, ()), context, info))
            }
          }
        ),
        "anotherField": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLID),
          resolve: nil
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
            struct Args: Decodable {
              var idNonNull: Any
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Extension.BasicInterfaceImpl.extendedFieldWithArg((source, (args.idNonNull, ()), context, info))
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
            struct Args: Decodable {
              var message: String
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.GrandParent.echo((source, (args.message, ()), context, info))
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
            struct Args: Decodable {
              var message: String
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeSucceededFuture(
              try config.Resolve.Schema.GrandParent.echoSync((source, (args.message, ()), context, info))
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
            struct Args: Decodable {
              var message: String
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Parent.echo((source, (args.message, ()), context, info))
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
            struct Args: Decodable {
              var message: String
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Parent.echoSync((source, (args.message, ()), context, info))
            }
          }
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
            struct Args: Decodable {
              var message: String
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.echo((source, (args.message, ()), context, info))
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
            struct Args: Decodable {
              var message: String
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.echoSync((source, (args.message, ()), context, info))
            }
          }
        ),
        "basicInterface": GraphQL.GraphQLField(
          type: GraphQLTypeReference("BasicInterface"),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicInterface((source, (()), context, info))
            }
          }
        ),
        "basicInterfaceList": GraphQL.GraphQLField(
          type: GraphQLList(GraphQLTypeReference("BasicInterface")),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicInterfaceList((source, (()), context, info))
            }
          }
        ),
        "basicInterfaceNonNullList": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicInterface"))),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicInterfaceNonNullList((source, (()), context, info))
            }
          }
        ),
        "basicInterfaceNonNullListNonNullElement": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicInterface")))),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicInterfaceNonNullListNonNullElement((source, (()), context, info))
            }
          }
        ),
        "basicUnion": GraphQL.GraphQLField(
          type: GraphQLTypeReference("BasicUnion"),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicUnion((source, (()), context, info))
            }
          }
        ),
        "basicUnionList": GraphQL.GraphQLField(
          type: GraphQLList(GraphQLTypeReference("BasicUnion")),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicUnionList((source, (()), context, info))
            }
          }
        ),
        "basicUnionNonNullList": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicUnion"))),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicUnionNonNullList((source, (()), context, info))
            }
          }
        ),
        "basicUnionNonNullListNonNullElement": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicUnion")))),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicUnionNonNullListNonNullElement((source, (()), context, info))
            }
          }
        ),
        "basicEnum": GraphQL.GraphQLField(
          type: GraphQLTypeReference("BasicEnum"),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicEnum((source, (()), context, info))
            }
          }
        ),
        "basicEnumList": GraphQL.GraphQLField(
          type: GraphQLList(GraphQLTypeReference("BasicEnum")),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicEnumList((source, (()), context, info))
            }
          }
        ),
        "basicEnumNonNullList": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicEnum"))),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicEnumNonNullList((source, (()), context, info))
            }
          }
        ),
        "basicEnumNonNullListNonNullElement": GraphQL.GraphQLField(
          type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicEnum")))),
          resolve: { source, args, context, eventLoopGroup, info in
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Resolve.Schema.Child.basicEnumNonNullListNonNullElement((source, (()), context, info))
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
            struct Args: Decodable {
              var name: String
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.Basic.Extension.Query.extendedHello((source, (args.name, ()), context, info))
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
            struct Args: Decodable {
            }
            let args: Args = try GraphQL.MapDecoder().decode(Args.self, from: args)
            return eventLoopGroup.next().makeFutureWithTask {
              return try config.source.source.Source.a((source, (()), context, info))
            }
          }
        )
      ]
    )
  }
  let types = Types(config: config)
  return GraphQL.GraphQLSchema(
    query: types.QueryDefinition,
    mutation: types.MutationDefinition,
    types: [
      types.BasicObjectDefinition,
      types.BasicInterfaceDefinition,
      types.BasicInterfaceImplDefinition,
      types.BasicUnionDefinition,
      types.BasicEnumDefinition,
      types.BasicInputDefinition,
      types.BasicScalarDefinition,
      types.AnotherInterfaceDefinition,
      types.AnotherTypeDefinition,
      types.DeprecatedFieldObjectDefinition,
      types.ModuleBDefinition,
      types.ModuleADefinition,
      types.Nested2Definition,
      types.Nested1Definition,
      types.GrandParentDefinition,
      types.ParentDefinition,
      types.ChildDefinition,
      types.GraphQLObjectDefinition,
      types.graphqlDefinition,
      types.typingDefinition,
      types.QueryDefinition,
      types.MutationDefinition,
      types.DateTimeDefinition,
      types.SourceDefinition,
    ]
  )
}