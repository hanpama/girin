from .. import runtime_spec
from .. import source_spec
import graphql
import typing


@typing.final
class Node(runtime_spec.Relay.Node):
    pass


@typing.final
class PageInfo(runtime_spec.Relay.PageInfo):
    pass


@typing.final
class Query(runtime_spec.Relay.Query):
    async def node(self, obj: source_spec.Query, info: graphql.GraphQLResolveInfo, id: object) -> source_spec.Node | None:
        raise NotImplementedError()

    async def nodes(self, obj: source_spec.Query, info: graphql.GraphQLResolveInfo, ids: list[object]) -> list[source_spec.Node | None]:
        raise NotImplementedError()



@typing.final
class Cursor(runtime_spec.Relay.Cursor):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:
        raise NotImplementedError()
