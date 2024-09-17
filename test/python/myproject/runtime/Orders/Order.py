from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class Order(runtime_spec.Orders.Order.Order):
    def id(self, obj: source_spec.Order, info: graphql.GraphQLResolveInfo, ) -> object:
        raise NotImplementedError()

    async def orderer(self, obj: source_spec.Order, info: graphql.GraphQLResolveInfo, ) -> source_spec.User:
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
    async def node(self, obj: source_spec.OrderEdge, info: graphql.GraphQLResolveInfo, ) -> source_spec.Order | None:
        raise NotImplementedError()



@typing.final
class Query(runtime_spec.Orders.Order.Query):
    async def order(self, obj: source_spec.Query, info: graphql.GraphQLResolveInfo, id: source_spec.OrderID) -> source_spec.Order:
        raise NotImplementedError()

    async def order_connection(self, obj: source_spec.Query, info: graphql.GraphQLResolveInfo, first: int | None = None, after: source_spec.Cursor | None = None, last: int | None = None, before: source_spec.Cursor | None = None, offset: int | None = None, filter: source_spec.OrderFilter | None = None) -> source_spec.OrderConnection:
        raise NotImplementedError()



@typing.final
class User(runtime_spec.Orders.Order.User):
    async def orders(self, obj: source_spec.User, info: graphql.GraphQLResolveInfo, first: int | None = None, after: source_spec.Cursor | None = None, last: int | None = None, before: source_spec.Cursor | None = None, offset: int | None = None, filter: source_spec.OrderFilter | None = None) -> source_spec.OrderConnection:
        raise NotImplementedError()



@typing.final
class OrderCreateInDraftPayload(runtime_spec.Orders.Order.OrderCreateInDraftPayload):
    pass


@typing.final
class Mutation(runtime_spec.Orders.Order.Mutation):
    async def order_create_in_draft(self, obj: source_spec.Mutation, info: graphql.GraphQLResolveInfo, input: source_spec.OrderCreateInDraftInput) -> source_spec.OrderCreateInDraftPayload:
        raise NotImplementedError()
