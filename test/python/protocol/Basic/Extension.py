# GENERATED. DO NOT EDIT.
# fmt: off
import typing

from ... import source


class Config(typing.NamedTuple):
    class BasicObjectConfig(typing.Protocol):
        def extended_field_with_arg(self, obj: source.BasicObjectSource, info, **args): ...
    
    class AnotherInterfaceConfig(typing.Protocol):
        pass
    
    class BasicInterfaceConfig(typing.Protocol):
        def extended_field_with_arg(self, obj: source.BasicInterfaceSource, info, **args): ...
    
    class BasicInterfaceImplConfig(typing.Protocol):
        def extended_field_with_arg(self, obj: source.BasicInterfaceImplSource, info, **args): ...
    
    class AnotherTypeConfig(typing.Protocol):
        pass
    
    class QueryConfig(typing.Protocol):
        def extended_hello(self, obj: source.QuerySource, info, **args): ...
    
    BasicObject: BasicObjectConfig
    AnotherInterface: AnotherInterfaceConfig
    BasicInterface: BasicInterfaceConfig
    BasicInterfaceImpl: BasicInterfaceImplConfig
    AnotherType: AnotherTypeConfig
    Query: QueryConfig
