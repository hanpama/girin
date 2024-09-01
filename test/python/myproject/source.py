# GENERATED. DO NOT EDIT.
# fmt: off
import typing



class BasicObjectSource(typing.Protocol):
    id_non_null: "typing.Any"
    string_non_null: "str"
    int_non_null: "int"
    float_non_null: "float"
    boolean_non_null: "bool"
    id: "typing.Optional[typing.Any]"
    string: "typing.Optional[str]"
    int: "typing.Optional[int]"
    float: "typing.Optional[float]"
    boolean: "typing.Optional[bool]"
    basic_scalar: "typing.Optional[BasicScalarSource]"
    extended_field: "typing.Any"


class BasicInterfaceSource(typing.Protocol):
    interface_field: "typing.Optional[str]"
    basic_scalar: "typing.Optional[BasicScalarSource]"
    extended_field: "typing.Any"
    another_field: "typing.Any"


class BasicInterfaceImplSource(BasicInterfaceSource, typing.Protocol):
    interface_field: "typing.Optional[str]"
    basic_scalar: "typing.Optional[BasicScalarSource]"
    extended_field: "typing.Any"
    another_field: "typing.Any"


BasicUnionSource = typing.Union["BasicObjectSource", "BasicInterfaceImplSource", "AnotherTypeSource"]


BasicEnumSource = typing.Literal[
    "ENUM_VALUE_1",
    "ENUM_VALUE_2",
    "EXTENDED_VALUE",
]


class BasicInputSource:
    id_non_null: "typing.Any"
    string_non_null: "str"
    int_non_null: "int"
    float_non_null: "float"
    boolean_non_null: "bool"
    basic_scalar_not_null: "BasicScalarSource"
    id: "typing.Optional[typing.Any]"
    string: "typing.Optional[str]"
    int: "typing.Optional[int]"
    float: "typing.Optional[float]"
    boolean: "typing.Optional[bool]"
    basic_scalar: "typing.Optional[BasicScalarSource]"
    extended_field: "typing.Any"


BasicScalarSource = typing.Any


class AnotherInterfaceSource(typing.Protocol):
    another_field: "typing.Any"


class AnotherTypeSource(typing.Protocol):
    another_field: "typing.Any"


class DeprecatedFieldObjectSource(typing.Protocol):
    deprecated_field: "typing.Optional[str]"


class ModuleBSource(typing.Protocol):
    id: "typing.Any"
    name: "str"


class ModuleASource(typing.Protocol):
    id: "typing.Any"
    name: "str"


class Nested2Source(typing.Protocol):
    value: "typing.Optional[str]"


class Nested1Source(typing.Protocol):
    nested2: "typing.Optional[Nested2Source]"


class GrandParentSource(typing.Protocol):
    pass


class ParentSource(GrandParentSource, typing.Protocol):
    pass


class ChildSource(ParentSource, GrandParentSource, typing.Protocol):
    pass


class GraphQLObjectSource(typing.Protocol):
    foo: "str"


class graphqlSource(typing.Protocol):
    bar: "str"


class typingSource(typing.Protocol):
    baz: "str"


class QuerySource(typing.Protocol):
    version: "typing.Optional[str]"


class MutationSource(typing.Protocol):
    version: "typing.Optional[str]"


class SourceSource(typing.Protocol):
    pass


