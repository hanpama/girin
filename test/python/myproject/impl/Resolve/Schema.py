from ... import source
from ...spec import Spec
import graphql
import typing



class GrandParentImpl(Spec.Resolve.Schema.GrandParentSpec):
    async def echo(self, obj: source.GrandParentSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str:
        raise NotImplementedError()
    
    def echo_sync(self, obj: source.GrandParentSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str:
        raise NotImplementedError()
    


class ParentImpl(Spec.Resolve.Schema.ParentSpec):
    async def echo(self, obj: source.ParentSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str:
        raise NotImplementedError()
    
    async def echo_sync(self, obj: source.ParentSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str:
        raise NotImplementedError()
    


class ChildImpl(Spec.Resolve.Schema.ChildSpec):
    async def echo(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str:
        raise NotImplementedError()
    
    async def echo_sync(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str:
        raise NotImplementedError()
    
    async def basic_interface(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> source.BasicInterfaceSource | None:
        raise NotImplementedError()
    
    async def basic_interface_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicInterfaceSource | None] | None:
        raise NotImplementedError()
    
    async def basic_interface_non_null_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicInterfaceSource | None]:
        raise NotImplementedError()
    
    async def basic_interface_non_null_list_non_null_element(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicInterfaceSource]:
        raise NotImplementedError()
    
    async def basic_union(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> source.BasicUnionSource | None:
        raise NotImplementedError()
    
    async def basic_union_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicUnionSource | None] | None:
        raise NotImplementedError()
    
    async def basic_union_non_null_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicUnionSource | None]:
        raise NotImplementedError()
    
    async def basic_union_non_null_list_non_null_element(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicUnionSource]:
        raise NotImplementedError()
    
    async def basic_enum(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> source.BasicEnumSource | None:
        raise NotImplementedError()
    
    async def basic_enum_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicEnumSource | None] | None:
        raise NotImplementedError()
    
    async def basic_enum_non_null_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicEnumSource | None]:
        raise NotImplementedError()
    
    async def basic_enum_non_null_list_non_null_element(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicEnumSource]:
        raise NotImplementedError()
    


