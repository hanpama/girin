# GENERATED. DO NOT EDIT.
# fmt: off
import graphql

from . import builder_config


def build_schema(config: builder_config.BuilderConfig) -> graphql.GraphQLSchema:
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
            ),
            "stringNonNullArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
            ),
            "intNonNullArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLInt),
            ),
            "floatNonNullArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLFloat),
            ),
            "booleanNonNullArg": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
            ),
            "idArg": graphql.GraphQLField(
                type_=graphql.GraphQLID,
            ),
            "stringArg": graphql.GraphQLField(
                type_=graphql.GraphQLString,
            ),
            "intArg": graphql.GraphQLField(
                type_=graphql.GraphQLInt,
            ),
            "floatArg": graphql.GraphQLField(
                type_=graphql.GraphQLFloat,
            ),
            "booleanArg": graphql.GraphQLField(
                type_=graphql.GraphQLBoolean,
            ),
            "basicScalar": graphql.GraphQLField(
                type_=BasicScalar,
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
        },
        interfaces=lambda: [
            BasicInterface,
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
        },
    )
    BasicScalar = graphql.GraphQLScalarType(
        name="BasicScalar",
    )
    Query = graphql.GraphQLObjectType(
        name="Query",
        fields=lambda: {
            "version": graphql.GraphQLField(
                type_=graphql.GraphQLString,
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
