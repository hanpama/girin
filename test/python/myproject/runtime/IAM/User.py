from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class User(runtime_spec.IAM.User.User):
    def id(self, obj: source_spec.User, info: graphql.GraphQLResolveInfo, ) -> object:
        raise NotImplementedError()



@typing.final
class UserID(runtime_spec.IAM.User.UserID):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:
        raise NotImplementedError()


@typing.final
class UserConnection(runtime_spec.IAM.User.UserConnection):
    pass


@typing.final
class UserEdge(runtime_spec.IAM.User.UserEdge):
    async def node(self, obj: source_spec.UserEdge, info: graphql.GraphQLResolveInfo, ) -> source_spec.User | None:
        raise NotImplementedError()



@typing.final
class Query(runtime_spec.IAM.User.Query):
    async def user(self, obj: source_spec.Query, info: graphql.GraphQLResolveInfo, id: source_spec.UserID) -> source_spec.User:
        raise NotImplementedError()

    async def user_connection(self, obj: source_spec.Query, info: graphql.GraphQLResolveInfo, first: int | None = None, after: source_spec.Cursor | None = None, last: int | None = None, before: source_spec.Cursor | None = None, offset: int | None = None, filter: source_spec.UserFilter | None = None) -> source_spec.UserConnection:
        raise NotImplementedError()
