# GENERATED. DO NOT EDIT.
# fmt: off
from . import source
import graphql
import typing



class Spec:
    class Basic:
        class Definition:
            class BasicObjectSpec(typing.Protocol):
                async def id_non_null_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> object: ...
                async def string_non_null_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str: ...
                async def int_non_null_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> int: ...
                async def float_non_null_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> float: ...
                async def boolean_non_null_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> bool: ...
                async def id_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> object | None: ...
                async def string_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str | None: ...
                async def int_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> int | None: ...
                async def float_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> float | None: ...
                async def boolean_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> bool | None: ...
                async def basic_input_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str | None: ...
                async def basic_input_arg_with_default(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str | None: ...
            
            class BasicInterfaceSpec(typing.Protocol):
                pass
            
            class BasicInterfaceImplSpec(typing.Protocol):
                pass
            
            class BasicScalarSpec(typing.Protocol):
                def serialize(self, value: typing.Any) -> typing.Any: ...
                def parse_value(self, value: typing.Any) -> typing.Any: ...
                def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any: ...
            
        class Extension:
            class BasicObjectSpec(typing.Protocol):
                async def extended_field_with_arg(self, obj: source.BasicObjectSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> object: ...
            
            class AnotherInterfaceSpec(typing.Protocol):
                pass
            
            class BasicInterfaceSpec(typing.Protocol):
                async def extended_field_with_arg(self, obj: source.BasicInterfaceSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> object: ...
            
            class BasicInterfaceImplSpec(typing.Protocol):
                async def extended_field_with_arg(self, obj: source.BasicInterfaceImplSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> object: ...
            
            class AnotherTypeSpec(typing.Protocol):
                pass
            
            class QuerySpec(typing.Protocol):
                async def extended_hello(self, obj: source.QuerySource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str: ...
            
    class Deprecation:
        class Definition:
            class DeprecatedFieldObjectSpec(typing.Protocol):
                pass
            
    class Module:
        class Module:
            class module:
                class ModuleBSpec(typing.Protocol):
                    pass
                
        class module:
            class ModuleASpec(typing.Protocol):
                pass
            
    class Nested1:
        class Nested2:
            class nested2:
                class Nested2Spec(typing.Protocol):
                    pass
                
        class nested1:
            class Nested1Spec(typing.Protocol):
                pass
            
    class Resolve:
        class Schema:
            class GrandParentSpec(typing.Protocol):
                async def echo(self, obj: source.GrandParentSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str: ...
                def echo_sync(self, obj: source.GrandParentSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str: ...
            
            class ParentSpec(typing.Protocol):
                async def echo(self, obj: source.ParentSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str: ...
                async def echo_sync(self, obj: source.ParentSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str: ...
            
            class ChildSpec(typing.Protocol):
                async def echo(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str: ...
                async def echo_sync(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> str: ...
                async def basic_interface(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> source.BasicInterfaceSource | None: ...
                async def basic_interface_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicInterfaceSource | None] | None: ...
                async def basic_interface_non_null_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicInterfaceSource | None]: ...
                async def basic_interface_non_null_list_non_null_element(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicInterfaceSource]: ...
                async def basic_union(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> source.BasicUnionSource | None: ...
                async def basic_union_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicUnionSource | None] | None: ...
                async def basic_union_non_null_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicUnionSource | None]: ...
                async def basic_union_non_null_list_non_null_element(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicUnionSource]: ...
                async def basic_enum(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> source.BasicEnumSource | None: ...
                async def basic_enum_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicEnumSource | None] | None: ...
                async def basic_enum_non_null_list(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicEnumSource | None]: ...
                async def basic_enum_non_null_list_non_null_element(self, obj: source.ChildSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> list[source.BasicEnumSource]: ...
            
    class graphql:
        class GraphQLObjectSpec(typing.Protocol):
            pass
        
        class graphqlSpec(typing.Protocol):
            pass
        
        class typingSpec(typing.Protocol):
            pass
        
    class root:
        class QuerySpec(typing.Protocol):
            pass
        
        class MutationSpec(typing.Protocol):
            pass
        
    class source:
        class source:
            class SourceSpec(typing.Protocol):
                async def a(self, obj: source.SourceSource, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> int | None: ...
            
