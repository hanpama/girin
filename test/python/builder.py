# GENERATED. DO NOT EDIT.
# fmt: off
import graphql

from .protocol import Config


def build_schema(config: Config) -> graphql.GraphQLSchema:
    BasicObject = graphql.GraphQLObjectType(
        name="BasicObject",
        fields=lambda: {
            "idNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
            "stringNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
            ),
            "intNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLInt),
            ),
            "floatNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLFloat),
            ),
            "booleanNonNull": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
            ),
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLID,
            ),
            "string": graphql.GraphQLField(
                type_=graphql.GraphQLString,
            ),
            "int": graphql.GraphQLField(
                type_=graphql.GraphQLInt,
            ),
            "float": graphql.GraphQLField(
                type_=graphql.GraphQLFloat,
            ),
            "boolean": graphql.GraphQLField(
                type_=graphql.GraphQLBoolean,
            ),
            "idNonNullArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                args={
                    "idNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                        default_value="default",
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
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.boolean_non_null_arg,
            ),
            "idArg": graphql.GraphQLField(
                type_=graphql.GraphQLID,
                args={
                    "id": graphql.GraphQLArgument(
                        type_=graphql.GraphQLID,
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.id_arg,
            ),
            "stringArg": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                args={
                    "string": graphql.GraphQLArgument(
                        type_=graphql.GraphQLString,
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.string_arg,
            ),
            "intArg": graphql.GraphQLField(
                type_=graphql.GraphQLInt,
                args={
                    "int": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.int_arg,
            ),
            "floatArg": graphql.GraphQLField(
                type_=graphql.GraphQLFloat,
                args={
                    "float": graphql.GraphQLArgument(
                        type_=graphql.GraphQLFloat,
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.float_arg,
            ),
            "booleanArg": graphql.GraphQLField(
                type_=graphql.GraphQLBoolean,
                args={
                    "boolean": graphql.GraphQLArgument(
                        type_=graphql.GraphQLBoolean,
                    ),
                },
                resolve=config.Basic.Definition.BasicObject.boolean_arg,
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalar,
            ),
            "extendedField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
            "extendedFieldWithArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                args={
                    "idNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLID),
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
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalar,
            ),
            "extendedField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
            "extendedFieldWithArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                args={
                    "idNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                    ),
                },
                resolve=config.Basic.Extension.BasicInterface.extended_field_with_arg,
            ),
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
        },
    )
    BasicInterfaceImpl = graphql.GraphQLObjectType(
        name="BasicInterfaceImpl",
        fields=lambda: {
            "interfaceField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalar,
            ),
            "extendedField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
            "extendedFieldWithArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                args={
                    "idNonNull": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                    ),
                },
                resolve=config.Basic.Extension.BasicInterfaceImpl.extended_field_with_arg,
            ),
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
        },
        interfaces=lambda: [
            BasicInterface,
            AnotherInterface,
        ],
    )
    BasicInput = graphql.GraphQLInputObjectType(
        name="BasicInput",
        fields=lambda: {
            "idNonNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
            "stringNonNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
            ),
            "intNonNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLInt),
            ),
            "floatNonNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLFloat),
            ),
            "booleanNonNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
            ),
            "basicScalarNotNull": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(BasicScalar),
            ),
            "id": graphql.GraphQLInputField(
                type_=graphql.GraphQLID,
            ),
            "string": graphql.GraphQLInputField(
                type_=graphql.GraphQLString,
            ),
            "int": graphql.GraphQLInputField(
                type_=graphql.GraphQLInt,
            ),
            "float": graphql.GraphQLInputField(
                type_=graphql.GraphQLFloat,
            ),
            "boolean": graphql.GraphQLInputField(
                type_=graphql.GraphQLBoolean,
            ),
            "basicScalar": graphql.GraphQLInputField(
                type_=BasicScalar,
            ),
            "extendedField": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
        },
    )
    BasicScalar = graphql.GraphQLScalarType(
        name="BasicScalar",
        serialize=config.Basic.Definition.BasicScalar.serialize,
        parse_value=config.Basic.Definition.BasicScalar.parse_value,
        parse_literal=config.Basic.Definition.BasicScalar.parse_literal,
    )
    AnotherInterface = graphql.GraphQLInterfaceType(
        name="AnotherInterface",
        fields=lambda: {
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
        },
    )
    AnotherType = graphql.GraphQLObjectType(
        name="AnotherType",
        fields=lambda: {
            "anotherField": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
        },
    )
    DeprecatedFieldObject = graphql.GraphQLObjectType(
        name="DeprecatedFieldObject",
        fields=lambda: {
            "deprecatedField": graphql.GraphQLField(
                type_=graphql.GraphQLString,
                deprecation_reason="No longer supported",
            ),
        },
    )
    Nested2 = graphql.GraphQLObjectType(
        name="Nested2",
        fields=lambda: {
            "value": graphql.GraphQLField(
                type_=graphql.GraphQLString,
            ),
        },
    )
    Nested1 = graphql.GraphQLObjectType(
        name="Nested1",
        fields=lambda: {
            "nested2": graphql.GraphQLField(
                type_=Nested2,
            ),
        },
    )
    Query = graphql.GraphQLObjectType(
        name="Query",
        fields=lambda: {
            "version": graphql.GraphQLField(
                type_=graphql.GraphQLString,
            ),
            "extendedHello": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                args={
                    "name": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLString),
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
            ),
        },
    )
    return graphql.GraphQLSchema(
        query=Query,
        mutation=Mutation,
        types=[
        ],
    )
