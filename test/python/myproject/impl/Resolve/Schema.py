from ... import source
from ...spec import Spec



class GrandParentImpl(Spec.Resolve.Schema.GrandParentSpec):
    async def echo(self, obj: source.GrandParentSource, info, **args) -> str:
        raise NotImplementedError()
    
    def echo_sync(self, obj: source.GrandParentSource, info, **args) -> str:
        raise NotImplementedError()
    


class ParentImpl(Spec.Resolve.Schema.ParentSpec):
    pass


class ChildImpl(Spec.Resolve.Schema.ChildSpec):
    async def echo(self, obj: source.ChildSource, info, **args) -> str:
        raise NotImplementedError()
    
    async def echo_sync(self, obj: source.ChildSource, info, **args) -> str:
        raise NotImplementedError()
    
    async def basic_interface(self, obj: source.ChildSource, info, **args) -> source.BasicInterfaceSource | None:
        raise NotImplementedError()
    
    async def basic_interface_list(self, obj: source.ChildSource, info, **args) -> list[source.BasicInterfaceSource | None] | None:
        raise NotImplementedError()
    
    async def basic_interface_non_null_list(self, obj: source.ChildSource, info, **args) -> list[source.BasicInterfaceSource | None]:
        raise NotImplementedError()
    
    async def basic_interface_non_null_list_non_null_element(self, obj: source.ChildSource, info, **args) -> list[source.BasicInterfaceSource]:
        raise NotImplementedError()
    
    async def basic_union(self, obj: source.ChildSource, info, **args) -> source.BasicUnionSource | None:
        raise NotImplementedError()
    
    async def basic_union_list(self, obj: source.ChildSource, info, **args) -> list[source.BasicUnionSource | None] | None:
        raise NotImplementedError()
    
    async def basic_union_non_null_list(self, obj: source.ChildSource, info, **args) -> list[source.BasicUnionSource | None]:
        raise NotImplementedError()
    
    async def basic_union_non_null_list_non_null_element(self, obj: source.ChildSource, info, **args) -> list[source.BasicUnionSource]:
        raise NotImplementedError()
    
    async def basic_enum(self, obj: source.ChildSource, info, **args) -> source.BasicEnumSource | None:
        raise NotImplementedError()
    
    async def basic_enum_list(self, obj: source.ChildSource, info, **args) -> list[source.BasicEnumSource | None] | None:
        raise NotImplementedError()
    
    async def basic_enum_non_null_list(self, obj: source.ChildSource, info, **args) -> list[source.BasicEnumSource | None]:
        raise NotImplementedError()
    
    async def basic_enum_non_null_list_non_null_element(self, obj: source.ChildSource, info, **args) -> list[source.BasicEnumSource]:
        raise NotImplementedError()
    


