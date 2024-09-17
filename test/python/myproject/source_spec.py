# GENERATED. DO NOT EDIT.
# fmt: off
from datetime import datetime as TimestampSource
from decimal import Decimal as DecimalSource
from myproject.scalars import BookmarkableID as BookmarkableIDSource
from uuid import UUID as BookmarkIDSource
from uuid import UUID as OrderIDSource
from uuid import UUID as TypeIDSource
from uuid import UUID as UserIDSource
import typing


class BookmarkSource(typing.Protocol):
    id: "object"
    created_at: "TimestampSource"
    bookmarker_id: "object"
    bookmarkable_id: "BookmarkableIDSource"
    bookmarkable_type_id: "TypeIDSource"


class BookmarkIDFilterSource(typing.TypedDict):
    eq: "typing.NotRequired[BookmarkIDSource | None]"
    in_: "typing.NotRequired[list[BookmarkIDSource] | None]"

class BookmarkFilterSource(typing.TypedDict):
    id: "typing.NotRequired[BookmarkIDFilterSource | None]"
    bookmarkable_id: "typing.NotRequired[BookmarkableIDFilterSource | None]"
    bookmarker_id: "typing.NotRequired[UserIDFilterSource | None]"

class BookmarkConnectionSource(typing.Protocol):
    edges: "list[BookmarkEdgeSource]"
    page_info: "PageInfoSource"

class BookmarkEdgeSource(typing.Protocol):
    cursor: "CursorSource"

BookmarkableSource: typing.TypeAlias = "OrderSource"


class BookmarkableIDFilterSource(typing.TypedDict):
    eq: "typing.NotRequired[BookmarkableIDSource | None]"
    in_: "typing.NotRequired[list[BookmarkableIDSource] | None]"

class BookmarkableBookmarkInputSource(typing.TypedDict):
    bookmarkable_id: "BookmarkableIDSource"

class BookmarkableBookmarkPayloadSource(typing.Protocol):
    bookmarkable_bookmarked_id: "BookmarkableIDSource"

class BookmarkableUnbookmarkInputSource(typing.TypedDict):
    bookmarkable_id: "BookmarkableIDSource"

class BookmarkableUnbookmarkPayloadSource(typing.Protocol):
    bookmarkable_unbookmarked_id: "BookmarkableIDSource"

CurrencySource = typing.Literal[
    "AED",
    "AFN",
    "ALL",
    "AMD",
    "ANG",
    "AOA",
    "ARS",
    "AUD",
    "AWG",
    "AZN",
    "BAM",
    "BBD",
    "BDT",
    "BGN",
    "BHD",
    "BIF",
    "BMD",
    "BND",
    "BOB",
    "BOV",
    "BRL",
    "BSD",
    "BTN",
    "BWP",
    "BYN",
    "BZD",
    "CAD",
    "CDF",
    "CHE",
    "CHF",
    "CHW",
    "CLF",
    "CLP",
    "CNY",
    "COP",
    "COU",
    "CRC",
    "CUC",
    "CUP",
    "CVE",
    "CZK",
    "DJF",
    "DKK",
    "DOP",
    "DZD",
    "EGP",
    "ERN",
    "ETB",
    "EUR",
    "FJD",
    "FKP",
    "GBP",
    "GEL",
    "GHS",
    "GIP",
    "GMD",
    "GNF",
    "GTQ",
    "GYD",
    "HKD",
    "HNL",
    "HRK",
    "HTG",
    "HUF",
    "IDR",
    "ILS",
    "INR",
    "IQD",
    "IRR",
    "ISK",
    "JMD",
    "JOD",
    "JPY",
    "KES",
    "KGS",
    "KHR",
    "KMF",
    "KPW",
    "KRW",
    "KWD",
    "KYD",
    "KZT",
    "LAK",
    "LBP",
    "LKR",
    "LRD",
    "LSL",
    "LYD",
    "MAD",
    "MDL",
    "MGA",
    "MKD",
    "MMK",
    "MNT",
    "MOP",
    "MRU",
    "MUR",
    "MVR",
    "MWK",
    "MXN",
    "MXV",
    "MYR",
    "MZN",
    "NAD",
    "NGN",
    "NIO",
    "NOK",
    "NPR",
    "NZD",
    "OMR",
    "PAB",
    "PEN",
    "PGK",
    "PHP",
    "PKR",
    "PLN",
    "PYG",
    "QAR",
    "RON",
    "RSD",
    "RUB",
    "RWF",
    "SAR",
    "SBD",
    "SCR",
    "SDG",
    "SEK",
    "SGD",
    "SHP",
    "SLL",
    "SOS",
    "SRD",
    "SSP",
    "STN",
    "SVC",
    "SYP",
    "SZL",
    "THB",
    "TJS",
    "TMT",
    "TND",
    "TOP",
    "TRY",
    "TTD",
    "TWD",
    "TZS",
    "UAH",
    "UGX",
    "USD",
    "USN",
    "UYI",
    "UYU",
    "UYW",
    "UZS",
    "VED",
    "VES",
    "VND",
    "VUV",
    "WST",
    "XAF",
    "XAG",
    "XAU",
    "XBA",
    "XBB",
    "XBC",
    "XBD",
    "XCD",
    "XDR",
    "XOF",
    "XPD",
    "XPF",
    "XPT",
    "XSU",
    "XTS",
    "XUA",
    "XXX",
    "YER",
    "ZAR",
    "ZMW",
    "ZWL",
]

class UserSource(typing.Protocol):
    pass


class UserFilterSource(typing.TypedDict):
    id: "typing.NotRequired[UserIDFilterSource | None]"
    q: "typing.NotRequired[str | None]"

class UserIDFilterSource(typing.TypedDict):
    eq: "typing.NotRequired[UserIDSource | None]"
    in_: "typing.NotRequired[list[UserIDSource] | None]"

class UserConnectionSource(typing.Protocol):
    edges: "list[UserEdgeSource]"
    page_info: "PageInfoSource"

class UserEdgeSource(typing.Protocol):
    cursor: "CursorSource"

class OrderSource(typing.Protocol):
    created_at: "TimestampSource"
    updated_at: "TimestampSource"
    orderer_id: "UserIDSource"
    status: "OrderStatusSource"
    destination: "str"
    products: "list[OrderProductSource]"
    currency: "CurrencySource"
    tax_rate: "DecimalSource"
    products_subtotal_amount: "DecimalSource"
    shipping_amount: "DecimalSource"
    tax_amount: "DecimalSource"
    total_amount: "DecimalSource"

OrderStatusSource = typing.Literal[
    "DRAFT",
    "PENDING",
    "CONFIRMED",
    "CANCELLED",
    "SHIPPED",
    "DELIVERED",
]

class OrderProductSource(typing.Protocol):
    description: "str"
    quantity: "int"
    unit_price: "DecimalSource"
    amount: "DecimalSource"

class OrderProductInputSource(typing.TypedDict):
    description: "typing.NotRequired[str | None]"
    quantity: "typing.NotRequired[int | None]"
    unit_price: "typing.NotRequired[DecimalSource | None]"
    amount: "typing.NotRequired[DecimalSource | None]"


class OrderFilterSource(typing.TypedDict):
    id: "typing.NotRequired[OrderIDFilterSource | None]"
    q: "typing.NotRequired[str | None]"

class OrderIDFilterSource(typing.TypedDict):
    eq: "typing.NotRequired[OrderIDSource | None]"
    in_: "typing.NotRequired[list[OrderIDSource] | None]"

class OrderConnectionSource(typing.Protocol):
    edges: "list[OrderEdgeSource]"
    page_info: "PageInfoSource"

class OrderEdgeSource(typing.Protocol):
    cursor: "CursorSource"

class OrderCreateInDraftInputSource(typing.TypedDict):
    products: "list[OrderProductInputSource]"
    currency: "CurrencySource"
    tax_rate: "DecimalSource"
    shipping_amount: "DecimalSource"
    destination: "str"

class OrderCreateInDraftPayloadSource(typing.Protocol):
    order_created_in_draft: "OrderSource"

NodeSource = typing.Union[
    "OrderSource",
    "BookmarkSource",
    "UserSource",
]

class PageInfoSource(typing.Protocol):
    has_next_page: "bool"
    has_previous_page: "bool"
    start_cursor: "CursorSource | None"
    end_cursor: "CursorSource | None"

CursorSource: typing.TypeAlias = str

class QuerySource(typing.Protocol):
    pass

class MutationSource(typing.Protocol):
    version: "str"
