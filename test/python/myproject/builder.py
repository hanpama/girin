# GENERATED. DO NOT EDIT.
# fmt: off
from .builder_config import BuilderConfig
import graphql



def build_schema(config: BuilderConfig) -> graphql.GraphQLSchema:
    BasicObject = graphql.GraphQLObjectType(
        name="BasicObject",
        description="BasicObject description\n",
        fields=lambda: {
            "idNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=lambda src, _: getattr(src, "id_non_null"),
            ),
            "stringNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=lambda src, _: getattr(src, "string_non_null"),
            ),
            "intNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLInt),
                resolve=lambda src, _: getattr(src, "int_non_null"),
            ),
            "floatNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLFloat),
                resolve=lambda src, _: getattr(src, "float_non_null"),
            ),
            "booleanNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
                resolve=lambda src, _: getattr(src, "boolean_non_null"),
            ),
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLID,
                description="id description\n",
                resolve=lambda src, _: getattr(src, "id"),
            ),
            "string": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                description="string description\n",
                resolve=lambda src, _: getattr(src, "string"),
            ),
            "int": graphql.GraphQLField(
                type_=graphql.GraphQLInt,
                description="int description\n",
                resolve=lambda src, _: getattr(src, "int"),
            ),
            "float": graphql.GraphQLField(
                type_=graphql.GraphQLFloat,
                description="float description\n",
                resolve=lambda src, _: getattr(src, "float"),
            ),
            "boolean": graphql.GraphQLField(
                type_=graphql.GraphQLBoolean,
                description="boolean description\n",
                resolve=lambda src, _: getattr(src, "boolean"),
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
                type_=BasicScalar,
                resolve=lambda src, _: getattr(src, "basic_scalar"),
            ),
            "basicInputArg": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                args={
                    "basicInput": graphql.GraphQLArgument(
                        type_=BasicInput,
                        out_name="basic_input",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.basic_input_arg,
            ),
            "basicInputArgWithDefault": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                args={
                    "basicInput": graphql.GraphQLArgument(
                        type_=BasicInput,
                        default_value={"int": 1, "basicScalar": "123", "id": "default", "string": "default", "boolean": True, "float": 1},
                        out_name="basic_input",
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.basic_input_arg_with_default,
            ),
            "extendedField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                description="extendedField description\n",
                resolve=lambda src, _: getattr(src, "extended_field"),
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
    BasicInterface = graphql.GraphQLInterfaceType(
        name="BasicInterface",
        fields=lambda: {
            "interfaceField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=lambda src, _: getattr(src, "interface_field"),
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalar,
                resolve=lambda src, _: getattr(src, "basic_scalar"),
            ),
            "extendedField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=lambda src, _: getattr(src, "extended_field"),
            ),
            "extendedFieldWithArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                args={
                    "idNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                        out_name="id_non_null",
                    ),
                },
                resolve=lambda src, _: getattr(src, "extended_field_with_arg"),
            ),
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=lambda src, _: getattr(src, "another_field"),
            ),
        },
    )
    BasicInterfaceImpl = graphql.GraphQLObjectType(
        name="BasicInterfaceImpl",
        fields=lambda: {
            "interfaceField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=lambda src, _: getattr(src, "interface_field"),
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalar,
                resolve=lambda src, _: getattr(src, "basic_scalar"),
            ),
            "extendedField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                description="extendedField description\n",
                resolve=lambda src, _: getattr(src, "extended_field"),
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
                resolve=lambda src, _: getattr(src, "another_field"),
            ),
        },
        interfaces=lambda: [
            BasicInterface,
            AnotherInterface,
        ],
    )
    BasicUnion = graphql.GraphQLUnionType(
        name="BasicUnion",
        types=lambda: [
            BasicObject,
            BasicInterfaceImpl,
            AnotherType,
        ]
    )
    BasicEnum = graphql.GraphQLEnumType(
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
    BasicInput = graphql.GraphQLInputObjectType(
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
                type_=graphql.GraphQLNonNull(BasicScalar),
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
                type_=BasicScalar,
                out_name="basic_scalar",
            ),
            "extendedField": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                description="extendedField description\n",
                out_name="extended_field",
            ),
        },
    )
    BasicScalar = graphql.GraphQLScalarType(
        name="BasicScalar",
        description="BasicScalar description\n",
        serialize=config.Basic.Definition.BasicScalar.serialize,
        parse_value=config.Basic.Definition.BasicScalar.parse_value,
        parse_literal=config.Basic.Definition.BasicScalar.parse_literal,
    )
    AnotherInterface = graphql.GraphQLInterfaceType(
        name="AnotherInterface",
        fields=lambda: {
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=lambda src, _: getattr(src, "another_field"),
            ),
        },
    )
    AnotherType = graphql.GraphQLObjectType(
        name="AnotherType",
        fields=lambda: {
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=lambda src, _: getattr(src, "another_field"),
            ),
        },
    )
    DeprecatedFieldObject = graphql.GraphQLObjectType(
        name="DeprecatedFieldObject",
        fields=lambda: {
            "deprecatedField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                deprecation_reason="No longer supported",
                resolve=lambda src, _: getattr(src, "deprecated_field"),
            ),
        },
    )
    ModuleB = graphql.GraphQLObjectType(
        name="ModuleB",
        fields=lambda: {
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=lambda src, _: getattr(src, "id"),
            ),
            "name": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=lambda src, _: getattr(src, "name"),
            ),
        },
    )
    ModuleA = graphql.GraphQLObjectType(
        name="ModuleA",
        fields=lambda: {
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=lambda src, _: getattr(src, "id"),
            ),
            "name": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=lambda src, _: getattr(src, "name"),
            ),
        },
    )
    Nested2 = graphql.GraphQLObjectType(
        name="Nested2",
        fields=lambda: {
            "value": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=lambda src, _: getattr(src, "value"),
            ),
        },
    )
    Nested1 = graphql.GraphQLObjectType(
        name="Nested1",
        fields=lambda: {
            "nested2": graphql.GraphQLField(
                type_=Nested2,
                resolve=lambda src, _: getattr(src, "nested2"),
            ),
        },
    )
    GrandParent = graphql.GraphQLInterfaceType(
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
    Parent = graphql.GraphQLInterfaceType(
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
                resolve=lambda src, _: getattr(src, "echo"),
            ),
            "echoSync": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "message": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                        out_name="message",
                    ),
                },
                resolve=lambda src, _: getattr(src, "echo_sync"),
            ),
        },
        interfaces=lambda: [
            GrandParent,
        ],
    )
    Child = graphql.GraphQLObjectType(
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
                type_=BasicInterface,
                resolve=config.Resolve.Schema.Child.basic_interface,
            ),
            "basicInterfaceList": graphql.GraphQLField(
                type_=graphql.GraphQLList(BasicInterface),
                resolve=config.Resolve.Schema.Child.basic_interface_list,
            ),
            "basicInterfaceNonNullList": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(BasicInterface)),
                resolve=config.Resolve.Schema.Child.basic_interface_non_null_list,
            ),
            "basicInterfaceNonNullListNonNullElement": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(BasicInterface))),
                resolve=config.Resolve.Schema.Child.basic_interface_non_null_list_non_null_element,
            ),
            "basicUnion": graphql.GraphQLField(
                type_=BasicUnion,
                resolve=config.Resolve.Schema.Child.basic_union,
            ),
            "basicUnionList": graphql.GraphQLField(
                type_=graphql.GraphQLList(BasicUnion),
                resolve=config.Resolve.Schema.Child.basic_union_list,
            ),
            "basicUnionNonNullList": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(BasicUnion)),
                resolve=config.Resolve.Schema.Child.basic_union_non_null_list,
            ),
            "basicUnionNonNullListNonNullElement": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(BasicUnion))),
                resolve=config.Resolve.Schema.Child.basic_union_non_null_list_non_null_element,
            ),
            "basicEnum": graphql.GraphQLField(
                type_=BasicEnum,
                resolve=config.Resolve.Schema.Child.basic_enum,
            ),
            "basicEnumList": graphql.GraphQLField(
                type_=graphql.GraphQLList(BasicEnum),
                resolve=config.Resolve.Schema.Child.basic_enum_list,
            ),
            "basicEnumNonNullList": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(BasicEnum)),
                resolve=config.Resolve.Schema.Child.basic_enum_non_null_list,
            ),
            "basicEnumNonNullListNonNullElement": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(BasicEnum))),
                resolve=config.Resolve.Schema.Child.basic_enum_non_null_list_non_null_element,
            ),
        },
        interfaces=lambda: [
            GrandParent,
            Parent,
        ],
    )
    Query = graphql.GraphQLObjectType(
        name="Query",
        fields=lambda: {
            "version": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=lambda src, _: getattr(src, "version"),
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
    Mutation = graphql.GraphQLObjectType(
        name="Mutation",
        fields=lambda: {
            "version": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                resolve=lambda src, _: getattr(src, "version"),
            ),
        },
    )
    Source = graphql.GraphQLObjectType(
        name="Source",
        fields=lambda: {
            "a": graphql.GraphQLField(
                type_=graphql.GraphQLInt,
                resolve=config.source.source.Source.a,
            ),
        },
    )
    return graphql.GraphQLSchema(
        query=Query,
        mutation=Mutation,
        types=[
            BasicObject,
            BasicInterface,
            BasicInterfaceImpl,
            BasicUnion,
            BasicEnum,
            BasicInput,
            BasicScalar,
            AnotherInterface,
            AnotherType,
            DeprecatedFieldObject,
            ModuleB,
            ModuleA,
            Nested2,
            Nested1,
            GrandParent,
            Parent,
            Child,
            Query,
            Mutation,
            Source,
        ],
    )
