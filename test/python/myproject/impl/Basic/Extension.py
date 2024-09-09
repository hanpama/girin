from ... import source
from ...spec import Spec
import graphql
import typing



class BasicObjectImpl(Spec.Basic.Extension.BasicObjectSpec):
    async def extended_field_with_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, id_non_null: object) -> object:
        raise NotImplementedError()
    


class AnotherInterfaceImpl(Spec.Basic.Extension.AnotherInterfaceSpec):
    pass


class BasicInterfaceImpl(Spec.Basic.Extension.BasicInterfaceSpec):
    async def extended_field_with_arg(self, obj: source.BasicInterfaceSource, info: graphql.GraphQLResolveInfo, id_non_null: object) -> object:
        raise NotImplementedError()
    


class BasicInterfaceImplImpl(Spec.Basic.Extension.BasicInterfaceImplSpec):
    async def extended_field_with_arg(self, obj: source.BasicInterfaceImplSource, info: graphql.GraphQLResolveInfo, id_non_null: object) -> object:
        raise NotImplementedError()
    


class AnotherTypeImpl(Spec.Basic.Extension.AnotherTypeSpec):
    pass


class QueryImpl(Spec.Basic.Extension.QuerySpec):
    async def extended_hello(self, obj: source.QuerySource, info: graphql.GraphQLResolveInfo, name: str) -> str:
        return f"Hello, {name}!"
    


