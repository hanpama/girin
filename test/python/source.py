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
    extendedField: "typing.Any"


class BasicInterfaceSource(typing.Protocol):
    interfaceField: "typing.Optional[str]"
    basicScalar: "typing.Optional[typing.Any]"


class BasicInterfaceImplSource(BasicInterfaceSource, typing.Protocol):
    interfaceField: "typing.Optional[str]"
    basicScalar: "typing.Optional[typing.Any]"
    extendedField: "typing.Any"
    anotherField: "typing.Any"


BasicEnumSource = typing.Literal[
    "ENUM_VALUE_1",
    "ENUM_VALUE_2",
    "EXTENDED_VALUE",
]


class BasicInputSource:
    idNonNull: "typing.Any"
    stringNonNull: "str"
    intNonNull: "int"
    floatNonNull: "float"
    booleanNonNull: "bool"
    basicScalarNotNull: "typing.Any"
    id: "typing.Optional[typing.Any]"
    string: "typing.Optional[str]"
    int: "typing.Optional[int]"
    float: "typing.Optional[float]"
    boolean: "typing.Optional[bool]"
    basicScalar: "typing.Optional[typing.Any]"


BasicScalarSource = str


class AnotherInterfaceSource(typing.Protocol):
    anotherField: "typing.Any"


class AnotherTypeSource(typing.Protocol):
    anotherField: "typing.Any"


class DeprecatedFieldObjectSource(typing.Protocol):
    deprecatedField: "typing.Optional[str]"


class QuerySource(typing.Protocol):
    version: "typing.Optional[str]"


class MutationSource(typing.Protocol):
    version: "typing.Optional[str]"


