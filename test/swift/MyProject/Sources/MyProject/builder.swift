import GraphQL

func buildSchema(config: BuilderConfig) throws -> GraphQLSchema {
}
fileprivate struct TypeRegistry {
  let config: BuilderConfig
  lazy var BasicObjectDefinition = try! GraphQLObjectType(
    name: "BasicObject",
    description: "BasicObject description\n",
    fields: [
      "idNonNull": .init(
        type: GraphQLNonNull(GraphQLID),
        resolve: nil
      ),
      "stringNonNull": .init(
        type: GraphQLNonNull(GraphQLString),
        resolve: nil
      ),
      "intNonNull": .init(
        type: GraphQLNonNull(GraphQLInt),
        resolve: nil
      ),
      "floatNonNull": .init(
        type: GraphQLNonNull(GraphQLFloat),
        resolve: nil
      ),
      "booleanNonNull": .init(
        type: GraphQLNonNull(GraphQLBoolean),
        resolve: nil
      ),
      "id": .init(
        type: GraphQLID,
        description: "id description\n",
        resolve: nil
      ),
      "string": .init(
        type: GraphQLString,
        description: "string description\n",
        resolve: nil
      ),
      "int": .init(
        type: GraphQLInt,
        description: "int description\n",
        resolve: nil
      ),
      "float": .init(
        type: GraphQLFloat,
        description: "float description\n",
        resolve: nil
      ),
      "boolean": .init(
        type: GraphQLBoolean,
        description: "boolean description\n",
        resolve: nil
      ),
      "idNonNullArg": .init(
        type: GraphQLNonNull(GraphQLID),
        args: [
          "idNonNull": .init(
            type: GraphQLNonNull(GraphQLID),
            description: "idNonNull description\n",
            defaultValue: "default"
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.idNonNullArg
      ),
      "stringNonNullArg": .init(
        type: GraphQLNonNull(GraphQLString),
        args: [
          "stringNonNull": .init(
            type: GraphQLNonNull(GraphQLString),
            defaultValue: "default"
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.stringNonNullArg
      ),
      "intNonNullArg": .init(
        type: GraphQLNonNull(GraphQLInt),
        args: [
          "intNonNull": .init(
            type: GraphQLNonNull(GraphQLInt),
            defaultValue: 1
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.intNonNullArg
      ),
      "floatNonNullArg": .init(
        type: GraphQLNonNull(GraphQLFloat),
        args: [
          "floatNonNull": .init(
            type: GraphQLNonNull(GraphQLFloat),
            defaultValue: 1
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.floatNonNullArg
      ),
      "booleanNonNullArg": .init(
        type: GraphQLNonNull(GraphQLBoolean),
        args: [
          "booleanNonNull": .init(
            type: GraphQLNonNull(GraphQLBoolean),
            defaultValue: true
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.booleanNonNullArg
      ),
      "idArg": .init(
        type: GraphQLID,
        args: [
          "id": .init(
            type: GraphQLID
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.idArg
      ),
      "stringArg": .init(
        type: GraphQLString,
        args: [
          "string": .init(
            type: GraphQLString
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.stringArg
      ),
      "intArg": .init(
        type: GraphQLInt,
        args: [
          "int": .init(
            type: GraphQLInt
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.intArg
      ),
      "floatArg": .init(
        type: GraphQLFloat,
        args: [
          "float": .init(
            type: GraphQLFloat
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.floatArg
      ),
      "booleanArg": .init(
        type: GraphQLBoolean,
        args: [
          "boolean": .init(
            type: GraphQLBoolean
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.booleanArg
      ),
      "basicScalar": .init(
        type: GraphQLTypeReference("BasicScalar"),
        resolve: nil
      ),
      "basicInputArg": .init(
        type: GraphQLString,
        args: [
          "basicInput": .init(
            type: GraphQLTypeReference("BasicInput")
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.basicInputArg
      ),
      "basicInputArgWithDefault": .init(
        type: GraphQLString,
        args: [
          "basicInput": .init(
            type: GraphQLTypeReference("BasicInput"),
            defaultValue: ["basicScalar": "123", "boolean": true, "float": 1, "id": "default", "int": 1, "string": "default"]
          ),
        ],
        resolve: config.Basic.Definition.BasicObject.basicInputArgWithDefault
      ),
      "extendedField": .init(
        type: GraphQLNonNull(GraphQLID),
        description: "extendedField description\n",
        resolve: nil
      ),
      "extendedFieldWithArg": .init(
        type: GraphQLNonNull(GraphQLID),
        args: [
          "idNonNull": .init(
            type: GraphQLNonNull(GraphQLID)
          ),
        ],
        resolve: config.Basic.Extension.BasicObject.extendedFieldWithArg
      ),
    ]
  )
  lazy var BasicInterfaceDefinition = try! GraphQLInterfaceType(
    name: "BasicInterface",
    fields: [
      "interfaceField": .init(
        type: GraphQLString,
        resolve: nil
      ),
      "basicScalar": .init(
        type: GraphQLTypeReference("BasicScalar"),
        resolve: nil
      ),
      "extendedField": .init(
        type: GraphQLNonNull(GraphQLID),
        resolve: nil
      ),
      "extendedFieldWithArg": .init(
        type: GraphQLNonNull(GraphQLID),
        args: [
          "idNonNull": .init(
            type: GraphQLNonNull(GraphQLID)
          ),
        ],
        resolve: config.Basic.Extension.BasicInterface.extendedFieldWithArg
      ),
      "anotherField": .init(
        type: GraphQLNonNull(GraphQLID),
        resolve: nil
      ),
    ]
  )
  lazy var BasicInterfaceImplDefinition = try! GraphQLObjectType(
    name: "BasicInterfaceImpl",
    fields: [
      "interfaceField": .init(
        type: GraphQLString,
        resolve: nil
      ),
      "basicScalar": .init(
        type: GraphQLTypeReference("BasicScalar"),
        resolve: nil
      ),
      "extendedField": .init(
        type: GraphQLNonNull(GraphQLID),
        description: "extendedField description\n",
        resolve: nil
      ),
      "extendedFieldWithArg": .init(
        type: GraphQLNonNull(GraphQLID),
        args: [
          "idNonNull": .init(
            type: GraphQLNonNull(GraphQLID)
          ),
        ],
        resolve: config.Basic.Extension.BasicInterfaceImpl.extendedFieldWithArg
      ),
      "anotherField": .init(
        type: GraphQLNonNull(GraphQLID),
        resolve: nil
      ),
    ],
    interfaces: [
      BasicInterfaceDefinition,
      AnotherInterfaceDefinition,
    ]
  )
  lazy var BasicInputDefinition = try! GraphQLInputObjectType(
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
  lazy var AnotherInterfaceDefinition = try! GraphQLInterfaceType(
    name: "AnotherInterface",
    fields: [
      "anotherField": .init(
        type: GraphQLNonNull(GraphQLID),
        resolve: nil
      ),
    ]
  )
  lazy var AnotherTypeDefinition = try! GraphQLObjectType(
    name: "AnotherType",
    fields: [
      "anotherField": .init(
        type: GraphQLNonNull(GraphQLID),
        resolve: nil
      ),
    ]
  )
  lazy var DeprecatedFieldObjectDefinition = try! GraphQLObjectType(
    name: "DeprecatedFieldObject",
    fields: [
      "deprecatedField": .init(
        type: GraphQLString,
        deprecationReason: "No longer supported",
        resolve: nil
      ),
    ]
  )
  lazy var ModuleBDefinition = try! GraphQLObjectType(
    name: "ModuleB",
    fields: [
      "id": .init(
        type: GraphQLNonNull(GraphQLID),
        resolve: nil
      ),
      "name": .init(
        type: GraphQLNonNull(GraphQLString),
        resolve: nil
      ),
    ]
  )
  lazy var ModuleADefinition = try! GraphQLObjectType(
    name: "ModuleA",
    fields: [
      "id": .init(
        type: GraphQLNonNull(GraphQLID),
        resolve: nil
      ),
      "name": .init(
        type: GraphQLNonNull(GraphQLString),
        resolve: nil
      ),
    ]
  )
  lazy var Nested2Definition = try! GraphQLObjectType(
    name: "Nested2",
    fields: [
      "value": .init(
        type: GraphQLString,
        resolve: nil
      ),
    ]
  )
  lazy var Nested1Definition = try! GraphQLObjectType(
    name: "Nested1",
    fields: [
      "nested2": .init(
        type: GraphQLTypeReference("Nested2"),
        resolve: nil
      ),
    ]
  )
  lazy var GrandParentDefinition = try! GraphQLInterfaceType(
    name: "GrandParent",
    fields: [
      "echo": .init(
        type: GraphQLNonNull(GraphQLString),
        args: [
          "message": .init(
            type: GraphQLNonNull(GraphQLString)
          ),
        ],
        resolve: config.Resolve.Schema.GrandParent.echo
      ),
      "echoSync": .init(
        type: GraphQLNonNull(GraphQLString),
        args: [
          "message": .init(
            type: GraphQLNonNull(GraphQLString)
          ),
        ],
        resolve: config.Resolve.Schema.GrandParent.echoSync
      ),
    ]
  )
  lazy var ParentDefinition = try! GraphQLInterfaceType(
    name: "Parent",
    interfaces: [
      GrandParentDefinition,
    ],
    fields: [
      "echo": .init(
        type: GraphQLNonNull(GraphQLString),
        args: [
          "message": .init(
            type: GraphQLNonNull(GraphQLString)
          ),
        ],
        resolve: config.Resolve.Schema.Parent.echo
      ),
      "echoSync": .init(
        type: GraphQLNonNull(GraphQLString),
        args: [
          "message": .init(
            type: GraphQLNonNull(GraphQLString)
          ),
        ],
        resolve: config.Resolve.Schema.Parent.echoSync
      ),
    ]
  )
  lazy var ChildDefinition = try! GraphQLObjectType(
    name: "Child",
    fields: [
      "echo": .init(
        type: GraphQLNonNull(GraphQLString),
        args: [
          "message": .init(
            type: GraphQLNonNull(GraphQLString)
          ),
        ],
        resolve: config.Resolve.Schema.Child.echo
      ),
      "echoSync": .init(
        type: GraphQLNonNull(GraphQLString),
        args: [
          "message": .init(
            type: GraphQLNonNull(GraphQLString)
          ),
        ],
        resolve: config.Resolve.Schema.Child.echoSync
      ),
      "basicInterface": .init(
        type: GraphQLTypeReference("BasicInterface"),
        resolve: config.Resolve.Schema.Child.basicInterface
      ),
      "basicInterfaceList": .init(
        type: GraphQLList(GraphQLTypeReference("BasicInterface")),
        resolve: config.Resolve.Schema.Child.basicInterfaceList
      ),
      "basicInterfaceNonNullList": .init(
        type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicInterface"))),
        resolve: config.Resolve.Schema.Child.basicInterfaceNonNullList
      ),
      "basicInterfaceNonNullListNonNullElement": .init(
        type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicInterface")))),
        resolve: config.Resolve.Schema.Child.basicInterfaceNonNullListNonNullElement
      ),
      "basicUnion": .init(
        type: GraphQLTypeReference("BasicUnion"),
        resolve: config.Resolve.Schema.Child.basicUnion
      ),
      "basicUnionList": .init(
        type: GraphQLList(GraphQLTypeReference("BasicUnion")),
        resolve: config.Resolve.Schema.Child.basicUnionList
      ),
      "basicUnionNonNullList": .init(
        type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicUnion"))),
        resolve: config.Resolve.Schema.Child.basicUnionNonNullList
      ),
      "basicUnionNonNullListNonNullElement": .init(
        type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicUnion")))),
        resolve: config.Resolve.Schema.Child.basicUnionNonNullListNonNullElement
      ),
      "basicEnum": .init(
        type: GraphQLTypeReference("BasicEnum"),
        resolve: config.Resolve.Schema.Child.basicEnum
      ),
      "basicEnumList": .init(
        type: GraphQLList(GraphQLTypeReference("BasicEnum")),
        resolve: config.Resolve.Schema.Child.basicEnumList
      ),
      "basicEnumNonNullList": .init(
        type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("BasicEnum"))),
        resolve: config.Resolve.Schema.Child.basicEnumNonNullList
      ),
      "basicEnumNonNullListNonNullElement": .init(
        type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("BasicEnum")))),
        resolve: config.Resolve.Schema.Child.basicEnumNonNullListNonNullElement
      ),
    ],
    interfaces: [
      GrandParentDefinition,
      ParentDefinition,
    ]
  )
  lazy var GraphQLObjectDefinition = try! GraphQLObjectType(
    name: "GraphQLObject",
    fields: [
      "foo": .init(
        type: GraphQLNonNull(GraphQLString),
        resolve: nil
      ),
    ]
  )
  lazy var graphqlDefinition = try! GraphQLObjectType(
    name: "graphql",
    fields: [
      "bar": .init(
        type: GraphQLNonNull(GraphQLString),
        resolve: nil
      ),
    ]
  )
  lazy var typingDefinition = try! GraphQLObjectType(
    name: "typing",
    fields: [
      "baz": .init(
        type: GraphQLNonNull(GraphQLString),
        resolve: nil
      ),
    ]
  )
  lazy var QueryDefinition = try! GraphQLObjectType(
    name: "Query",
    fields: [
      "version": .init(
        type: GraphQLString,
        resolve: nil
      ),
      "extendedHello": .init(
        type: GraphQLNonNull(GraphQLString),
        args: [
          "name": .init(
            type: GraphQLNonNull(GraphQLString)
          ),
        ],
        resolve: config.Basic.Extension.Query.extendedHello
      ),
    ]
  )
  lazy var MutationDefinition = try! GraphQLObjectType(
    name: "Mutation",
    fields: [
      "version": .init(
        type: GraphQLString,
        resolve: nil
      ),
    ]
  )
  lazy var SourceDefinition = try! GraphQLObjectType(
    name: "Source",
    fields: [
      "a": .init(
        type: GraphQLInt,
        resolve: config.source.source.Source.a
      ),
    ]
  )
}