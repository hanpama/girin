from ... import source
from ...spec import Spec
import graphql
import typing



class SourceImpl(Spec.source.source.SourceSpec):
    async def a(self, obj: source.SourceSource, info: graphql.GraphQLResolveInfo, ) -> int | None:
        raise NotImplementedError()
    


