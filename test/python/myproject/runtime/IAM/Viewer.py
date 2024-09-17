from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class Query(runtime_spec.IAM.Viewer.Query):
    async def viewer(self, obj: source_spec.Query, info: graphql.GraphQLResolveInfo, ) -> source_spec.User | None:
        raise NotImplementedError()
