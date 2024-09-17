from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class Query(runtime_spec.IAM.Viewer.Query):
    async def viewer(self, obj: QuerySource, info: graphql.GraphQLResolveInfo, ) -> source_spec.UserSource | None:
        raise NotImplementedError()
