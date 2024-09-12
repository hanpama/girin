from .... import runtime_spec
import typing


@typing.final
class Nested2(runtime_spec.Nested1.Nested2.nested2.Nested2):
    pass
