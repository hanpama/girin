# GENERATED. DO NOT EDIT.
# fmt: off
import typing

from . import Basic
from . import Deprecation
from . import Nested1
from . import root


class Config(typing.NamedTuple):
    Basic: Basic.Config
    Deprecation: Deprecation.Config
    Nested1: Nested1.Config
    root: root.Config
