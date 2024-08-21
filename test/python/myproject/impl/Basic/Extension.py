from ... import source
from ...spec import Spec



class BasicObjectImpl(Spec.Basic.Extension.BasicObjectSpec):
    async def extended_field_with_arg(self, obj: source.BasicObjectSource, info, **args) -> object:
        raise NotImplementedError()
    


class AnotherInterfaceImpl(Spec.Basic.Extension.AnotherInterfaceSpec):
    pass


class BasicInterfaceImpl(Spec.Basic.Extension.BasicInterfaceSpec):
    pass


class BasicInterfaceImplImpl(Spec.Basic.Extension.BasicInterfaceImplSpec):
    async def extended_field_with_arg(self, obj: source.BasicInterfaceImplSource, info, **args) -> object:
        raise NotImplementedError()
    


class AnotherTypeImpl(Spec.Basic.Extension.AnotherTypeSpec):
    pass


class QueryImpl(Spec.Basic.Extension.QuerySpec):
    async def extended_hello(self, obj: source.QuerySource, info, **args) -> str:
        raise NotImplementedError()
    


