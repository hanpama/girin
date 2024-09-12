# GENERATED. DO NOT EDIT.
# fmt: off
from .builder_config import BuilderConfig
import graphql


def build_schema() -> graphql.GraphQLSchema:
    BasicObjectType = graphql.GraphQLObjectType(
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
                resolve=BuilderConfig.Basic.Definition.BasicObject.id_non_null_arg,
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
                resolve=BuilderConfig.Basic.Definition.BasicObject.string_non_null_arg,
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
                resolve=BuilderConfig.Basic.Definition.BasicObject.int_non_null_arg,
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
                resolve=BuilderConfig.Basic.Definition.BasicObject.float_non_null_arg,
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
                resolve=BuilderConfig.Basic.Definition.BasicObject.boolean_non_null_arg,
            ),
            "idArg": graphql.GraphQLField(
                type_=graphql.GraphQLID,
                args={
                    "id": graphql.GraphQLArgument(
                        type_=graphql.GraphQLID,
                        out_name="id",
                    ),
                },
                resolve=BuilderConfig.Basic.Definition.BasicObject.id_arg,
            ),
            "stringArg": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                args={
                    "string": graphql.GraphQLArgument(
                        type_=graphql.GraphQLString,
                        out_name="string",
                    ),
                },
                resolve=BuilderConfig.Basic.Definition.BasicObject.string_arg,
            ),
            "intArg": graphql.GraphQLField(
                type_=graphql.GraphQLInt,
                args={
                    "int": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="int_",
                    ),
                },
                resolve=BuilderConfig.Basic.Definition.BasicObject.int_arg,
            ),
            "floatArg": graphql.GraphQLField(
                type_=graphql.GraphQLFloat,
                args={
                    "float": graphql.GraphQLArgument(
                        type_=graphql.GraphQLFloat,
                        out_name="float_",
                    ),
                },
                resolve=BuilderConfig.Basic.Definition.BasicObject.float_arg,
            ),
            "booleanArg": graphql.GraphQLField(
                type_=graphql.GraphQLBoolean,
                args={
                    "boolean": graphql.GraphQLArgument(
                        type_=graphql.GraphQLBoolean,
                        out_name="boolean",
                    ),
                },
                resolve=BuilderConfig.Basic.Definition.BasicObject.boolean_arg,
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalarType,
                resolve=graphql.default_field_resolver,
            ),
            "basicInputArg": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                args={
                    "basicInput": graphql.GraphQLArgument(
                        type_=BasicInputType,
                        out_name="basic_input",
                    ),
                },
                resolve=BuilderConfig.Basic.Definition.BasicObject.basic_input_arg,
            ),
            "basicInputArgWithDefault": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                args={
                    "basicInput": graphql.GraphQLArgument(
                        type_=BasicInputType,
                        default_value={"basicScalar": "123", "boolean": True, "float": 1, "id": "default", "int": 1, "string": "default"},
                        out_name="basic_input",
                    ),
                },
                resolve=BuilderConfig.Basic.Definition.BasicObject.basic_input_arg_with_default,
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
                resolve=BuilderConfig.Basic.Extension.BasicObject.extended_field_with_arg,
            ),
        },
    )
    BasicInterfaceType = graphql.GraphQLInterfaceType(
        name="BasicInterface",
        fields=lambda: {
            "interfaceField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=graphql.default_field_resolver,
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalarType,
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
                resolve=BuilderConfig.Basic.Extension.BasicInterface.extended_field_with_arg,
            ),
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    BasicInterfaceImplType = graphql.GraphQLObjectType(
        name="BasicInterfaceImpl",
        fields=lambda: {
            "interfaceField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=graphql.default_field_resolver,
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalarType,
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
                resolve=BuilderConfig.Basic.Extension.BasicInterfaceImpl.extended_field_with_arg,
            ),
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
        },
        interfaces=lambda: [
            BasicInterfaceType,
            AnotherInterfaceType,
        ],
    )
    BasicUnionType = graphql.GraphQLUnionType(
        name="BasicUnion",
        types=lambda: [
            BasicObjectType,
            BasicInterfaceImplType,
            AnotherTypeType,
        ]
    )
    BasicEnumType = graphql.GraphQLEnumType(
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
    BasicInputType = graphql.GraphQLInputObjectType(
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
                type_=graphql.GraphQLNonNull(BasicScalarType),
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
                out_name="int_",
            ),
            "float": graphql.GraphQLInputField(
                type_=graphql.GraphQLFloat,
                out_name="float_",
            ),
            "boolean": graphql.GraphQLInputField(
                type_=graphql.GraphQLBoolean,
                out_name="boolean",
            ),
            "basicScalar": graphql.GraphQLInputField(
                type_=BasicScalarType,
                out_name="basic_scalar",
            ),
            "extendedField": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                description="extendedField description\n",
                out_name="extended_field",
            ),
        },
    )
    BasicScalarType = graphql.GraphQLScalarType(
        name="BasicScalar",
        description="BasicScalar description\n",
        serialize=BuilderConfig.Basic.Definition.BasicScalar.serialize,
        parse_value=BuilderConfig.Basic.Definition.BasicScalar.parse_value,
        parse_literal=BuilderConfig.Basic.Definition.BasicScalar.parse_literal,
    )
    AnotherInterfaceType = graphql.GraphQLInterfaceType(
        name="AnotherInterface",
        fields=lambda: {
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    AnotherTypeType = graphql.GraphQLObjectType(
        name="AnotherType",
        fields=lambda: {
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    DeprecatedFieldObjectType = graphql.GraphQLObjectType(
        name="DeprecatedFieldObject",
        fields=lambda: {
            "deprecatedField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                deprecation_reason="No longer supported",
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    ModuleBType = graphql.GraphQLObjectType(
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
    ModuleAType = graphql.GraphQLObjectType(
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
    Nested2Type = graphql.GraphQLObjectType(
        name="Nested2",
        fields=lambda: {
            "value": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    Nested1Type = graphql.GraphQLObjectType(
        name="Nested1",
        fields=lambda: {
            "nested2": graphql.GraphQLField(
                type_=Nested2Type,
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    GrandParentType = graphql.GraphQLInterfaceType(
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
                resolve=BuilderConfig.Resolve.Schema.GrandParent.echo,
            ),
            "echoSync": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "message": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="message",
                    ),
                },
                resolve=BuilderConfig.Resolve.Schema.GrandParent.echo_sync,
            ),
        },
    )
    ParentType = graphql.GraphQLInterfaceType(
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
                resolve=BuilderConfig.Resolve.Schema.Parent.echo,
            ),
            "echoSync": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "message": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="message",
                    ),
                },
                resolve=BuilderConfig.Resolve.Schema.Parent.echo_sync,
            ),
        },
        interfaces=lambda: [
            GrandParentType,
        ],
    )
    ChildType = graphql.GraphQLObjectType(
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
                resolve=BuilderConfig.Resolve.Schema.Child.echo,
            ),
            "echoSync": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "message": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="message",
                    ),
                },
                resolve=BuilderConfig.Resolve.Schema.Child.echo_sync,
            ),
            "basicInterface": graphql.GraphQLField(
                type_=BasicInterfaceType,
                resolve=BuilderConfig.Resolve.Schema.Child.basic_interface,
            ),
            "basicInterfaceList": graphql.GraphQLField(
                type_=graphql.GraphQLList(BasicInterfaceType),
                resolve=BuilderConfig.Resolve.Schema.Child.basic_interface_list,
            ),
            "basicInterfaceNonNullList": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(BasicInterfaceType)),
                resolve=BuilderConfig.Resolve.Schema.Child.basic_interface_non_null_list,
            ),
            "basicInterfaceNonNullListNonNullElement": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(BasicInterfaceType))),
                resolve=BuilderConfig.Resolve.Schema.Child.basic_interface_non_null_list_non_null_element,
            ),
            "basicUnion": graphql.GraphQLField(
                type_=BasicUnionType,
                resolve=BuilderConfig.Resolve.Schema.Child.basic_union,
            ),
            "basicUnionList": graphql.GraphQLField(
                type_=graphql.GraphQLList(BasicUnionType),
                resolve=BuilderConfig.Resolve.Schema.Child.basic_union_list,
            ),
            "basicUnionNonNullList": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(BasicUnionType)),
                resolve=BuilderConfig.Resolve.Schema.Child.basic_union_non_null_list,
            ),
            "basicUnionNonNullListNonNullElement": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(BasicUnionType))),
                resolve=BuilderConfig.Resolve.Schema.Child.basic_union_non_null_list_non_null_element,
            ),
            "basicEnum": graphql.GraphQLField(
                type_=BasicEnumType,
                resolve=BuilderConfig.Resolve.Schema.Child.basic_enum,
            ),
            "basicEnumList": graphql.GraphQLField(
                type_=graphql.GraphQLList(BasicEnumType),
                resolve=BuilderConfig.Resolve.Schema.Child.basic_enum_list,
            ),
            "basicEnumNonNullList": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(BasicEnumType)),
                resolve=BuilderConfig.Resolve.Schema.Child.basic_enum_non_null_list,
            ),
            "basicEnumNonNullListNonNullElement": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(BasicEnumType))),
                resolve=BuilderConfig.Resolve.Schema.Child.basic_enum_non_null_list_non_null_element,
            ),
        },
        interfaces=lambda: [
            GrandParentType,
            ParentType,
        ],
    )
    GraphQLObjectType = graphql.GraphQLObjectType(
        name="GraphQLObject",
        fields=lambda: {
            "foo": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    graphql_Type = graphql.GraphQLObjectType(
        name="graphql",
        fields=lambda: {
            "bar": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    typing_Type = graphql.GraphQLObjectType(
        name="typing",
        fields=lambda: {
            "baz": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    QueryType = graphql.GraphQLObjectType(
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
                resolve=BuilderConfig.Basic.Extension.Query.extended_hello,
            ),
        },
    )
    MutationType = graphql.GraphQLObjectType(
        name="Mutation",
        fields=lambda: {
            "version": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    SourceType = graphql.GraphQLObjectType(
        name="Source",
        fields=lambda: {
            "a": graphql.GraphQLField(
                type_=graphql.GraphQLInt,
                resolve=BuilderConfig.source.source.Source.a,
            ),
        },
    )
    return graphql.GraphQLSchema(
        query=QueryType,
        mutation=MutationType,
        types=[
            BasicObjectType,
            BasicInterfaceType,
            BasicInterfaceImplType,
            BasicUnionType,
            BasicEnumType,
            BasicInputType,
            BasicScalarType,
            AnotherInterfaceType,
            AnotherTypeType,
            DeprecatedFieldObjectType,
            ModuleBType,
            ModuleAType,
            Nested2Type,
            Nested1Type,
            GrandParentType,
            ParentType,
            ChildType,
            GraphQLObjectType,
            graphql_Type,
            typing_Type,
            QueryType,
            MutationType,
            SourceType,
        ],
    )
