# GENERATED. DO NOT EDIT.
# fmt: off
import typing



class BasicObjectSource(typing.Protocol):
    idNonNull: "typing.Any"
    stringNonNull: "str"
    intNonNull: "int"
    floatNonNull: "float"
    booleanNonNull: "bool"
    id: "typing.Optional[typing.Any]"
    string: "typing.Optional[str]"
    int: "typing.Optional[int]"
    float: "typing.Optional[float]"
    boolean: "typing.Optional[bool]"
    basicScalar: "typing.Optional[typing.Any]"


class BasicInterfaceSource(typing.Protocol):
    interfaceField: "typing.Optional[str]"
    basicScalar: "typing.Optional[typing.Any]"


class BasicInterfaceImplSource(BasicInterfaceSource, typing.Protocol):
    interfaceField: "typing.Optional[str]"
    basicScalar: "typing.Optional[typing.Any]"


BasicEnumSource = typing.Literal[
    "ENUM_VALUE_1",
    "ENUM_VALUE_2",
]


class BasicInputSource:
    intNonNull: "int"
    boolean: "typing.Optional[bool]"
    basicScalar: "typing.Optional[typing.Any]"
    stringNonNull: "str"
    booleanNonNull: "bool"
    id: "typing.Optional[typing.Any]"
    string: "typing.Optional[str]"
    floatNonNull: "float"
    idNonNull: "typing.Any"
    float: "typing.Optional[float]"
    int: "typing.Optional[int]"


BasicScalarSource = str


class QuerySource(typing.Protocol):
    version: "typing.Optional[str]"


class MutationSource(typing.Protocol):
    version: "typing.Optional[str]"


