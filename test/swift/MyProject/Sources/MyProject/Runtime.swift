import GraphQL

public struct Runtime {
    public let schema: GraphQLSchema
    public struct Wiring {
        struct Currency {
        }
        struct IAM {
            struct User {
                struct User {
                    var id: (_ source: SourceSpec.User, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) throws -> String
                }
                struct UserConnection {
                }
                struct UserEdge {
                    var node: (_ source: SourceSpec.UserEdge, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.User?
                }
                struct Query {
                    var user: (_ source: SourceSpec.Query, _ args: (id: SourceSpec.UserID, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.User
                    var userConnection: (_ source: SourceSpec.Query, _ args: (first: Int?, after: SourceSpec.Cursor?, last: Int?, before: SourceSpec.Cursor?, offset: Int?, filter: SourceSpec.UserFilter?), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.UserConnection
                }
                var User: User
                var UserConnection: UserConnection
                var UserEdge: UserEdge
                var Query: Query
            }
            struct Viewer {
                struct Query {
                    var viewer: (_ source: SourceSpec.Query, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.User?
                }
                var Query: Query
            }
            var User: User
            var Viewer: Viewer
        }
        struct Orders {
            struct Order {
                struct Order {
                    var id: (_ source: SourceSpec.Order, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) throws -> String
                    var orderer: (_ source: SourceSpec.Order, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.User
                }
                struct OrderProduct {
                }
                struct OrderConnection {
                }
                struct OrderEdge {
                    var node: (_ source: SourceSpec.OrderEdge, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.Order?
                }
                struct Query {
                    var order: (_ source: SourceSpec.Query, _ args: (id: SourceSpec.OrderID, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.Order
                    var orderConnection: (_ source: SourceSpec.Query, _ args: (first: Int?, after: SourceSpec.Cursor?, last: Int?, before: SourceSpec.Cursor?, offset: Int?, filter: SourceSpec.OrderFilter?), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.OrderConnection
                }
                struct User {
                    var orders: (_ source: SourceSpec.User, _ args: (first: Int?, after: SourceSpec.Cursor?, last: Int?, before: SourceSpec.Cursor?, offset: Int?, filter: SourceSpec.OrderFilter?), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.OrderConnection
                }
                struct OrderCreateInDraftPayload {
                }
                struct Mutation {
                    var orderCreateInDraft: (_ source: SourceSpec.Mutation, _ args: (input: SourceSpec.OrderCreateInDraftInput, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.OrderCreateInDraftPayload
                }
                var Order: Order
                var OrderProduct: OrderProduct
                var OrderConnection: OrderConnection
                var OrderEdge: OrderEdge
                var Query: Query
                var User: User
                var OrderCreateInDraftPayload: OrderCreateInDraftPayload
                var Mutation: Mutation
            }
            var Order: Order
        }
        struct Relay {
            struct PageInfo {
            }
            struct Query {
                var node: (_ source: SourceSpec.Query, _ args: (id: String, _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> SourceSpec.Node?
                var nodes: (_ source: SourceSpec.Query, _ args: (ids: [String], _: ()), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) async throws -> [SourceSpec.Node?]
            }
            var PageInfo: PageInfo
            var Query: Query
        }
        struct Root {
            struct Query {
                var version: (_ source: SourceSpec.Query, _ args: (), _ context: Any, _ info: GraphQL.GraphQLResolveInfo) throws -> String
            }
            struct Mutation {
            }
            var Query: Query
            var Mutation: Mutation
        }
        var Currency: Currency
        var IAM: IAM
        var Orders: Orders
        var Relay: Relay
        var Root: Root
    }
    init(wiring: Wiring, encoder: GraphQL.MapEncoder, decoder: GraphQL.MapDecoder) {
        let UserIDDefinition = try! GraphQL.GraphQLScalarType(
            name: "UserID",
            serialize: { value in
                guard let value = value as? Encodable else {
                    throw GraphQLError(message: "UserID cannot represent value: \(value)")
                }
                return try encoder.encode(value)
            }
        )
        let OrderIDDefinition = try! GraphQL.GraphQLScalarType(
            name: "OrderID",
            serialize: { value in
                guard let value = value as? Encodable else {
                    throw GraphQLError(message: "OrderID cannot represent value: \(value)")
                }
                return try encoder.encode(value)
            }
        )
        let CursorDefinition = try! GraphQL.GraphQLScalarType(
            name: "Cursor",
            serialize: { value in
                guard let value = value as? Encodable else {
                    throw GraphQLError(message: "Cursor cannot represent value: \(value)")
                }
                return try encoder.encode(value)
            }
        )
        let TimestampDefinition = try! GraphQL.GraphQLScalarType(
            name: "Timestamp",
            description: "An [ISO-8601](https://en.wikipedia.org/wiki/ISO_8601) encoded UTC date string.\n",
            serialize: { value in
                guard let value = value as? Encodable else {
                    throw GraphQLError(message: "Timestamp cannot represent value: \(value)")
                }
                return try encoder.encode(value)
            }
        )
        let DecimalDefinition = try! GraphQL.GraphQLScalarType(
            name: "Decimal",
            description: "Decimal floating point partially implmenting http://speleotrove.com/decimal/decarith.html\n",
            serialize: { value in
                guard let value = value as? Encodable else {
                    throw GraphQLError(message: "Decimal cannot represent value: \(value)")
                }
                return try encoder.encode(value)
            }
        )
        let CurrencyDefinition = try! GraphQL.GraphQLEnumType(
            name: "Currency",
            description: "Currency (ISO 4217)\n",
            values: [
                "AED": GraphQL.GraphQLEnumValue(
                    value: "AED",
                    description: "United Arab Emirates dirham\n"
                ),
                "AFN": GraphQL.GraphQLEnumValue(
                    value: "AFN",
                    description: "Afghan afghani\n"
                ),
                "ALL": GraphQL.GraphQLEnumValue(
                    value: "ALL",
                    description: "Albanian lek\n"
                ),
                "AMD": GraphQL.GraphQLEnumValue(
                    value: "AMD",
                    description: "Armenian dram\n"
                ),
                "ANG": GraphQL.GraphQLEnumValue(
                    value: "ANG",
                    description: "Netherlands Antillean guilder\n"
                ),
                "AOA": GraphQL.GraphQLEnumValue(
                    value: "AOA",
                    description: "Angolan kwanza\n"
                ),
                "ARS": GraphQL.GraphQLEnumValue(
                    value: "ARS",
                    description: "Argentine peso\n"
                ),
                "AUD": GraphQL.GraphQLEnumValue(
                    value: "AUD",
                    description: "Australian dollar\n"
                ),
                "AWG": GraphQL.GraphQLEnumValue(
                    value: "AWG",
                    description: "Aruban florin\n"
                ),
                "AZN": GraphQL.GraphQLEnumValue(
                    value: "AZN",
                    description: "Azerbaijani manat\n"
                ),
                "BAM": GraphQL.GraphQLEnumValue(
                    value: "BAM",
                    description: "Bosnia and Herzegovina convertible mark\n"
                ),
                "BBD": GraphQL.GraphQLEnumValue(
                    value: "BBD",
                    description: "Barbados dollar\n"
                ),
                "BDT": GraphQL.GraphQLEnumValue(
                    value: "BDT",
                    description: "Bangladeshi taka\n"
                ),
                "BGN": GraphQL.GraphQLEnumValue(
                    value: "BGN",
                    description: "Bulgarian lev\n"
                ),
                "BHD": GraphQL.GraphQLEnumValue(
                    value: "BHD",
                    description: "Bahraini dinar\n"
                ),
                "BIF": GraphQL.GraphQLEnumValue(
                    value: "BIF",
                    description: "Burundian franc\n"
                ),
                "BMD": GraphQL.GraphQLEnumValue(
                    value: "BMD",
                    description: "Bermudian dollar\n"
                ),
                "BND": GraphQL.GraphQLEnumValue(
                    value: "BND",
                    description: "Brunei dollar\n"
                ),
                "BOB": GraphQL.GraphQLEnumValue(
                    value: "BOB",
                    description: "Boliviano\n"
                ),
                "BOV": GraphQL.GraphQLEnumValue(
                    value: "BOV",
                    description: "Bolivian Mvdol (funds code)\n"
                ),
                "BRL": GraphQL.GraphQLEnumValue(
                    value: "BRL",
                    description: "Brazilian real\n"
                ),
                "BSD": GraphQL.GraphQLEnumValue(
                    value: "BSD",
                    description: "Bahamian dollar\n"
                ),
                "BTN": GraphQL.GraphQLEnumValue(
                    value: "BTN",
                    description: "Bhutanese ngultrum\n"
                ),
                "BWP": GraphQL.GraphQLEnumValue(
                    value: "BWP",
                    description: "Botswana pula\n"
                ),
                "BYN": GraphQL.GraphQLEnumValue(
                    value: "BYN",
                    description: "Belarusian ruble\n"
                ),
                "BZD": GraphQL.GraphQLEnumValue(
                    value: "BZD",
                    description: "Belize dollar\n"
                ),
                "CAD": GraphQL.GraphQLEnumValue(
                    value: "CAD",
                    description: "Canadian dollar\n"
                ),
                "CDF": GraphQL.GraphQLEnumValue(
                    value: "CDF",
                    description: "Congolese franc\n"
                ),
                "CHE": GraphQL.GraphQLEnumValue(
                    value: "CHE",
                    description: "WIR euro (complementary currency)\n"
                ),
                "CHF": GraphQL.GraphQLEnumValue(
                    value: "CHF",
                    description: "Swiss franc\n"
                ),
                "CHW": GraphQL.GraphQLEnumValue(
                    value: "CHW",
                    description: "WIR franc (complementary currency)\n"
                ),
                "CLF": GraphQL.GraphQLEnumValue(
                    value: "CLF",
                    description: "Unidad de Fomento (funds code)\n"
                ),
                "CLP": GraphQL.GraphQLEnumValue(
                    value: "CLP",
                    description: "Chilean peso\n"
                ),
                "CNY": GraphQL.GraphQLEnumValue(
                    value: "CNY",
                    description: "Chinese yuan\n"
                ),
                "COP": GraphQL.GraphQLEnumValue(
                    value: "COP",
                    description: "Colombian peso\n"
                ),
                "COU": GraphQL.GraphQLEnumValue(
                    value: "COU",
                    description: "Unidad de Valor Real (UVR) (funds code)\n"
                ),
                "CRC": GraphQL.GraphQLEnumValue(
                    value: "CRC",
                    description: "Costa Rican colon\n"
                ),
                "CUC": GraphQL.GraphQLEnumValue(
                    value: "CUC",
                    description: "Cuban convertible peso\n"
                ),
                "CUP": GraphQL.GraphQLEnumValue(
                    value: "CUP",
                    description: "Cuban peso\n"
                ),
                "CVE": GraphQL.GraphQLEnumValue(
                    value: "CVE",
                    description: "Cape Verdean escudo\n"
                ),
                "CZK": GraphQL.GraphQLEnumValue(
                    value: "CZK",
                    description: "Czech koruna\n"
                ),
                "DJF": GraphQL.GraphQLEnumValue(
                    value: "DJF",
                    description: "Djiboutian franc\n"
                ),
                "DKK": GraphQL.GraphQLEnumValue(
                    value: "DKK",
                    description: "Danish krone\n"
                ),
                "DOP": GraphQL.GraphQLEnumValue(
                    value: "DOP",
                    description: "Dominican peso\n"
                ),
                "DZD": GraphQL.GraphQLEnumValue(
                    value: "DZD",
                    description: "Algerian dinar\n"
                ),
                "EGP": GraphQL.GraphQLEnumValue(
                    value: "EGP",
                    description: "Egyptian pound\n"
                ),
                "ERN": GraphQL.GraphQLEnumValue(
                    value: "ERN",
                    description: "Eritrean nakfa\n"
                ),
                "ETB": GraphQL.GraphQLEnumValue(
                    value: "ETB",
                    description: "Ethiopian birr\n"
                ),
                "EUR": GraphQL.GraphQLEnumValue(
                    value: "EUR",
                    description: "Euro\n"
                ),
                "FJD": GraphQL.GraphQLEnumValue(
                    value: "FJD",
                    description: "Fiji dollar\n"
                ),
                "FKP": GraphQL.GraphQLEnumValue(
                    value: "FKP",
                    description: "Falkland Islands pound\n"
                ),
                "GBP": GraphQL.GraphQLEnumValue(
                    value: "GBP",
                    description: "Pound sterling\n"
                ),
                "GEL": GraphQL.GraphQLEnumValue(
                    value: "GEL",
                    description: "Georgian lari\n"
                ),
                "GHS": GraphQL.GraphQLEnumValue(
                    value: "GHS",
                    description: "Ghanaian cedi\n"
                ),
                "GIP": GraphQL.GraphQLEnumValue(
                    value: "GIP",
                    description: "Gibraltar pound\n"
                ),
                "GMD": GraphQL.GraphQLEnumValue(
                    value: "GMD",
                    description: "Gambian dalasi\n"
                ),
                "GNF": GraphQL.GraphQLEnumValue(
                    value: "GNF",
                    description: "Guinean franc\n"
                ),
                "GTQ": GraphQL.GraphQLEnumValue(
                    value: "GTQ",
                    description: "Guatemalan quetzal\n"
                ),
                "GYD": GraphQL.GraphQLEnumValue(
                    value: "GYD",
                    description: "Guyanese dollar\n"
                ),
                "HKD": GraphQL.GraphQLEnumValue(
                    value: "HKD",
                    description: "Hong Kong dollar\n"
                ),
                "HNL": GraphQL.GraphQLEnumValue(
                    value: "HNL",
                    description: "Honduran lempira\n"
                ),
                "HRK": GraphQL.GraphQLEnumValue(
                    value: "HRK",
                    description: "Croatian kuna\n"
                ),
                "HTG": GraphQL.GraphQLEnumValue(
                    value: "HTG",
                    description: "Haitian gourde\n"
                ),
                "HUF": GraphQL.GraphQLEnumValue(
                    value: "HUF",
                    description: "Hungarian forint\n"
                ),
                "IDR": GraphQL.GraphQLEnumValue(
                    value: "IDR",
                    description: "Indonesian rupiah\n"
                ),
                "ILS": GraphQL.GraphQLEnumValue(
                    value: "ILS",
                    description: "Israeli new shekel\n"
                ),
                "INR": GraphQL.GraphQLEnumValue(
                    value: "INR",
                    description: "Indian rupee\n"
                ),
                "IQD": GraphQL.GraphQLEnumValue(
                    value: "IQD",
                    description: "Iraqi dinar\n"
                ),
                "IRR": GraphQL.GraphQLEnumValue(
                    value: "IRR",
                    description: "Iranian rial\n"
                ),
                "ISK": GraphQL.GraphQLEnumValue(
                    value: "ISK",
                    description: "Icelandic króna (plural: krónur)\n"
                ),
                "JMD": GraphQL.GraphQLEnumValue(
                    value: "JMD",
                    description: "Jamaican dollar\n"
                ),
                "JOD": GraphQL.GraphQLEnumValue(
                    value: "JOD",
                    description: "Jordanian dinar\n"
                ),
                "JPY": GraphQL.GraphQLEnumValue(
                    value: "JPY",
                    description: "Japanese yen\n"
                ),
                "KES": GraphQL.GraphQLEnumValue(
                    value: "KES",
                    description: "Kenyan shilling\n"
                ),
                "KGS": GraphQL.GraphQLEnumValue(
                    value: "KGS",
                    description: "Kyrgyzstani som\n"
                ),
                "KHR": GraphQL.GraphQLEnumValue(
                    value: "KHR",
                    description: "Cambodian riel\n"
                ),
                "KMF": GraphQL.GraphQLEnumValue(
                    value: "KMF",
                    description: "Comoro franc\n"
                ),
                "KPW": GraphQL.GraphQLEnumValue(
                    value: "KPW",
                    description: "North Korean won\n"
                ),
                "KRW": GraphQL.GraphQLEnumValue(
                    value: "KRW",
                    description: "South Korean won\n"
                ),
                "KWD": GraphQL.GraphQLEnumValue(
                    value: "KWD",
                    description: "Kuwaiti dinar\n"
                ),
                "KYD": GraphQL.GraphQLEnumValue(
                    value: "KYD",
                    description: "Cayman Islands dollar\n"
                ),
                "KZT": GraphQL.GraphQLEnumValue(
                    value: "KZT",
                    description: "Kazakhstani tenge\n"
                ),
                "LAK": GraphQL.GraphQLEnumValue(
                    value: "LAK",
                    description: "Lao kip\n"
                ),
                "LBP": GraphQL.GraphQLEnumValue(
                    value: "LBP",
                    description: "Lebanese pound\n"
                ),
                "LKR": GraphQL.GraphQLEnumValue(
                    value: "LKR",
                    description: "Sri Lankan rupee\n"
                ),
                "LRD": GraphQL.GraphQLEnumValue(
                    value: "LRD",
                    description: "Liberian dollar\n"
                ),
                "LSL": GraphQL.GraphQLEnumValue(
                    value: "LSL",
                    description: "Lesotho loti\n"
                ),
                "LYD": GraphQL.GraphQLEnumValue(
                    value: "LYD",
                    description: "Libyan dinar\n"
                ),
                "MAD": GraphQL.GraphQLEnumValue(
                    value: "MAD",
                    description: "Moroccan dirham\n"
                ),
                "MDL": GraphQL.GraphQLEnumValue(
                    value: "MDL",
                    description: "Moldovan leu\n"
                ),
                "MGA": GraphQL.GraphQLEnumValue(
                    value: "MGA",
                    description: "Malagasy ariary\n"
                ),
                "MKD": GraphQL.GraphQLEnumValue(
                    value: "MKD",
                    description: "Macedonian denar\n"
                ),
                "MMK": GraphQL.GraphQLEnumValue(
                    value: "MMK",
                    description: "Myanmar kyat\n"
                ),
                "MNT": GraphQL.GraphQLEnumValue(
                    value: "MNT",
                    description: "Mongolian tögrög\n"
                ),
                "MOP": GraphQL.GraphQLEnumValue(
                    value: "MOP",
                    description: "Macanese pataca\n"
                ),
                "MRU": GraphQL.GraphQLEnumValue(
                    value: "MRU",
                    description: "Mauritanian ouguiya\n"
                ),
                "MUR": GraphQL.GraphQLEnumValue(
                    value: "MUR",
                    description: "Mauritian rupee\n"
                ),
                "MVR": GraphQL.GraphQLEnumValue(
                    value: "MVR",
                    description: "Maldivian rufiyaa\n"
                ),
                "MWK": GraphQL.GraphQLEnumValue(
                    value: "MWK",
                    description: "Malawian kwacha\n"
                ),
                "MXN": GraphQL.GraphQLEnumValue(
                    value: "MXN",
                    description: "Mexican peso\n"
                ),
                "MXV": GraphQL.GraphQLEnumValue(
                    value: "MXV",
                    description: "Mexican Unidad de Inversion (UDI) (funds code)\n"
                ),
                "MYR": GraphQL.GraphQLEnumValue(
                    value: "MYR",
                    description: "Malaysian ringgit\n"
                ),
                "MZN": GraphQL.GraphQLEnumValue(
                    value: "MZN",
                    description: "Mozambican metical\n"
                ),
                "NAD": GraphQL.GraphQLEnumValue(
                    value: "NAD",
                    description: "Namibian dollar\n"
                ),
                "NGN": GraphQL.GraphQLEnumValue(
                    value: "NGN",
                    description: "Nigerian naira\n"
                ),
                "NIO": GraphQL.GraphQLEnumValue(
                    value: "NIO",
                    description: "Nicaraguan córdoba\n"
                ),
                "NOK": GraphQL.GraphQLEnumValue(
                    value: "NOK",
                    description: "Norwegian krone\n"
                ),
                "NPR": GraphQL.GraphQLEnumValue(
                    value: "NPR",
                    description: "Nepalese rupee\n"
                ),
                "NZD": GraphQL.GraphQLEnumValue(
                    value: "NZD",
                    description: "New Zealand dollar\n"
                ),
                "OMR": GraphQL.GraphQLEnumValue(
                    value: "OMR",
                    description: "Omani rial\n"
                ),
                "PAB": GraphQL.GraphQLEnumValue(
                    value: "PAB",
                    description: "Panamanian balboa\n"
                ),
                "PEN": GraphQL.GraphQLEnumValue(
                    value: "PEN",
                    description: "Peruvian sol\n"
                ),
                "PGK": GraphQL.GraphQLEnumValue(
                    value: "PGK",
                    description: "Papua New Guinean kina\n"
                ),
                "PHP": GraphQL.GraphQLEnumValue(
                    value: "PHP",
                    description: "Philippine peso\n"
                ),
                "PKR": GraphQL.GraphQLEnumValue(
                    value: "PKR",
                    description: "Pakistani rupee\n"
                ),
                "PLN": GraphQL.GraphQLEnumValue(
                    value: "PLN",
                    description: "Polish złoty\n"
                ),
                "PYG": GraphQL.GraphQLEnumValue(
                    value: "PYG",
                    description: "Paraguayan guaraní\n"
                ),
                "QAR": GraphQL.GraphQLEnumValue(
                    value: "QAR",
                    description: "Qatari riyal\n"
                ),
                "RON": GraphQL.GraphQLEnumValue(
                    value: "RON",
                    description: "Romanian leu\n"
                ),
                "RSD": GraphQL.GraphQLEnumValue(
                    value: "RSD",
                    description: "Serbian dinar\n"
                ),
                "RUB": GraphQL.GraphQLEnumValue(
                    value: "RUB",
                    description: "Russian ruble\n"
                ),
                "RWF": GraphQL.GraphQLEnumValue(
                    value: "RWF",
                    description: "Rwandan franc\n"
                ),
                "SAR": GraphQL.GraphQLEnumValue(
                    value: "SAR",
                    description: "Saudi riyal\n"
                ),
                "SBD": GraphQL.GraphQLEnumValue(
                    value: "SBD",
                    description: "Solomon Islands dollar\n"
                ),
                "SCR": GraphQL.GraphQLEnumValue(
                    value: "SCR",
                    description: "Seychelles rupee\n"
                ),
                "SDG": GraphQL.GraphQLEnumValue(
                    value: "SDG",
                    description: "Sudanese pound\n"
                ),
                "SEK": GraphQL.GraphQLEnumValue(
                    value: "SEK",
                    description: "Swedish krona (plural: kronor)\n"
                ),
                "SGD": GraphQL.GraphQLEnumValue(
                    value: "SGD",
                    description: "Singapore dollar\n"
                ),
                "SHP": GraphQL.GraphQLEnumValue(
                    value: "SHP",
                    description: "Saint Helena pound\n"
                ),
                "SLL": GraphQL.GraphQLEnumValue(
                    value: "SLL",
                    description: "Sierra Leonean leone\n"
                ),
                "SOS": GraphQL.GraphQLEnumValue(
                    value: "SOS",
                    description: "Somali shilling\n"
                ),
                "SRD": GraphQL.GraphQLEnumValue(
                    value: "SRD",
                    description: "Surinamese dollar\n"
                ),
                "SSP": GraphQL.GraphQLEnumValue(
                    value: "SSP",
                    description: "South Sudanese pound\n"
                ),
                "STN": GraphQL.GraphQLEnumValue(
                    value: "STN",
                    description: "São Tomé and Príncipe dobra\n"
                ),
                "SVC": GraphQL.GraphQLEnumValue(
                    value: "SVC",
                    description: "Salvadoran colón\n"
                ),
                "SYP": GraphQL.GraphQLEnumValue(
                    value: "SYP",
                    description: "Syrian pound\n"
                ),
                "SZL": GraphQL.GraphQLEnumValue(
                    value: "SZL",
                    description: "Swazi lilangeni\n"
                ),
                "THB": GraphQL.GraphQLEnumValue(
                    value: "THB",
                    description: "Thai baht\n"
                ),
                "TJS": GraphQL.GraphQLEnumValue(
                    value: "TJS",
                    description: "Tajikistani somoni\n"
                ),
                "TMT": GraphQL.GraphQLEnumValue(
                    value: "TMT",
                    description: "Turkmenistan manat\n"
                ),
                "TND": GraphQL.GraphQLEnumValue(
                    value: "TND",
                    description: "Tunisian dinar\n"
                ),
                "TOP": GraphQL.GraphQLEnumValue(
                    value: "TOP",
                    description: "Tongan paʻanga\n"
                ),
                "TRY": GraphQL.GraphQLEnumValue(
                    value: "TRY",
                    description: "Turkish lira\n"
                ),
                "TTD": GraphQL.GraphQLEnumValue(
                    value: "TTD",
                    description: "Trinidad and Tobago dollar\n"
                ),
                "TWD": GraphQL.GraphQLEnumValue(
                    value: "TWD",
                    description: "New Taiwan dollar\n"
                ),
                "TZS": GraphQL.GraphQLEnumValue(
                    value: "TZS",
                    description: "Tanzanian shilling\n"
                ),
                "UAH": GraphQL.GraphQLEnumValue(
                    value: "UAH",
                    description: "Ukrainian hryvnia\n"
                ),
                "UGX": GraphQL.GraphQLEnumValue(
                    value: "UGX",
                    description: "Ugandan shilling\n"
                ),
                "USD": GraphQL.GraphQLEnumValue(
                    value: "USD",
                    description: "United States dollar\n"
                ),
                "USN": GraphQL.GraphQLEnumValue(
                    value: "USN",
                    description: "United States dollar (next day) (funds code)\n"
                ),
                "UYI": GraphQL.GraphQLEnumValue(
                    value: "UYI",
                    description: "Uruguay Peso en Unidades Indexadas (URUIURUI) (funds code)\n"
                ),
                "UYU": GraphQL.GraphQLEnumValue(
                    value: "UYU",
                    description: "Uruguayan peso\n"
                ),
                "UYW": GraphQL.GraphQLEnumValue(
                    value: "UYW",
                    description: "Unidad previsional\n"
                ),
                "UZS": GraphQL.GraphQLEnumValue(
                    value: "UZS",
                    description: "Uzbekistan som\n"
                ),
                "VED": GraphQL.GraphQLEnumValue(
                    value: "VED",
                    description: "Venezuelan bolívar digital\n"
                ),
                "VES": GraphQL.GraphQLEnumValue(
                    value: "VES",
                    description: "Venezuelan bolívar soberano\n"
                ),
                "VND": GraphQL.GraphQLEnumValue(
                    value: "VND",
                    description: "Vietnamese đồng\n"
                ),
                "VUV": GraphQL.GraphQLEnumValue(
                    value: "VUV",
                    description: "Vanuatu vatu\n"
                ),
                "WST": GraphQL.GraphQLEnumValue(
                    value: "WST",
                    description: "Samoan tala\n"
                ),
                "XAF": GraphQL.GraphQLEnumValue(
                    value: "XAF",
                    description: "CFA franc BEAC\n"
                ),
                "XAG": GraphQL.GraphQLEnumValue(
                    value: "XAG",
                    description: "Silver\n"
                ),
                "XAU": GraphQL.GraphQLEnumValue(
                    value: "XAU",
                    description: "Gold\n"
                ),
                "XBA": GraphQL.GraphQLEnumValue(
                    value: "XBA",
                    description: "European Composite Unit\n"
                ),
                "XBB": GraphQL.GraphQLEnumValue(
                    value: "XBB",
                    description: "European Monetary Unit\n"
                ),
                "XBC": GraphQL.GraphQLEnumValue(
                    value: "XBC",
                    description: "European Unit of Account 9\n"
                ),
                "XBD": GraphQL.GraphQLEnumValue(
                    value: "XBD",
                    description: "European Unit of Account 17\n"
                ),
                "XCD": GraphQL.GraphQLEnumValue(
                    value: "XCD",
                    description: "East Caribbean dollar\n"
                ),
                "XDR": GraphQL.GraphQLEnumValue(
                    value: "XDR",
                    description: "Special drawing rights\n"
                ),
                "XOF": GraphQL.GraphQLEnumValue(
                    value: "XOF",
                    description: "CFA franc BCEAO\n"
                ),
                "XPD": GraphQL.GraphQLEnumValue(
                    value: "XPD",
                    description: "Palladium\n"
                ),
                "XPF": GraphQL.GraphQLEnumValue(
                    value: "XPF",
                    description: "CFP franc\n"
                ),
                "XPT": GraphQL.GraphQLEnumValue(
                    value: "XPT",
                    description: "Platinum\n"
                ),
                "XSU": GraphQL.GraphQLEnumValue(
                    value: "XSU",
                    description: "SUCRE\n"
                ),
                "XTS": GraphQL.GraphQLEnumValue(
                    value: "XTS",
                    description: "Testing\n"
                ),
                "XUA": GraphQL.GraphQLEnumValue(
                    value: "XUA",
                    description: "ADB Unit of Account\n"
                ),
                "XXX": GraphQL.GraphQLEnumValue(
                    value: "XXX",
                    description: "No currency\n"
                ),
                "YER": GraphQL.GraphQLEnumValue(
                    value: "YER",
                    description: "Yemeni rial\n"
                ),
                "ZAR": GraphQL.GraphQLEnumValue(
                    value: "ZAR",
                    description: "South African rand\n"
                ),
                "ZMW": GraphQL.GraphQLEnumValue(
                    value: "ZMW",
                    description: "Zambian kwacha\n"
                ),
                "ZWL": GraphQL.GraphQLEnumValue(
                    value: "ZWL",
                    description: "Zimbabwean dollar\n"
                )
            ]
        )
        let OrderStatusDefinition = try! GraphQL.GraphQLEnumType(
            name: "OrderStatus",
            values: [
                "DRAFT": GraphQL.GraphQLEnumValue(
                    value: "DRAFT"
                ),
                "PENDING": GraphQL.GraphQLEnumValue(
                    value: "PENDING"
                ),
                "CONFIRMED": GraphQL.GraphQLEnumValue(
                    value: "CONFIRMED"
                ),
                "CANCELLED": GraphQL.GraphQLEnumValue(
                    value: "CANCELLED"
                ),
                "SHIPPED": GraphQL.GraphQLEnumValue(
                    value: "SHIPPED"
                ),
                "DELIVERED": GraphQL.GraphQLEnumValue(
                    value: "DELIVERED"
                )
            ]
        )
        let UserFilterDefinition = try! GraphQL.GraphQLInputObjectType(
            name: "UserFilter",
            fields: [
                "id": .init(
                    type: GraphQLTypeReference("UserIDFilter")
                ),
                "q": .init(
                    type: GraphQLString
                ),
            ]
        )
        let UserIDFilterDefinition = try! GraphQL.GraphQLInputObjectType(
            name: "UserIDFilter",
            fields: [
                "eq": .init(
                    type: GraphQLTypeReference("UserID")
                ),
                "in": .init(
                    type: GraphQLList(GraphQLNonNull(GraphQLTypeReference("UserID")))
                ),
            ]
        )
        let OrderProductInputDefinition = try! GraphQL.GraphQLInputObjectType(
            name: "OrderProductInput",
            fields: [
                "description": .init(
                    type: GraphQLString
                ),
                "quantity": .init(
                    type: GraphQLInt
                ),
                "unitPrice": .init(
                    type: GraphQLTypeReference("Decimal")
                ),
                "amount": .init(
                    type: GraphQLTypeReference("Decimal")
                ),
            ]
        )
        let OrderFilterDefinition = try! GraphQL.GraphQLInputObjectType(
            name: "OrderFilter",
            fields: [
                "id": .init(
                    type: GraphQLTypeReference("OrderIDFilter")
                ),
                "q": .init(
                    type: GraphQLString
                ),
            ]
        )
        let OrderIDFilterDefinition = try! GraphQL.GraphQLInputObjectType(
            name: "OrderIDFilter",
            fields: [
                "eq": .init(
                    type: GraphQLTypeReference("OrderID")
                ),
                "in": .init(
                    type: GraphQLList(GraphQLNonNull(GraphQLTypeReference("OrderID")))
                ),
            ]
        )
        let OrderCreateInDraftInputDefinition = try! GraphQL.GraphQLInputObjectType(
            name: "OrderCreateInDraftInput",
            fields: [
                "products": .init(
                    type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("OrderProductInput"))))
                ),
                "currency": .init(
                    type: GraphQLNonNull(GraphQLTypeReference("Currency"))
                ),
                "taxRate": .init(
                    type: GraphQLNonNull(GraphQLTypeReference("Decimal"))
                ),
                "shippingAmount": .init(
                    type: GraphQLNonNull(GraphQLTypeReference("Decimal"))
                ),
                "destination": .init(
                    type: GraphQLNonNull(GraphQLString)
                ),
            ]
        )
        let NodeDefinition = try! GraphQL.GraphQLInterfaceType(
            name: "Node",
            description: "An object with an ID\n",
            fields: [
                "id": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    description: "The id of the object.\n",
                    resolve: nil
                )
            ]
        )
        let UserDefinition = try! GraphQL.GraphQLObjectType(
            name: "User",
            fields: [
                "id": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.User
                        let function = wiring.IAM.User.User.id
                        return eventLoopGroup.next().makeSucceededFuture(
                            try function(source, (), context, info)
                        )
                    }
                ),
                "orders": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("OrderConnection")),
                    args: [
                        "first": GraphQL.GraphQLArgument(
                            type: GraphQLInt
                        ),
                        "after": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("Cursor")
                        ),
                        "last": GraphQL.GraphQLArgument(
                            type: GraphQLInt
                        ),
                        "before": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("Cursor")
                        ),
                        "offset": GraphQL.GraphQLArgument(
                            type: GraphQLInt
                        ),
                        "filter": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("OrderFilter")
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.User
                        let function = wiring.Orders.Order.User.orders
                        struct Args: Decodable {
                            var first: Int?
                            var after: SourceSpec.Cursor?
                            var last: Int?
                            var before: SourceSpec.Cursor?
                            var offset: Int?
                            var filter: SourceSpec.OrderFilter?
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.first, args.after, args.last, args.before, args.offset, args.filter), context, info)
                        }
                    }
                )
            ],
            interfaces: [
                NodeDefinition,
            ]
        )
        let UserConnectionDefinition = try! GraphQL.GraphQLObjectType(
            name: "UserConnection",
            fields: [
                "edges": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("UserEdge")))),
                    resolve: nil
                ),
                "pageInfo": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("PageInfo")),
                    resolve: nil
                )
            ]
        )
        let UserEdgeDefinition = try! GraphQL.GraphQLObjectType(
            name: "UserEdge",
            fields: [
                "node": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("User"),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.UserEdge
                        let function = wiring.IAM.User.UserEdge.node
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (), context, info)
                        }
                    }
                ),
                "cursor": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Cursor")),
                    resolve: nil
                )
            ]
        )
        let OrderDefinition = try! GraphQL.GraphQLObjectType(
            name: "Order",
            fields: [
                "id": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLID),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Order
                        let function = wiring.Orders.Order.Order.id
                        return eventLoopGroup.next().makeSucceededFuture(
                            try function(source, (), context, info)
                        )
                    }
                ),
                "createdAt": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Timestamp")),
                    resolve: nil
                ),
                "updatedAt": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Timestamp")),
                    resolve: nil
                ),
                "orderer": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("User")),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Order
                        let function = wiring.Orders.Order.Order.orderer
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (), context, info)
                        }
                    }
                ),
                "status": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("OrderStatus")),
                    resolve: nil
                ),
                "destination": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    resolve: nil
                ),
                "products": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("OrderProduct")))),
                    resolve: nil
                ),
                "currency": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Currency")),
                    resolve: nil
                ),
                "taxRate": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Decimal")),
                    resolve: nil
                ),
                "productsSubtotalAmount": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Decimal")),
                    resolve: nil
                ),
                "shippingAmount": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Decimal")),
                    resolve: nil
                ),
                "taxAmount": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Decimal")),
                    resolve: nil
                ),
                "totalAmount": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Decimal")),
                    resolve: nil
                )
            ],
            interfaces: [
                NodeDefinition,
            ]
        )
        let OrderProductDefinition = try! GraphQL.GraphQLObjectType(
            name: "OrderProduct",
            fields: [
                "description": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    resolve: nil
                ),
                "quantity": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLInt),
                    resolve: nil
                ),
                "unitPrice": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Decimal")),
                    resolve: nil
                ),
                "amount": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Decimal")),
                    resolve: nil
                )
            ]
        )
        let OrderConnectionDefinition = try! GraphQL.GraphQLObjectType(
            name: "OrderConnection",
            fields: [
                "edges": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLTypeReference("OrderEdge")))),
                    resolve: nil
                ),
                "pageInfo": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("PageInfo")),
                    resolve: nil
                )
            ]
        )
        let OrderEdgeDefinition = try! GraphQL.GraphQLObjectType(
            name: "OrderEdge",
            fields: [
                "node": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("Order"),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.OrderEdge
                        let function = wiring.Orders.Order.OrderEdge.node
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (), context, info)
                        }
                    }
                ),
                "cursor": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Cursor")),
                    resolve: nil
                )
            ]
        )
        let OrderCreateInDraftPayloadDefinition = try! GraphQL.GraphQLObjectType(
            name: "OrderCreateInDraftPayload",
            fields: [
                "orderCreatedInDraft": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Order")),
                    resolve: nil
                )
            ]
        )
        let PageInfoDefinition = try! GraphQL.GraphQLObjectType(
            name: "PageInfo",
            description: "Information about pagination in a connection.\n",
            fields: [
                "hasNextPage": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLBoolean),
                    description: "When paginating forwards, are there more items?\n",
                    resolve: nil
                ),
                "hasPreviousPage": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLBoolean),
                    description: "When paginating backwards, are there more items?\n",
                    resolve: nil
                ),
                "startCursor": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("Cursor"),
                    description: "When paginating backwards, the cursor to continue.\n",
                    resolve: nil
                ),
                "endCursor": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("Cursor"),
                    description: "When paginating forwards, the cursor to continue.\n",
                    resolve: nil
                )
            ]
        )
        let QueryDefinition = try! GraphQL.GraphQLObjectType(
            name: "Query",
            fields: [
                "version": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Query
                        let function = wiring.Root.Query.version
                        return eventLoopGroup.next().makeSucceededFuture(
                            try function(source, (), context, info)
                        )
                    }
                ),
                "node": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("Node"),
                    description: "Fetches an object given its ID\n",
                    args: [
                        "id": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLID)
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Query
                        let function = wiring.Relay.Query.node
                        struct Args: Decodable {
                            var id: String
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.id, ()), context, info)
                        }
                    }
                ),
                "nodes": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLList(GraphQLTypeReference("Node"))),
                    description: "Fetches objects given their IDs\n",
                    args: [
                        "ids": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLList(GraphQLNonNull(GraphQLID)))
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Query
                        let function = wiring.Relay.Query.nodes
                        struct Args: Decodable {
                            var ids: [String]
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.ids, ()), context, info)
                        }
                    }
                ),
                "user": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("User")),
                    args: [
                        "id": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLTypeReference("UserID"))
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Query
                        let function = wiring.IAM.User.Query.user
                        struct Args: Decodable {
                            var id: SourceSpec.UserID
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.id, ()), context, info)
                        }
                    }
                ),
                "userConnection": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("UserConnection")),
                    args: [
                        "first": GraphQL.GraphQLArgument(
                            type: GraphQLInt
                        ),
                        "after": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("Cursor")
                        ),
                        "last": GraphQL.GraphQLArgument(
                            type: GraphQLInt
                        ),
                        "before": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("Cursor")
                        ),
                        "offset": GraphQL.GraphQLArgument(
                            type: GraphQLInt
                        ),
                        "filter": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("UserFilter")
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Query
                        let function = wiring.IAM.User.Query.userConnection
                        struct Args: Decodable {
                            var first: Int?
                            var after: SourceSpec.Cursor?
                            var last: Int?
                            var before: SourceSpec.Cursor?
                            var offset: Int?
                            var filter: SourceSpec.UserFilter?
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.first, args.after, args.last, args.before, args.offset, args.filter), context, info)
                        }
                    }
                ),
                "viewer": GraphQL.GraphQLField(
                    type: GraphQLTypeReference("User"),
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Query
                        let function = wiring.IAM.Viewer.Query.viewer
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (), context, info)
                        }
                    }
                ),
                "order": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("Order")),
                    args: [
                        "id": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLTypeReference("OrderID"))
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Query
                        let function = wiring.Orders.Order.Query.order
                        struct Args: Decodable {
                            var id: SourceSpec.OrderID
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.id, ()), context, info)
                        }
                    }
                ),
                "orderConnection": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("OrderConnection")),
                    args: [
                        "first": GraphQL.GraphQLArgument(
                            type: GraphQLInt
                        ),
                        "after": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("Cursor")
                        ),
                        "last": GraphQL.GraphQLArgument(
                            type: GraphQLInt
                        ),
                        "before": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("Cursor")
                        ),
                        "offset": GraphQL.GraphQLArgument(
                            type: GraphQLInt
                        ),
                        "filter": GraphQL.GraphQLArgument(
                            type: GraphQLTypeReference("OrderFilter")
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Query
                        let function = wiring.Orders.Order.Query.orderConnection
                        struct Args: Decodable {
                            var first: Int?
                            var after: SourceSpec.Cursor?
                            var last: Int?
                            var before: SourceSpec.Cursor?
                            var offset: Int?
                            var filter: SourceSpec.OrderFilter?
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.first, args.after, args.last, args.before, args.offset, args.filter), context, info)
                        }
                    }
                )
            ]
        )
        let MutationDefinition = try! GraphQL.GraphQLObjectType(
            name: "Mutation",
            fields: [
                "version": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLString),
                    resolve: nil
                ),
                "orderCreateInDraft": GraphQL.GraphQLField(
                    type: GraphQLNonNull(GraphQLTypeReference("OrderCreateInDraftPayload")),
                    args: [
                        "input": GraphQL.GraphQLArgument(
                            type: GraphQLNonNull(GraphQLTypeReference("OrderCreateInDraftInput"))
                        ),
                    ],
                    resolve: { source, args, context, eventLoopGroup, info in
                        let source = source as! SourceSpec.Mutation
                        let function = wiring.Orders.Order.Mutation.orderCreateInDraft
                        struct Args: Decodable {
                            var input: SourceSpec.OrderCreateInDraftInput
                        }
                        let args: Args = try decoder.decode(Args.self, from: args)
                        return eventLoopGroup.next().makeFutureWithTask {
                            return try await function(source, (args.input, ()), context, info)
                        }
                    }
                )
            ]
        )
        self.schema = try! GraphQL.GraphQLSchema(
            query: QueryDefinition,
            mutation: MutationDefinition,
            types: [
                CurrencyDefinition,
                UserDefinition,
                UserIDDefinition,
                UserFilterDefinition,
                UserIDFilterDefinition,
                UserConnectionDefinition,
                UserEdgeDefinition,
                OrderDefinition,
                OrderStatusDefinition,
                OrderProductDefinition,
                OrderProductInputDefinition,
                OrderIDDefinition,
                OrderFilterDefinition,
                OrderIDFilterDefinition,
                OrderConnectionDefinition,
                OrderEdgeDefinition,
                OrderCreateInDraftInputDefinition,
                OrderCreateInDraftPayloadDefinition,
                NodeDefinition,
                PageInfoDefinition,
                CursorDefinition,
                QueryDefinition,
                MutationDefinition,
                TimestampDefinition,
                DecimalDefinition,
            ]
        )
    }
}
