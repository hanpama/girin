from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class Source(runtime_spec.source.source.Source):
    async def a(self, obj: source_spec.Source, info: graphql.GraphQLResolveInfo, ) -> int | None:
        raise NotImplementedError()
