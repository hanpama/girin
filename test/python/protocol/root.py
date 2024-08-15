# GENERATED. DO NOT EDIT.
# fmt: off
import typing


class Config(typing.NamedTuple):
    class QueryConfig(typing.Protocol):
        pass
    
    class MutationConfig(typing.Protocol):
        pass
    
    Query: QueryConfig
    Mutation: MutationConfig
