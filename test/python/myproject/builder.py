# GENERATED. DO NOT EDIT.
# fmt: off
from .builder_config import BuilderConfig
import graphql



def build_schema(config: BuilderConfig) -> graphql.GraphQLSchema:
    BasicObject_type = graphql.GraphQLObjectType(
        name="BasicObject",
        description="BasicObject description\n",
        fields=lambda: {
            "idNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
            "stringNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
            "intNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLInt),
                resolve=graphql.default_field_resolver,
            ),
            "floatNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLFloat),
                resolve=graphql.default_field_resolver,
            ),
            "booleanNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
                resolve=graphql.default_field_resolver,
            ),
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLID,
                description="id description\n",
                resolve=graphql.default_field_resolver,
            ),
            "string": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                description="string description\n",
                resolve=graphql.default_field_resolver,
            ),
            "int": graphql.GraphQLField(
                type_=graphql.GraphQLInt,
                description="int description\n",
                resolve=graphql.default_field_resolver,
            ),
            "float": graphql.GraphQLField(
                type_=graphql.GraphQLFloat,
                description="float description\n",
                resolve=graphql.default_field_resolver,
            ),
            "boolean": graphql.GraphQLField(
                type_=graphql.GraphQLBoolean,
                description="boolean description\n",
                resolve=graphql.default_field_resolver,
            ),
            "idNonNullArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                args={
                    "idNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                        description="idNonNull description\n",
                        default_value="default",
                        out_name="id_non_null",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.id_non_null_arg,
            ),
            "stringNonNullArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "stringNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        default_value="default",
                        out_name="string_non_null",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.string_non_null_arg,
            ),
            "intNonNullArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLInt),
                args={
                    "intNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLInt),
                        default_value=1,
                        out_name="int_non_null",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.int_non_null_arg,
            ),
            "floatNonNullArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLFloat),
                args={
                    "floatNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLFloat),
                        default_value=1,
                        out_name="float_non_null",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.float_non_null_arg,
            ),
            "booleanNonNullArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
                args={
                    "booleanNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
                        default_value=True,
                        out_name="boolean_non_null",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.boolean_non_null_arg,
            ),
            "idArg": graphql.GraphQLField(
                type_=graphql.GraphQLID,
                args={
                    "id": graphql.GraphQLArgument(
                        type_=graphql.GraphQLID,
                        out_name="id",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.id_arg,
            ),
            "stringArg": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                args={
                    "string": graphql.GraphQLArgument(
                        type_=graphql.GraphQLString,
                        out_name="string",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.string_arg,
            ),
            "intArg": graphql.GraphQLField(
                type_=graphql.GraphQLInt,
                args={
                    "int": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="int",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.int_arg,
            ),
            "floatArg": graphql.GraphQLField(
                type_=graphql.GraphQLFloat,
                args={
                    "float": graphql.GraphQLArgument(
                        type_=graphql.GraphQLFloat,
                        out_name="float",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.float_arg,
            ),
            "booleanArg": graphql.GraphQLField(
                type_=graphql.GraphQLBoolean,
                args={
                    "boolean": graphql.GraphQLArgument(
                        type_=graphql.GraphQLBoolean,
                        out_name="boolean",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.boolean_arg,
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalar_type,
                resolve=graphql.default_field_resolver,
            ),
            "basicInputArg": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                args={
                    "basicInput": graphql.GraphQLArgument(
                        type_=BasicInput_type,
                        out_name="basic_input",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.basic_input_arg,
            ),
            "basicInputArgWithDefault": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                args={
                    "basicInput": graphql.GraphQLArgument(
                        type_=BasicInput_type,
                        default_value={"basicScalar": "123", "boolean": True, "float": 1, "id": "default", "int": 1, "string": "default"},
                        out_name="basic_input",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.basic_input_arg_with_default,
            ),
            "extendedField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                description="extendedField description\n",
                resolve=graphql.default_field_resolver,
            ),
            "extendedFieldWithArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                args={
                    "idNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                        out_name="id_non_null",
                    ),
                },
                resolve=config.Basic.Extension.BasicObject.extended_field_with_arg,
            ),
        },
    )
    BasicInterface_type = graphql.GraphQLInterfaceType(
        name="BasicInterface",
        fields=lambda: {
            "interfaceField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=graphql.default_field_resolver,
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalar_type,
                resolve=graphql.default_field_resolver,
            ),
            "extendedField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
            "extendedFieldWithArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                args={
                    "idNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                        out_name="id_non_null",
                    ),
                },
                resolve=config.Basic.Extension.BasicInterface.extended_field_with_arg,
            ),
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    BasicInterfaceImpl_type = graphql.GraphQLObjectType(
        name="BasicInterfaceImpl",
        fields=lambda: {
            "interfaceField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=graphql.default_field_resolver,
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalar_type,
                resolve=graphql.default_field_resolver,
            ),
            "extendedField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                description="extendedField description\n",
                resolve=graphql.default_field_resolver,
            ),
            "extendedFieldWithArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                args={
                    "idNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                        out_name="id_non_null",
                    ),
                },
                resolve=config.Basic.Extension.BasicInterfaceImpl.extended_field_with_arg,
            ),
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
        },
        interfaces=lambda: [
            BasicInterface_type,
            AnotherInterface_type,
        ],
    )
    BasicUnion_type = graphql.GraphQLUnionType(
        name="BasicUnion",
        types=lambda: [
            BasicObject_type,
            BasicInterfaceImpl_type,
            AnotherType_type,
        ]
    )
    BasicEnum_type = graphql.GraphQLEnumType(
        name="BasicEnum",
        values={
            "ENUM_VALUE_1": graphql.GraphQLEnumValue(
                value="ENUM_VALUE_1",
                description="ENUM_VALUE_1 description\n",
            ),
            "ENUM_VALUE_2": graphql.GraphQLEnumValue(
                value="ENUM_VALUE_2",
            ),
            "EXTENDED_VALUE": graphql.GraphQLEnumValue(
                value="EXTENDED_VALUE",
                description="EXTENDED_VALUE description\n",
            ),
        }
    )
    BasicInput_type = graphql.GraphQLInputObjectType(
        name="BasicInput",
        fields=lambda: {
            "idNonNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                description="idNonNull description\n",
                out_name="id_non_null",
            ),
            "stringNonNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                out_name="string_non_null",
            ),
            "intNonNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLInt),
                out_name="int_non_null",
            ),
            "floatNonNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLFloat),
                out_name="float_non_null",
            ),
            "booleanNonNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
                out_name="boolean_non_null",
            ),
            "basicScalarNotNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(BasicScalar_type),
                out_name="basic_scalar_not_null",
            ),
            "id": graphql.GraphQLInputField(
                type_=graphql.GraphQLID,
                out_name="id",
            ),
            "string": graphql.GraphQLInputField(
                type_=graphql.GraphQLString,
                out_name="string",
            ),
            "int": graphql.GraphQLInputField(
                type_=graphql.GraphQLInt,
                out_name="int",
            ),
            "float": graphql.GraphQLInputField(
                type_=graphql.GraphQLFloat,
                out_name="float",
            ),
            "boolean": graphql.GraphQLInputField(
                type_=graphql.GraphQLBoolean,
                out_name="boolean",
            ),
            "basicScalar": graphql.GraphQLInputField(
                type_=BasicScalar_type,
                out_name="basic_scalar",
            ),
            "extendedField": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                description="extendedField description\n",
                out_name="extended_field",
            ),
        },
    )
    BasicScalar_type = graphql.GraphQLScalarType(
        name="BasicScalar",
        description="BasicScalar description\n",
        serialize=config.Basic.Definition.BasicScalar.serialize,
        parse_value=config.Basic.Definition.BasicScalar.parse_value,
        parse_literal=config.Basic.Definition.BasicScalar.parse_literal,
    )
    AnotherInterface_type = graphql.GraphQLInterfaceType(
        name="AnotherInterface",
        fields=lambda: {
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    AnotherType_type = graphql.GraphQLObjectType(
        name="AnotherType",
        fields=lambda: {
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    DeprecatedFieldObject_type = graphql.GraphQLObjectType(
        name="DeprecatedFieldObject",
        fields=lambda: {
            "deprecatedField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                deprecation_reason="No longer supported",
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    ModuleB_type = graphql.GraphQLObjectType(
        name="ModuleB",
        fields=lambda: {
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
            "name": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    ModuleA_type = graphql.GraphQLObjectType(
        name="ModuleA",
        fields=lambda: {
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
            "name": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    Nested2_type = graphql.GraphQLObjectType(
        name="Nested2",
        fields=lambda: {
            "value": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    Nested1_type = graphql.GraphQLObjectType(
        name="Nested1",
        fields=lambda: {
            "nested2": graphql.GraphQLField(
                type_=Nested2_type,
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    GrandParent_type = graphql.GraphQLInterfaceType(
        name="GrandParent",
        fields=lambda: {
            "echo": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "message": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="message",
                    ),
                },
                resolve=config.Resolve.Schema.GrandParent.echo,
            ),
            "echoSync": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "message": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="message",
                    ),
                },
                resolve=config.Resolve.Schema.GrandParent.echo_sync,
            ),
        },
    )
    Parent_type = graphql.GraphQLInterfaceType(
        name="Parent",
        fields=lambda: {
            "echo": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "message": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="message",
                    ),
                },
                resolve=config.Resolve.Schema.Parent.echo,
            ),
            "echoSync": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "message": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="message",
                    ),
                },
                resolve=config.Resolve.Schema.Parent.echo_sync,
            ),
        },
        interfaces=lambda: [
            GrandParent_type,
        ],
    )
    Child_type = graphql.GraphQLObjectType(
        name="Child",
        fields=lambda: {
            "echo": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "message": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="message",
                    ),
                },
                resolve=config.Resolve.Schema.Child.echo,
            ),
            "echoSync": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "message": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="message",
                    ),
                },
                resolve=config.Resolve.Schema.Child.echo_sync,
            ),
            "basicInterface": graphql.GraphQLField(
                type_=BasicInterface_type,
                resolve=config.Resolve.Schema.Child.basic_interface,
            ),
            "basicInterfaceList": graphql.GraphQLField(
                type_=graphql.GraphQLList(BasicInterface_type),
                resolve=config.Resolve.Schema.Child.basic_interface_list,
            ),
            "basicInterfaceNonNullList": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(BasicInterface_type)),
                resolve=config.Resolve.Schema.Child.basic_interface_non_null_list,
            ),
            "basicInterfaceNonNullListNonNullElement": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(BasicInterface_type))),
                resolve=config.Resolve.Schema.Child.basic_interface_non_null_list_non_null_element,
            ),
            "basicUnion": graphql.GraphQLField(
                type_=BasicUnion_type,
                resolve=config.Resolve.Schema.Child.basic_union,
            ),
            "basicUnionList": graphql.GraphQLField(
                type_=graphql.GraphQLList(BasicUnion_type),
                resolve=config.Resolve.Schema.Child.basic_union_list,
            ),
            "basicUnionNonNullList": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(BasicUnion_type)),
                resolve=config.Resolve.Schema.Child.basic_union_non_null_list,
            ),
            "basicUnionNonNullListNonNullElement": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(BasicUnion_type))),
                resolve=config.Resolve.Schema.Child.basic_union_non_null_list_non_null_element,
            ),
            "basicEnum": graphql.GraphQLField(
                type_=BasicEnum_type,
                resolve=config.Resolve.Schema.Child.basic_enum,
            ),
            "basicEnumList": graphql.GraphQLField(
                type_=graphql.GraphQLList(BasicEnum_type),
                resolve=config.Resolve.Schema.Child.basic_enum_list,
            ),
            "basicEnumNonNullList": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(BasicEnum_type)),
                resolve=config.Resolve.Schema.Child.basic_enum_non_null_list,
            ),
            "basicEnumNonNullListNonNullElement": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(BasicEnum_type))),
                resolve=config.Resolve.Schema.Child.basic_enum_non_null_list_non_null_element,
            ),
        },
        interfaces=lambda: [
            GrandParent_type,
            Parent_type,
        ],
    )
    GraphQLObject_type = graphql.GraphQLObjectType(
        name="GraphQLObject",
        fields=lambda: {
            "foo": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    graphql_type = graphql.GraphQLObjectType(
        name="graphql",
        fields=lambda: {
            "bar": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    typing_type = graphql.GraphQLObjectType(
        name="typing",
        fields=lambda: {
            "baz": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    Query_type = graphql.GraphQLObjectType(
        name="Query",
        fields=lambda: {
            "version": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=graphql.default_field_resolver,
            ),
            "extendedHello": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "name": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="name",
                    ),
                },
                resolve=config.Basic.Extension.Query.extended_hello,
            ),
        },
    )
    Mutation_type = graphql.GraphQLObjectType(
        name="Mutation",
        fields=lambda: {
            "version": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    Source_type = graphql.GraphQLObjectType(
        name="Source",
        fields=lambda: {
            "a": graphql.GraphQLField(
                type_=graphql.GraphQLInt,
                resolve=config.source.source.Source.a,
            ),
        },
    )
    return graphql.GraphQLSchema(
        query=Query_type,
        mutation=Mutation_type,
        types=[
            BasicObject_type,
            BasicInterface_type,
            BasicInterfaceImpl_type,
            BasicUnion_type,
            BasicEnum_type,
            BasicInput_type,
            BasicScalar_type,
            AnotherInterface_type,
            AnotherType_type,
            DeprecatedFieldObject_type,
            ModuleB_type,
            ModuleA_type,
            Nested2_type,
            Nested1_type,
            GrandParent_type,
            Parent_type,
            Child_type,
            GraphQLObject_type,
            graphql_type,
            typing_type,
            Query_type,
            Mutation_type,
            Source_type,
        ],
    )
