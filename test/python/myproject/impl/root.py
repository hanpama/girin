from .. import runtime_spec
import typing


@typing.final
class Query(runtime_spec.root.Query):
    pass


@typing.final
class Mutation(runtime_spec.root.Mutation):
    pass
