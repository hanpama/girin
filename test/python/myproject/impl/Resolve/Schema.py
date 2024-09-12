from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class GrandParent(runtime_spec.Resolve.Schema.GrandParent):
    async def echo(self, obj: source_spec.GrandParent, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()

    def echo_sync(self, obj: source_spec.GrandParent, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()



@typing.final
class Parent(runtime_spec.Resolve.Schema.Parent):
    async def echo(self, obj: source_spec.Parent, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()

    async def echo_sync(self, obj: source_spec.Parent, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()



@typing.final
class Child(runtime_spec.Resolve.Schema.Child):
    async def echo(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()

    async def echo_sync(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, message: str) -> str:
        raise NotImplementedError()

    async def basic_interface(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> source_spec.BasicInterface | None:
        raise NotImplementedError()

    async def basic_interface_list(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> list[source_spec.BasicInterface | None] | None:
        raise NotImplementedError()

    async def basic_interface_non_null_list(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> list[source_spec.BasicInterface | None]:
        raise NotImplementedError()

    async def basic_interface_non_null_list_non_null_element(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> list[source_spec.BasicInterface]:
        raise NotImplementedError()

    async def basic_union(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> source_spec.BasicUnion | None:
        raise NotImplementedError()

    async def basic_union_list(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> list[source_spec.BasicUnion | None] | None:
        raise NotImplementedError()

    async def basic_union_non_null_list(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> list[source_spec.BasicUnion | None]:
        raise NotImplementedError()

    async def basic_union_non_null_list_non_null_element(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> list[source_spec.BasicUnion]:
        raise NotImplementedError()

    async def basic_enum(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> source_spec.BasicEnum | None:
        raise NotImplementedError()

    async def basic_enum_list(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> list[source_spec.BasicEnum | None] | None:
        raise NotImplementedError()

    async def basic_enum_non_null_list(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> list[source_spec.BasicEnum | None]:
        raise NotImplementedError()

    async def basic_enum_non_null_list_non_null_element(self, obj: source_spec.Child, info: graphql.GraphQLResolveInfo, ) -> list[source_spec.BasicEnum]:
        raise NotImplementedError()
