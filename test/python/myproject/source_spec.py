# GENERATED. DO NOT EDIT.
# fmt: off
import typing


Currency = typing.Literal[
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

class User(Node, typing.Protocol):
    pass

UserID = typing.Any

class UserFilter(typing.TypedDict):
    id: "typing.NotRequired[UserIDFilter | None]"
    q: "typing.NotRequired[str | None]"

class UserIDFilter(typing.TypedDict):
    eq: "typing.NotRequired[UserID | None]"
    in_: "typing.NotRequired[list[UserID] | None]"

class UserConnection(typing.Protocol):
    edges: "list[UserEdge]"
    page_info: "PageInfo"

class UserEdge(typing.Protocol):
    cursor: "Cursor"

class Order(Node, typing.Protocol):
    created_at: "Timestamp"
    updated_at: "Timestamp"
    orderer_id: "UserID! | None"
    status: "OrderStatus"
    destination: "str"
    products: "list[OrderProduct]"
    currency: "Currency"
    tax_rate: "Decimal"
    products_subtotal_amount: "Decimal"
    shipping_amount: "Decimal"
    tax_amount: "Decimal"
    total_amount: "Decimal"

OrderStatus = typing.Literal[
    "DRAFT",
    "PENDING",
    "CONFIRMED",
    "CANCELLED",
    "SHIPPED",
    "DELIVERED",
]

class OrderProduct(typing.Protocol):
    description: "str"
    quantity: "int"
    unit_price: "Decimal"
    amount: "Decimal"

class OrderProductInput(typing.TypedDict):
    description: "typing.NotRequired[str | None]"
    quantity: "typing.NotRequired[int | None]"
    unit_price: "typing.NotRequired[Decimal | None]"
    amount: "typing.NotRequired[Decimal | None]"

OrderID = typing.Any

class OrderFilter(typing.TypedDict):
    id: "typing.NotRequired[OrderIDFilter | None]"
    q: "typing.NotRequired[str | None]"

class OrderIDFilter(typing.TypedDict):
    eq: "typing.NotRequired[OrderID | None]"
    in_: "typing.NotRequired[list[OrderID] | None]"

class OrderConnection(typing.Protocol):
    edges: "list[OrderEdge]"
    page_info: "PageInfo"

class OrderEdge(typing.Protocol):
    cursor: "Cursor"

class OrderCreateInDraftInput(typing.TypedDict):
    products: "list[OrderProductInput]"
    currency: "Currency"
    tax_rate: "Decimal"
    shipping_amount: "Decimal"
    destination: "str"

class OrderCreateInDraftPayload(typing.Protocol):
    order_created_in_draft: "Order"

class Node(typing.Protocol):
    id: "ID! | None"

class PageInfo(typing.Protocol):
    has_next_page: "bool"
    has_previous_page: "bool"
    start_cursor: "Cursor | None"
    end_cursor: "Cursor | None"

Cursor = typing.Any

class Query(typing.Protocol):
    pass

class Mutation(typing.Protocol):
    version: "str"

Timestamp = typing.Any

Decimal = typing.Any
