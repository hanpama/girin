from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class Order(runtime_spec.Orders.Order.Order):
    def id(self, obj: OrderSource, info: graphql.GraphQLResolveInfo, ) -> object:
        raise NotImplementedError()

    async def orderer(self, obj: OrderSource, info: graphql.GraphQLResolveInfo, ) -> source_spec.UserSource:
        raise NotImplementedError()

    async def viewer_has_bookmarked(self, obj: OrderSource, info: graphql.GraphQLResolveInfo, ) -> bool:
        raise NotImplementedError()



@typing.final
class OrderProduct(runtime_spec.Orders.Order.OrderProduct):
    pass


@typing.final
class OrderID(runtime_spec.Orders.Order.OrderID):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:
        raise NotImplementedError()


@typing.final
class OrderConnection(runtime_spec.Orders.Order.OrderConnection):
    pass


@typing.final
class OrderEdge(runtime_spec.Orders.Order.OrderEdge):
    async def node(self, obj: OrderEdgeSource, info: graphql.GraphQLResolveInfo, ) -> source_spec.OrderSource | None:
        raise NotImplementedError()



@typing.final
class Query(runtime_spec.Orders.Order.Query):
    async def order(self, obj: QuerySource, info: graphql.GraphQLResolveInfo, id: source_spec.OrderIDSource) -> source_spec.OrderSource:
        raise NotImplementedError()

    async def order_connection(self, obj: QuerySource, info: graphql.GraphQLResolveInfo, first: int | None = None, after: source_spec.CursorSource | None = None, last: int | None = None, before: source_spec.CursorSource | None = None, offset: int | None = None, filters: list[source_spec.OrderFilterSource] | None = None) -> source_spec.OrderConnectionSource:
        raise NotImplementedError()



@typing.final
class User(runtime_spec.Orders.Order.User):
    async def orders(self, obj: UserSource, info: graphql.GraphQLResolveInfo, first: int | None = None, after: source_spec.CursorSource | None = None, last: int | None = None, before: source_spec.CursorSource | None = None, offset: int | None = None, filters: list[source_spec.OrderFilterSource] | None = None) -> source_spec.OrderConnectionSource:
        raise NotImplementedError()



@typing.final
class OrderCreateInDraftPayload(runtime_spec.Orders.Order.OrderCreateInDraftPayload):
    pass


@typing.final
class Mutation(runtime_spec.Orders.Order.Mutation):
    async def order_create_in_draft(self, obj: MutationSource, info: graphql.GraphQLResolveInfo, input: source_spec.OrderCreateInDraftInputSource) -> source_spec.OrderCreateInDraftPayloadSource | None:
        raise NotImplementedError()
