from ... import source
from ...spec import Spec
import graphql
import typing



class GrandParentImpl(Spec.Resolve.Schema.GrandParentSpec):
    async def echo(self, obj: source.GrandParentSource, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()
    
    def echo_sync(self, obj: source.GrandParentSource, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()
    


class ParentImpl(Spec.Resolve.Schema.ParentSpec):
    async def echo(self, obj: source.ParentSource, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()
    
    async def echo_sync(self, obj: source.ParentSource, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()
    


class ChildImpl(Spec.Resolve.Schema.ChildSpec):
    async def echo(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()
    
    async def echo_sync(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()
    
    async def basic_interface(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> source.BasicInterfaceSource | None:
        raise NotImplementedError()
    
    async def basic_interface_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> list[source.BasicInterfaceSource | None] | None:
        raise NotImplementedError()
    
    async def basic_interface_non_null_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> list[source.BasicInterfaceSource | None]:
        raise NotImplementedError()
    
    async def basic_interface_non_null_list_non_null_element(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> list[source.BasicInterfaceSource]:
        raise NotImplementedError()
    
    async def basic_union(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> source.BasicUnionSource | None:
        raise NotImplementedError()
    
    async def basic_union_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> list[source.BasicUnionSource | None] | None:
        raise NotImplementedError()
    
    async def basic_union_non_null_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> list[source.BasicUnionSource | None]:
        raise NotImplementedError()
    
    async def basic_union_non_null_list_non_null_element(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> list[source.BasicUnionSource]:
        raise NotImplementedError()
    
    async def basic_enum(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> source.BasicEnumSource | None:
        raise NotImplementedError()
    
    async def basic_enum_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> list[source.BasicEnumSource | None] | None:
        raise NotImplementedError()
    
    async def basic_enum_non_null_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> list[source.BasicEnumSource | None]:
        raise NotImplementedError()
    
    async def basic_enum_non_null_list_non_null_element(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, ) -> list[source.BasicEnumSource]:
        raise NotImplementedError()
    


