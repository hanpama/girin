from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class User(runtime_spec.IAM.User.User):
    def id(self, obj: UserSource, info: graphql.GraphQLResolveInfo, ) -> object:
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
    async def node(self, obj: UserEdgeSource, info: graphql.GraphQLResolveInfo, ) -> source_spec.UserSource | None:
        raise NotImplementedError()



@typing.final
class Query(runtime_spec.IAM.User.Query):
    async def user(self, obj: QuerySource, info: graphql.GraphQLResolveInfo, id: source_spec.UserIDSource) -> source_spec.UserSource:
        raise NotImplementedError()

    async def user_connection(self, obj: QuerySource, info: graphql.GraphQLResolveInfo, first: int | None = None, after: source_spec.CursorSource | None = None, last: int | None = None, before: source_spec.CursorSource | None = None, offset: int | None = None, filter: source_spec.UserFilterSource | None = None) -> source_spec.UserConnectionSource:
        raise NotImplementedError()
