from .. import runtime_spec
import graphql
import typing


@typing.final
class Query(runtime_spec.root.Query):
    pass


@typing.final
class Mutation(runtime_spec.root.Mutation):
    pass


@typing.final
class DateTime(runtime_spec.root.DateTime):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:
        raise NotImplementedError()
