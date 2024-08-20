from ... import source
from ...spec import Spec
import graphql
import typing



class BasicObjectImpl(Spec.Basic.Definition.BasicObjectSpec):
    async def id_non_null_arg(self, obj: source.BasicObjectSource, info, **args) -> object:
        raise NotImplementedError()
    
    async def string_non_null_arg(self, obj: source.BasicObjectSource, info, **args) -> str:
        raise NotImplementedError()
    
    async def int_non_null_arg(self, obj: source.BasicObjectSource, info, **args) -> int:
        raise NotImplementedError()
    
    async def float_non_null_arg(self, obj: source.BasicObjectSource, info, **args) -> float:
        raise NotImplementedError()
    
    async def boolean_non_null_arg(self, obj: source.BasicObjectSource, info, **args) -> bool:
        raise NotImplementedError()
    
    async def id_arg(self, obj: source.BasicObjectSource, info, **args) -> object | None:
        raise NotImplementedError()
    
    async def string_arg(self, obj: source.BasicObjectSource, info, **args) -> str | None:
        raise NotImplementedError()
    
    async def int_arg(self, obj: source.BasicObjectSource, info, **args) -> int | None:
        raise NotImplementedError()
    
    async def float_arg(self, obj: source.BasicObjectSource, info, **args) -> float | None:
        raise NotImplementedError()
    
    async def boolean_arg(self, obj: source.BasicObjectSource, info, **args) -> bool | None:
        raise NotImplementedError()
    
    async def basic_input_arg(self, obj: source.BasicObjectSource, info, **args) -> str | None:
        raise NotImplementedError()
    
    async def basic_input_arg_with_default(self, obj: source.BasicObjectSource, info, **args) -> str | None:
        raise NotImplementedError()
    


class BasicInterfaceImpl(Spec.Basic.Definition.BasicInterfaceSpec):
    pass


class BasicInterfaceImplImpl(Spec.Basic.Definition.BasicInterfaceImplSpec):
    pass


class BasicScalarImpl(Spec.Basic.Definition.BasicScalarSpec):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()
    
    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()
    
    def parse_literal(self, node: graphql.ValueNode, variables) -> typing.Any:
        raise NotImplementedError()


