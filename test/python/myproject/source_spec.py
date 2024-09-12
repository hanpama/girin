# GENERATED. DO NOT EDIT.
# fmt: off
import typing


class BasicObject(typing.Protocol):
    id_non_null: "object"
    string_non_null: "str"
    int_non_null: "int"
    float_non_null: "float"
    boolean_non_null: "bool"
    id: "object | None"
    string: "str | None"
    int_: "int | None"
    float_: "float | None"
    boolean: "bool | None"
    basic_scalar: "BasicScalar | None"
    extended_field: "object"

class BasicInterface(typing.Protocol):
    interface_field: "str | None"
    basic_scalar: "BasicScalar | None"
    extended_field: "object"
    another_field: "object"

class BasicInterfaceImpl(BasicInterface, typing.Protocol):
    interface_field: "str | None"
    basic_scalar: "BasicScalar | None"
    extended_field: "object"
    another_field: "object"

BasicUnion = typing.Union["BasicObject", "BasicInterfaceImpl", "AnotherType"]

BasicEnum = typing.Literal[
    "ENUM_VALUE_1",
    "ENUM_VALUE_2",
    "EXTENDED_VALUE",
]

class BasicInput(typing.TypedDict):
    id_non_null: "object"
    string_non_null: "str"
    int_non_null: "int"
    float_non_null: "float"
    boolean_non_null: "bool"
    basic_scalar_not_null: "BasicScalar"
    id: "typing.NotRequired[object | None]"
    string: "typing.NotRequired[str | None]"
    int_: "typing.NotRequired[int | None]"
    float_: "typing.NotRequired[float | None]"
    boolean: "typing.NotRequired[bool | None]"
    basic_scalar: "typing.NotRequired[BasicScalar | None]"
    extended_field: "object"

BasicScalar = typing.Any

class AnotherInterface(typing.Protocol):
    another_field: "object"

class AnotherType(typing.Protocol):
    another_field: "object"

class DeprecatedFieldObject(typing.Protocol):
    deprecated_field: "str | None"

class ModuleB(typing.Protocol):
    id: "object"
    name: "str"

class ModuleA(typing.Protocol):
    id: "object"
    name: "str"

class Nested2(typing.Protocol):
    value: "str | None"

class Nested1(typing.Protocol):
    nested2: "Nested2 | None"

class GrandParent(typing.Protocol):
    pass

class Parent(GrandParent, typing.Protocol):
    pass

class Child(Parent, GrandParent, typing.Protocol):
    pass

class GraphQLObject(typing.Protocol):
    foo: "str"

class graphql_(typing.Protocol):
    bar: "str"

class typing_(typing.Protocol):
    baz: "str"

class Query(typing.Protocol):
    version: "str | None"

class Mutation(typing.Protocol):
    version: "str | None"

class Source(typing.Protocol):
    pass
