# GENERATED. DO NOT EDIT.
# fmt: off
import typing

from . import root
from . import Deprecation
from . import Nested1
from . import Basic


class Config(typing.NamedTuple):
    Nested1: Nested1.Config
    Basic: Basic.Config
    Deprecation: Deprecation.Config
    root: root.Config
