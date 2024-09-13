from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class BasicObject(runtime_spec.Basic.Extension.BasicObject):
    async def extended_field_with_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, id_non_null: object) -> object:
        raise NotImplementedError()



@typing.final
class AnotherInterface(runtime_spec.Basic.Extension.AnotherInterface):
    pass


@typing.final
class BasicInterface(runtime_spec.Basic.Extension.BasicInterface):
    async def extended_field_with_arg(self, obj: source_spec.BasicInterface, info: graphql.GraphQLResolveInfo, id_non_null: object) -> object:
        raise NotImplementedError()



@typing.final
class BasicInterfaceImpl(runtime_spec.Basic.Extension.BasicInterfaceImpl):
    async def extended_field_with_arg(self, obj: source_spec.BasicInterfaceImpl, info: graphql.GraphQLResolveInfo, id_non_null: object) -> object:
        raise NotImplementedError()



@typing.final
class AnotherType(runtime_spec.Basic.Extension.AnotherType):
    pass


@typing.final
class Query(runtime_spec.Basic.Extension.Query):
    async def extended_hello(self, obj: source_spec.Query, info: graphql.GraphQLResolveInfo, name: str) -> str:
        raise NotImplementedError()
