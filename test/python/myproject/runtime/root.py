from .. import runtime_spec
from .. import source_spec
import graphql
import typing


@typing.final
class Query(runtime_spec.Root.Query):
    def version(self, obj: QuerySource, info: graphql.GraphQLResolveInfo, ) -> str:
        raise NotImplementedError()



@typing.final
class Mutation(runtime_spec.Root.Mutation):
    pass


@typing.final
class Timestamp(runtime_spec.Root.Timestamp):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:
        raise NotImplementedError()


@typing.final
class Decimal(runtime_spec.Root.Decimal):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:
        raise NotImplementedError()


@typing.final
class TypeID(runtime_spec.Root.TypeID):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:
        raise NotImplementedError()
