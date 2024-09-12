from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class BasicObject(runtime_spec.Basic.Definition.BasicObject):
    async def id_non_null_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, id_non_null: object) -> object:
        raise NotImplementedError()

    async def string_non_null_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, string_non_null: str) -> str:
        raise NotImplementedError()

    async def int_non_null_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, int_non_null: int) -> int:
        raise NotImplementedError()

    async def float_non_null_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, float_non_null: float) -> float:
        raise NotImplementedError()

    async def boolean_non_null_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, boolean_non_null: bool) -> bool:
        raise NotImplementedError()

    async def id_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, id: object | None = None) -> object | None:
        raise NotImplementedError()

    async def string_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, string: str | None = None) -> str | None:
        raise NotImplementedError()

    async def int_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, int_: int | None = None) -> int | None:
        raise NotImplementedError()

    async def float_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, float_: float | None = None) -> float | None:
        raise NotImplementedError()

    async def boolean_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, boolean: bool | None = None) -> bool | None:
        raise NotImplementedError()

    async def basic_input_arg(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, basic_input: source_spec.BasicInput | None = None) -> str | None:
        raise NotImplementedError()

    async def basic_input_arg_with_default(self, obj: source_spec.BasicObject, info: graphql.GraphQLResolveInfo, basic_input: source_spec.BasicInput | None = None) -> str | None:
        raise NotImplementedError()



@typing.final
class BasicInterface(runtime_spec.Basic.Definition.BasicInterface):
    pass


@typing.final
class BasicInterfaceImpl(runtime_spec.Basic.Definition.BasicInterfaceImpl):
    pass


@typing.final
class BasicScalar(runtime_spec.Basic.Definition.BasicScalar):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:
        raise NotImplementedError()
