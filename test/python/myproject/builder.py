# GENERATED. DO NOT EDIT.
# fmt: off
from .builder_config import BuilderConfig
import graphql


def build_schema() -> graphql.GraphQLSchema:
    BookmarkType = graphql.GraphQLObjectType(
        name="Bookmark",
        fields=lambda: {
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=graphql.default_field_resolver,
            ),
            "createdAt": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(TimestampType),
                resolve=graphql.default_field_resolver,
            ),
            "bookmarker": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(UserType),
                resolve=BuilderConfig.Bookmark.Bookmark.Bookmark.bookmarker,
            ),
            "bookmarkable": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(BookmarkableType),
                resolve=BuilderConfig.Bookmark.Bookmark.Bookmark.bookmarkable,
            ),
        },
        interfaces=lambda: [
            NodeType,
        ],
    )
    BookmarkIDType = graphql.GraphQLScalarType(
        name="BookmarkID",
        serialize=BuilderConfig.Bookmark.Bookmark.BookmarkID.serialize,
        parse_value=BuilderConfig.Bookmark.Bookmark.BookmarkID.parse_value,
        parse_literal=BuilderConfig.Bookmark.Bookmark.BookmarkID.parse_literal,
    )
    BookmarkIDFilterType = graphql.GraphQLInputObjectType(
        name="BookmarkIDFilter",
        fields=lambda: {
            "eq": graphql.GraphQLInputField(
                type_=BookmarkIDType,
                out_name="eq",
            ),
            "in": graphql.GraphQLInputField(
                type_=graphql.GraphQLList(graphql.GraphQLNonNull(BookmarkIDType)),
                out_name="in_",
            ),
        },
    )
    BookmarkFilterType = graphql.GraphQLInputObjectType(
        name="BookmarkFilter",
        fields=lambda: {
            "id": graphql.GraphQLInputField(
                type_=BookmarkIDFilterType,
                out_name="id",
            ),
            "bookmarkableId": graphql.GraphQLInputField(
                type_=BookmarkableIDFilterType,
                out_name="bookmarkable_id",
            ),
            "bookmarkerId": graphql.GraphQLInputField(
                type_=UserIDFilterType,
                out_name="bookmarker_id",
            ),
        },
    )
    BookmarkConnectionType = graphql.GraphQLObjectType(
        name="BookmarkConnection",
        fields=lambda: {
            "edges": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(BookmarkEdgeType))),
                resolve=graphql.default_field_resolver,
            ),
            "pageInfo": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(PageInfoType),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    BookmarkEdgeType = graphql.GraphQLObjectType(
        name="BookmarkEdge",
        fields=lambda: {
            "node": graphql.GraphQLField(
                type_=BookmarkType,
                resolve=BuilderConfig.Bookmark.Bookmark.BookmarkEdge.node,
            ),
            "cursor": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(CursorType),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    BookmarkableType = graphql.GraphQLInterfaceType(
        name="Bookmarkable",
        fields=lambda: {
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
            ),
            "viewerHasBookmarked": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
            ),
            "bookmarks": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(BookmarkConnectionType),
                args={
                    "first": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="first",
                    ),
                    "after": graphql.GraphQLArgument(
                        type_=CursorType,
                        out_name="after",
                    ),
                    "last": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="last",
                    ),
                    "before": graphql.GraphQLArgument(
                        type_=CursorType,
                        out_name="before",
                    ),
                    "filters": graphql.GraphQLArgument(
                        type_=graphql.GraphQLList(graphql.GraphQLNonNull(BookmarkFilterType)),
                        out_name="filters",
                    ),
                },
            ),
        },
        interfaces=lambda: [
            NodeType,
        ],
    )
    BookmarkableIDType = graphql.GraphQLScalarType(
        name="BookmarkableID",
        serialize=BuilderConfig.Bookmark.Bookmarkable.BookmarkableID.serialize,
        parse_value=BuilderConfig.Bookmark.Bookmarkable.BookmarkableID.parse_value,
        parse_literal=BuilderConfig.Bookmark.Bookmarkable.BookmarkableID.parse_literal,
    )
    BookmarkableIDFilterType = graphql.GraphQLInputObjectType(
        name="BookmarkableIDFilter",
        fields=lambda: {
            "eq": graphql.GraphQLInputField(
                type_=BookmarkableIDType,
                out_name="eq",
            ),
            "in": graphql.GraphQLInputField(
                type_=graphql.GraphQLList(graphql.GraphQLNonNull(BookmarkableIDType)),
                out_name="in_",
            ),
        },
    )
    BookmarkableBookmarkInputType = graphql.GraphQLInputObjectType(
        name="BookmarkableBookmarkInput",
        fields=lambda: {
            "bookmarkableId": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(BookmarkableIDType),
                out_name="bookmarkable_id",
            ),
        },
    )
    BookmarkableBookmarkPayloadType = graphql.GraphQLObjectType(
        name="BookmarkableBookmarkPayload",
        fields=lambda: {
            "bookmarkableBookmarked": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(BookmarkableType),
                resolve=BuilderConfig.Bookmark.Bookmarkable.BookmarkableBookmarkPayload.bookmarkable_bookmarked,
            ),
        },
    )
    BookmarkableUnbookmarkInputType = graphql.GraphQLInputObjectType(
        name="BookmarkableUnbookmarkInput",
        fields=lambda: {
            "bookmarkableId": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(BookmarkableIDType),
                out_name="bookmarkable_id",
            ),
        },
    )
    BookmarkableUnbookmarkPayloadType = graphql.GraphQLObjectType(
        name="BookmarkableUnbookmarkPayload",
        fields=lambda: {
            "bookmarkableUnbookmarked": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(BookmarkableType),
                resolve=BuilderConfig.Bookmark.Bookmarkable.BookmarkableUnbookmarkPayload.bookmarkable_unbookmarked,
            ),
        },
    )
    CurrencyType = graphql.GraphQLEnumType(
        name="Currency",
        description="Currency (ISO 4217)\n",
        values={
            "AED": graphql.GraphQLEnumValue(
                value="AED",
                description="United Arab Emirates dirham\n",
            ),
            "AFN": graphql.GraphQLEnumValue(
                value="AFN",
                description="Afghan afghani\n",
            ),
            "ALL": graphql.GraphQLEnumValue(
                value="ALL",
                description="Albanian lek\n",
            ),
            "AMD": graphql.GraphQLEnumValue(
                value="AMD",
                description="Armenian dram\n",
            ),
            "ANG": graphql.GraphQLEnumValue(
                value="ANG",
                description="Netherlands Antillean guilder\n",
            ),
            "AOA": graphql.GraphQLEnumValue(
                value="AOA",
                description="Angolan kwanza\n",
            ),
            "ARS": graphql.GraphQLEnumValue(
                value="ARS",
                description="Argentine peso\n",
            ),
            "AUD": graphql.GraphQLEnumValue(
                value="AUD",
                description="Australian dollar\n",
            ),
            "AWG": graphql.GraphQLEnumValue(
                value="AWG",
                description="Aruban florin\n",
            ),
            "AZN": graphql.GraphQLEnumValue(
                value="AZN",
                description="Azerbaijani manat\n",
            ),
            "BAM": graphql.GraphQLEnumValue(
                value="BAM",
                description="Bosnia and Herzegovina convertible mark\n",
            ),
            "BBD": graphql.GraphQLEnumValue(
                value="BBD",
                description="Barbados dollar\n",
            ),
            "BDT": graphql.GraphQLEnumValue(
                value="BDT",
                description="Bangladeshi taka\n",
            ),
            "BGN": graphql.GraphQLEnumValue(
                value="BGN",
                description="Bulgarian lev\n",
            ),
            "BHD": graphql.GraphQLEnumValue(
                value="BHD",
                description="Bahraini dinar\n",
            ),
            "BIF": graphql.GraphQLEnumValue(
                value="BIF",
                description="Burundian franc\n",
            ),
            "BMD": graphql.GraphQLEnumValue(
                value="BMD",
                description="Bermudian dollar\n",
            ),
            "BND": graphql.GraphQLEnumValue(
                value="BND",
                description="Brunei dollar\n",
            ),
            "BOB": graphql.GraphQLEnumValue(
                value="BOB",
                description="Boliviano\n",
            ),
            "BOV": graphql.GraphQLEnumValue(
                value="BOV",
                description="Bolivian Mvdol (funds code)\n",
            ),
            "BRL": graphql.GraphQLEnumValue(
                value="BRL",
                description="Brazilian real\n",
            ),
            "BSD": graphql.GraphQLEnumValue(
                value="BSD",
                description="Bahamian dollar\n",
            ),
            "BTN": graphql.GraphQLEnumValue(
                value="BTN",
                description="Bhutanese ngultrum\n",
            ),
            "BWP": graphql.GraphQLEnumValue(
                value="BWP",
                description="Botswana pula\n",
            ),
            "BYN": graphql.GraphQLEnumValue(
                value="BYN",
                description="Belarusian ruble\n",
            ),
            "BZD": graphql.GraphQLEnumValue(
                value="BZD",
                description="Belize dollar\n",
            ),
            "CAD": graphql.GraphQLEnumValue(
                value="CAD",
                description="Canadian dollar\n",
            ),
            "CDF": graphql.GraphQLEnumValue(
                value="CDF",
                description="Congolese franc\n",
            ),
            "CHE": graphql.GraphQLEnumValue(
                value="CHE",
                description="WIR euro (complementary currency)\n",
            ),
            "CHF": graphql.GraphQLEnumValue(
                value="CHF",
                description="Swiss franc\n",
            ),
            "CHW": graphql.GraphQLEnumValue(
                value="CHW",
                description="WIR franc (complementary currency)\n",
            ),
            "CLF": graphql.GraphQLEnumValue(
                value="CLF",
                description="Unidad de Fomento (funds code)\n",
            ),
            "CLP": graphql.GraphQLEnumValue(
                value="CLP",
                description="Chilean peso\n",
            ),
            "CNY": graphql.GraphQLEnumValue(
                value="CNY",
                description="Chinese yuan\n",
            ),
            "COP": graphql.GraphQLEnumValue(
                value="COP",
                description="Colombian peso\n",
            ),
            "COU": graphql.GraphQLEnumValue(
                value="COU",
                description="Unidad de Valor Real (UVR) (funds code)\n",
            ),
            "CRC": graphql.GraphQLEnumValue(
                value="CRC",
                description="Costa Rican colon\n",
            ),
            "CUC": graphql.GraphQLEnumValue(
                value="CUC",
                description="Cuban convertible peso\n",
            ),
            "CUP": graphql.GraphQLEnumValue(
                value="CUP",
                description="Cuban peso\n",
            ),
            "CVE": graphql.GraphQLEnumValue(
                value="CVE",
                description="Cape Verdean escudo\n",
            ),
            "CZK": graphql.GraphQLEnumValue(
                value="CZK",
                description="Czech koruna\n",
            ),
            "DJF": graphql.GraphQLEnumValue(
                value="DJF",
                description="Djiboutian franc\n",
            ),
            "DKK": graphql.GraphQLEnumValue(
                value="DKK",
                description="Danish krone\n",
            ),
            "DOP": graphql.GraphQLEnumValue(
                value="DOP",
                description="Dominican peso\n",
            ),
            "DZD": graphql.GraphQLEnumValue(
                value="DZD",
                description="Algerian dinar\n",
            ),
            "EGP": graphql.GraphQLEnumValue(
                value="EGP",
                description="Egyptian pound\n",
            ),
            "ERN": graphql.GraphQLEnumValue(
                value="ERN",
                description="Eritrean nakfa\n",
            ),
            "ETB": graphql.GraphQLEnumValue(
                value="ETB",
                description="Ethiopian birr\n",
            ),
            "EUR": graphql.GraphQLEnumValue(
                value="EUR",
                description="Euro\n",
            ),
            "FJD": graphql.GraphQLEnumValue(
                value="FJD",
                description="Fiji dollar\n",
            ),
            "FKP": graphql.GraphQLEnumValue(
                value="FKP",
                description="Falkland Islands pound\n",
            ),
            "GBP": graphql.GraphQLEnumValue(
                value="GBP",
                description="Pound sterling\n",
            ),
            "GEL": graphql.GraphQLEnumValue(
                value="GEL",
                description="Georgian lari\n",
            ),
            "GHS": graphql.GraphQLEnumValue(
                value="GHS",
                description="Ghanaian cedi\n",
            ),
            "GIP": graphql.GraphQLEnumValue(
                value="GIP",
                description="Gibraltar pound\n",
            ),
            "GMD": graphql.GraphQLEnumValue(
                value="GMD",
                description="Gambian dalasi\n",
            ),
            "GNF": graphql.GraphQLEnumValue(
                value="GNF",
                description="Guinean franc\n",
            ),
            "GTQ": graphql.GraphQLEnumValue(
                value="GTQ",
                description="Guatemalan quetzal\n",
            ),
            "GYD": graphql.GraphQLEnumValue(
                value="GYD",
                description="Guyanese dollar\n",
            ),
            "HKD": graphql.GraphQLEnumValue(
                value="HKD",
                description="Hong Kong dollar\n",
            ),
            "HNL": graphql.GraphQLEnumValue(
                value="HNL",
                description="Honduran lempira\n",
            ),
            "HRK": graphql.GraphQLEnumValue(
                value="HRK",
                description="Croatian kuna\n",
            ),
            "HTG": graphql.GraphQLEnumValue(
                value="HTG",
                description="Haitian gourde\n",
            ),
            "HUF": graphql.GraphQLEnumValue(
                value="HUF",
                description="Hungarian forint\n",
            ),
            "IDR": graphql.GraphQLEnumValue(
                value="IDR",
                description="Indonesian rupiah\n",
            ),
            "ILS": graphql.GraphQLEnumValue(
                value="ILS",
                description="Israeli new shekel\n",
            ),
            "INR": graphql.GraphQLEnumValue(
                value="INR",
                description="Indian rupee\n",
            ),
            "IQD": graphql.GraphQLEnumValue(
                value="IQD",
                description="Iraqi dinar\n",
            ),
            "IRR": graphql.GraphQLEnumValue(
                value="IRR",
                description="Iranian rial\n",
            ),
            "ISK": graphql.GraphQLEnumValue(
                value="ISK",
                description="Icelandic króna (plural: krónur)\n",
            ),
            "JMD": graphql.GraphQLEnumValue(
                value="JMD",
                description="Jamaican dollar\n",
            ),
            "JOD": graphql.GraphQLEnumValue(
                value="JOD",
                description="Jordanian dinar\n",
            ),
            "JPY": graphql.GraphQLEnumValue(
                value="JPY",
                description="Japanese yen\n",
            ),
            "KES": graphql.GraphQLEnumValue(
                value="KES",
                description="Kenyan shilling\n",
            ),
            "KGS": graphql.GraphQLEnumValue(
                value="KGS",
                description="Kyrgyzstani som\n",
            ),
            "KHR": graphql.GraphQLEnumValue(
                value="KHR",
                description="Cambodian riel\n",
            ),
            "KMF": graphql.GraphQLEnumValue(
                value="KMF",
                description="Comoro franc\n",
            ),
            "KPW": graphql.GraphQLEnumValue(
                value="KPW",
                description="North Korean won\n",
            ),
            "KRW": graphql.GraphQLEnumValue(
                value="KRW",
                description="South Korean won\n",
            ),
            "KWD": graphql.GraphQLEnumValue(
                value="KWD",
                description="Kuwaiti dinar\n",
            ),
            "KYD": graphql.GraphQLEnumValue(
                value="KYD",
                description="Cayman Islands dollar\n",
            ),
            "KZT": graphql.GraphQLEnumValue(
                value="KZT",
                description="Kazakhstani tenge\n",
            ),
            "LAK": graphql.GraphQLEnumValue(
                value="LAK",
                description="Lao kip\n",
            ),
            "LBP": graphql.GraphQLEnumValue(
                value="LBP",
                description="Lebanese pound\n",
            ),
            "LKR": graphql.GraphQLEnumValue(
                value="LKR",
                description="Sri Lankan rupee\n",
            ),
            "LRD": graphql.GraphQLEnumValue(
                value="LRD",
                description="Liberian dollar\n",
            ),
            "LSL": graphql.GraphQLEnumValue(
                value="LSL",
                description="Lesotho loti\n",
            ),
            "LYD": graphql.GraphQLEnumValue(
                value="LYD",
                description="Libyan dinar\n",
            ),
            "MAD": graphql.GraphQLEnumValue(
                value="MAD",
                description="Moroccan dirham\n",
            ),
            "MDL": graphql.GraphQLEnumValue(
                value="MDL",
                description="Moldovan leu\n",
            ),
            "MGA": graphql.GraphQLEnumValue(
                value="MGA",
                description="Malagasy ariary\n",
            ),
            "MKD": graphql.GraphQLEnumValue(
                value="MKD",
                description="Macedonian denar\n",
            ),
            "MMK": graphql.GraphQLEnumValue(
                value="MMK",
                description="Myanmar kyat\n",
            ),
            "MNT": graphql.GraphQLEnumValue(
                value="MNT",
                description="Mongolian tögrög\n",
            ),
            "MOP": graphql.GraphQLEnumValue(
                value="MOP",
                description="Macanese pataca\n",
            ),
            "MRU": graphql.GraphQLEnumValue(
                value="MRU",
                description="Mauritanian ouguiya\n",
            ),
            "MUR": graphql.GraphQLEnumValue(
                value="MUR",
                description="Mauritian rupee\n",
            ),
            "MVR": graphql.GraphQLEnumValue(
                value="MVR",
                description="Maldivian rufiyaa\n",
            ),
            "MWK": graphql.GraphQLEnumValue(
                value="MWK",
                description="Malawian kwacha\n",
            ),
            "MXN": graphql.GraphQLEnumValue(
                value="MXN",
                description="Mexican peso\n",
            ),
            "MXV": graphql.GraphQLEnumValue(
                value="MXV",
                description="Mexican Unidad de Inversion (UDI) (funds code)\n",
            ),
            "MYR": graphql.GraphQLEnumValue(
                value="MYR",
                description="Malaysian ringgit\n",
            ),
            "MZN": graphql.GraphQLEnumValue(
                value="MZN",
                description="Mozambican metical\n",
            ),
            "NAD": graphql.GraphQLEnumValue(
                value="NAD",
                description="Namibian dollar\n",
            ),
            "NGN": graphql.GraphQLEnumValue(
                value="NGN",
                description="Nigerian naira\n",
            ),
            "NIO": graphql.GraphQLEnumValue(
                value="NIO",
                description="Nicaraguan córdoba\n",
            ),
            "NOK": graphql.GraphQLEnumValue(
                value="NOK",
                description="Norwegian krone\n",
            ),
            "NPR": graphql.GraphQLEnumValue(
                value="NPR",
                description="Nepalese rupee\n",
            ),
            "NZD": graphql.GraphQLEnumValue(
                value="NZD",
                description="New Zealand dollar\n",
            ),
            "OMR": graphql.GraphQLEnumValue(
                value="OMR",
                description="Omani rial\n",
            ),
            "PAB": graphql.GraphQLEnumValue(
                value="PAB",
                description="Panamanian balboa\n",
            ),
            "PEN": graphql.GraphQLEnumValue(
                value="PEN",
                description="Peruvian sol\n",
            ),
            "PGK": graphql.GraphQLEnumValue(
                value="PGK",
                description="Papua New Guinean kina\n",
            ),
            "PHP": graphql.GraphQLEnumValue(
                value="PHP",
                description="Philippine peso\n",
            ),
            "PKR": graphql.GraphQLEnumValue(
                value="PKR",
                description="Pakistani rupee\n",
            ),
            "PLN": graphql.GraphQLEnumValue(
                value="PLN",
                description="Polish złoty\n",
            ),
            "PYG": graphql.GraphQLEnumValue(
                value="PYG",
                description="Paraguayan guaraní\n",
            ),
            "QAR": graphql.GraphQLEnumValue(
                value="QAR",
                description="Qatari riyal\n",
            ),
            "RON": graphql.GraphQLEnumValue(
                value="RON",
                description="Romanian leu\n",
            ),
            "RSD": graphql.GraphQLEnumValue(
                value="RSD",
                description="Serbian dinar\n",
            ),
            "RUB": graphql.GraphQLEnumValue(
                value="RUB",
                description="Russian ruble\n",
            ),
            "RWF": graphql.GraphQLEnumValue(
                value="RWF",
                description="Rwandan franc\n",
            ),
            "SAR": graphql.GraphQLEnumValue(
                value="SAR",
                description="Saudi riyal\n",
            ),
            "SBD": graphql.GraphQLEnumValue(
                value="SBD",
                description="Solomon Islands dollar\n",
            ),
            "SCR": graphql.GraphQLEnumValue(
                value="SCR",
                description="Seychelles rupee\n",
            ),
            "SDG": graphql.GraphQLEnumValue(
                value="SDG",
                description="Sudanese pound\n",
            ),
            "SEK": graphql.GraphQLEnumValue(
                value="SEK",
                description="Swedish krona (plural: kronor)\n",
            ),
            "SGD": graphql.GraphQLEnumValue(
                value="SGD",
                description="Singapore dollar\n",
            ),
            "SHP": graphql.GraphQLEnumValue(
                value="SHP",
                description="Saint Helena pound\n",
            ),
            "SLL": graphql.GraphQLEnumValue(
                value="SLL",
                description="Sierra Leonean leone\n",
            ),
            "SOS": graphql.GraphQLEnumValue(
                value="SOS",
                description="Somali shilling\n",
            ),
            "SRD": graphql.GraphQLEnumValue(
                value="SRD",
                description="Surinamese dollar\n",
            ),
            "SSP": graphql.GraphQLEnumValue(
                value="SSP",
                description="South Sudanese pound\n",
            ),
            "STN": graphql.GraphQLEnumValue(
                value="STN",
                description="São Tomé and Príncipe dobra\n",
            ),
            "SVC": graphql.GraphQLEnumValue(
                value="SVC",
                description="Salvadoran colón\n",
            ),
            "SYP": graphql.GraphQLEnumValue(
                value="SYP",
                description="Syrian pound\n",
            ),
            "SZL": graphql.GraphQLEnumValue(
                value="SZL",
                description="Swazi lilangeni\n",
            ),
            "THB": graphql.GraphQLEnumValue(
                value="THB",
                description="Thai baht\n",
            ),
            "TJS": graphql.GraphQLEnumValue(
                value="TJS",
                description="Tajikistani somoni\n",
            ),
            "TMT": graphql.GraphQLEnumValue(
                value="TMT",
                description="Turkmenistan manat\n",
            ),
            "TND": graphql.GraphQLEnumValue(
                value="TND",
                description="Tunisian dinar\n",
            ),
            "TOP": graphql.GraphQLEnumValue(
                value="TOP",
                description="Tongan paʻanga\n",
            ),
            "TRY": graphql.GraphQLEnumValue(
                value="TRY",
                description="Turkish lira\n",
            ),
            "TTD": graphql.GraphQLEnumValue(
                value="TTD",
                description="Trinidad and Tobago dollar\n",
            ),
            "TWD": graphql.GraphQLEnumValue(
                value="TWD",
                description="New Taiwan dollar\n",
            ),
            "TZS": graphql.GraphQLEnumValue(
                value="TZS",
                description="Tanzanian shilling\n",
            ),
            "UAH": graphql.GraphQLEnumValue(
                value="UAH",
                description="Ukrainian hryvnia\n",
            ),
            "UGX": graphql.GraphQLEnumValue(
                value="UGX",
                description="Ugandan shilling\n",
            ),
            "USD": graphql.GraphQLEnumValue(
                value="USD",
                description="United States dollar\n",
            ),
            "USN": graphql.GraphQLEnumValue(
                value="USN",
                description="United States dollar (next day) (funds code)\n",
            ),
            "UYI": graphql.GraphQLEnumValue(
                value="UYI",
                description="Uruguay Peso en Unidades Indexadas (URUIURUI) (funds code)\n",
            ),
            "UYU": graphql.GraphQLEnumValue(
                value="UYU",
                description="Uruguayan peso\n",
            ),
            "UYW": graphql.GraphQLEnumValue(
                value="UYW",
                description="Unidad previsional\n",
            ),
            "UZS": graphql.GraphQLEnumValue(
                value="UZS",
                description="Uzbekistan som\n",
            ),
            "VED": graphql.GraphQLEnumValue(
                value="VED",
                description="Venezuelan bolívar digital\n",
            ),
            "VES": graphql.GraphQLEnumValue(
                value="VES",
                description="Venezuelan bolívar soberano\n",
            ),
            "VND": graphql.GraphQLEnumValue(
                value="VND",
                description="Vietnamese đồng\n",
            ),
            "VUV": graphql.GraphQLEnumValue(
                value="VUV",
                description="Vanuatu vatu\n",
            ),
            "WST": graphql.GraphQLEnumValue(
                value="WST",
                description="Samoan tala\n",
            ),
            "XAF": graphql.GraphQLEnumValue(
                value="XAF",
                description="CFA franc BEAC\n",
            ),
            "XAG": graphql.GraphQLEnumValue(
                value="XAG",
                description="Silver\n",
            ),
            "XAU": graphql.GraphQLEnumValue(
                value="XAU",
                description="Gold\n",
            ),
            "XBA": graphql.GraphQLEnumValue(
                value="XBA",
                description="European Composite Unit\n",
            ),
            "XBB": graphql.GraphQLEnumValue(
                value="XBB",
                description="European Monetary Unit\n",
            ),
            "XBC": graphql.GraphQLEnumValue(
                value="XBC",
                description="European Unit of Account 9\n",
            ),
            "XBD": graphql.GraphQLEnumValue(
                value="XBD",
                description="European Unit of Account 17\n",
            ),
            "XCD": graphql.GraphQLEnumValue(
                value="XCD",
                description="East Caribbean dollar\n",
            ),
            "XDR": graphql.GraphQLEnumValue(
                value="XDR",
                description="Special drawing rights\n",
            ),
            "XOF": graphql.GraphQLEnumValue(
                value="XOF",
                description="CFA franc BCEAO\n",
            ),
            "XPD": graphql.GraphQLEnumValue(
                value="XPD",
                description="Palladium\n",
            ),
            "XPF": graphql.GraphQLEnumValue(
                value="XPF",
                description="CFP franc\n",
            ),
            "XPT": graphql.GraphQLEnumValue(
                value="XPT",
                description="Platinum\n",
            ),
            "XSU": graphql.GraphQLEnumValue(
                value="XSU",
                description="SUCRE\n",
            ),
            "XTS": graphql.GraphQLEnumValue(
                value="XTS",
                description="Testing\n",
            ),
            "XUA": graphql.GraphQLEnumValue(
                value="XUA",
                description="ADB Unit of Account\n",
            ),
            "XXX": graphql.GraphQLEnumValue(
                value="XXX",
                description="No currency\n",
            ),
            "YER": graphql.GraphQLEnumValue(
                value="YER",
                description="Yemeni rial\n",
            ),
            "ZAR": graphql.GraphQLEnumValue(
                value="ZAR",
                description="South African rand\n",
            ),
            "ZMW": graphql.GraphQLEnumValue(
                value="ZMW",
                description="Zambian kwacha\n",
            ),
            "ZWL": graphql.GraphQLEnumValue(
                value="ZWL",
                description="Zimbabwean dollar\n",
            ),
        }
    )
    UserType = graphql.GraphQLObjectType(
        name="User",
        fields=lambda: {
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=BuilderConfig.IAM.User.User.id,
            ),
            "orders": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(OrderConnectionType),
                args={
                    "first": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="first",
                    ),
                    "after": graphql.GraphQLArgument(
                        type_=CursorType,
                        out_name="after",
                    ),
                    "last": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="last",
                    ),
                    "before": graphql.GraphQLArgument(
                        type_=CursorType,
                        out_name="before",
                    ),
                    "offset": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="offset",
                    ),
                    "filters": graphql.GraphQLArgument(
                        type_=graphql.GraphQLList(graphql.GraphQLNonNull(OrderFilterType)),
                        out_name="filters",
                    ),
                },
                resolve=BuilderConfig.Orders.Order.User.orders,
            ),
        },
        interfaces=lambda: [
            NodeType,
        ],
    )
    UserIDType = graphql.GraphQLScalarType(
        name="UserID",
        serialize=BuilderConfig.IAM.User.UserID.serialize,
        parse_value=BuilderConfig.IAM.User.UserID.parse_value,
        parse_literal=BuilderConfig.IAM.User.UserID.parse_literal,
    )
    UserFilterType = graphql.GraphQLInputObjectType(
        name="UserFilter",
        fields=lambda: {
            "id": graphql.GraphQLInputField(
                type_=UserIDFilterType,
                out_name="id",
            ),
            "q": graphql.GraphQLInputField(
                type_=graphql.GraphQLString,
                out_name="q",
            ),
        },
    )
    UserIDFilterType = graphql.GraphQLInputObjectType(
        name="UserIDFilter",
        fields=lambda: {
            "eq": graphql.GraphQLInputField(
                type_=UserIDType,
                out_name="eq",
            ),
            "in": graphql.GraphQLInputField(
                type_=graphql.GraphQLList(graphql.GraphQLNonNull(UserIDType)),
                out_name="in_",
            ),
        },
    )
    UserConnectionType = graphql.GraphQLObjectType(
        name="UserConnection",
        fields=lambda: {
            "edges": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(UserEdgeType))),
                resolve=graphql.default_field_resolver,
            ),
            "pageInfo": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(PageInfoType),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    UserEdgeType = graphql.GraphQLObjectType(
        name="UserEdge",
        fields=lambda: {
            "node": graphql.GraphQLField(
                type_=UserType,
                resolve=BuilderConfig.IAM.User.UserEdge.node,
            ),
            "cursor": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(CursorType),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    OrderType = graphql.GraphQLObjectType(
        name="Order",
        fields=lambda: {
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                resolve=BuilderConfig.Orders.Order.Order.id,
            ),
            "createdAt": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(TimestampType),
                resolve=graphql.default_field_resolver,
            ),
            "updatedAt": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(TimestampType),
                resolve=graphql.default_field_resolver,
            ),
            "orderer": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(UserType),
                resolve=BuilderConfig.Orders.Order.Order.orderer,
            ),
            "status": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(OrderStatusType),
                resolve=graphql.default_field_resolver,
            ),
            "destination": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
            "products": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(OrderProductType))),
                resolve=graphql.default_field_resolver,
            ),
            "currency": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(CurrencyType),
                resolve=graphql.default_field_resolver,
            ),
            "taxRate": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(DecimalType),
                resolve=graphql.default_field_resolver,
            ),
            "productsSubtotalAmount": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(DecimalType),
                resolve=graphql.default_field_resolver,
            ),
            "shippingAmount": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(DecimalType),
                resolve=graphql.default_field_resolver,
            ),
            "taxAmount": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(DecimalType),
                resolve=graphql.default_field_resolver,
            ),
            "totalAmount": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(DecimalType),
                resolve=graphql.default_field_resolver,
            ),
            "viewerHasBookmarked": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
                resolve=BuilderConfig.Orders.Order.Order.viewer_has_bookmarked,
            ),
        },
        interfaces=lambda: [
            NodeType,
            BookmarkableType,
        ],
    )
    OrderStatusType = graphql.GraphQLEnumType(
        name="OrderStatus",
        values={
            "DRAFT": graphql.GraphQLEnumValue(
                value="DRAFT",
            ),
            "PENDING": graphql.GraphQLEnumValue(
                value="PENDING",
            ),
            "CONFIRMED": graphql.GraphQLEnumValue(
                value="CONFIRMED",
            ),
            "CANCELLED": graphql.GraphQLEnumValue(
                value="CANCELLED",
            ),
            "SHIPPED": graphql.GraphQLEnumValue(
                value="SHIPPED",
            ),
            "DELIVERED": graphql.GraphQLEnumValue(
                value="DELIVERED",
            ),
        }
    )
    OrderProductType = graphql.GraphQLObjectType(
        name="OrderProduct",
        fields=lambda: {
            "description": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
            "quantity": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLInt),
                resolve=graphql.default_field_resolver,
            ),
            "unitPrice": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(DecimalType),
                resolve=graphql.default_field_resolver,
            ),
            "amount": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(DecimalType),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    OrderProductInputType = graphql.GraphQLInputObjectType(
        name="OrderProductInput",
        fields=lambda: {
            "description": graphql.GraphQLInputField(
                type_=graphql.GraphQLString,
                out_name="description",
            ),
            "quantity": graphql.GraphQLInputField(
                type_=graphql.GraphQLInt,
                out_name="quantity",
            ),
            "unitPrice": graphql.GraphQLInputField(
                type_=DecimalType,
                out_name="unit_price",
            ),
            "amount": graphql.GraphQLInputField(
                type_=DecimalType,
                out_name="amount",
            ),
        },
    )
    OrderIDType = graphql.GraphQLScalarType(
        name="OrderID",
        serialize=BuilderConfig.Orders.Order.OrderID.serialize,
        parse_value=BuilderConfig.Orders.Order.OrderID.parse_value,
        parse_literal=BuilderConfig.Orders.Order.OrderID.parse_literal,
    )
    OrderFilterType = graphql.GraphQLInputObjectType(
        name="OrderFilter",
        fields=lambda: {
            "id": graphql.GraphQLInputField(
                type_=OrderIDFilterType,
                out_name="id",
            ),
            "q": graphql.GraphQLInputField(
                type_=graphql.GraphQLString,
                out_name="q",
            ),
        },
    )
    OrderIDFilterType = graphql.GraphQLInputObjectType(
        name="OrderIDFilter",
        fields=lambda: {
            "eq": graphql.GraphQLInputField(
                type_=OrderIDType,
                out_name="eq",
            ),
            "in": graphql.GraphQLInputField(
                type_=graphql.GraphQLList(graphql.GraphQLNonNull(OrderIDType)),
                out_name="in_",
            ),
        },
    )
    OrderConnectionType = graphql.GraphQLObjectType(
        name="OrderConnection",
        fields=lambda: {
            "edges": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(OrderEdgeType))),
                resolve=graphql.default_field_resolver,
            ),
            "pageInfo": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(PageInfoType),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    OrderEdgeType = graphql.GraphQLObjectType(
        name="OrderEdge",
        fields=lambda: {
            "node": graphql.GraphQLField(
                type_=OrderType,
                resolve=BuilderConfig.Orders.Order.OrderEdge.node,
            ),
            "cursor": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(CursorType),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    OrderCreateInDraftInputType = graphql.GraphQLInputObjectType(
        name="OrderCreateInDraftInput",
        fields=lambda: {
            "products": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(OrderProductInputType))),
                out_name="products",
            ),
            "currency": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(CurrencyType),
                out_name="currency",
            ),
            "taxRate": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(DecimalType),
                out_name="tax_rate",
            ),
            "shippingAmount": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(DecimalType),
                out_name="shipping_amount",
            ),
            "destination": graphql.GraphQLInputField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                out_name="destination",
            ),
        },
    )
    OrderCreateInDraftPayloadType = graphql.GraphQLObjectType(
        name="OrderCreateInDraftPayload",
        fields=lambda: {
            "orderCreatedInDraft": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(OrderType),
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    NodeType = graphql.GraphQLInterfaceType(
        name="Node",
        description="An object with an ID\n",
        fields=lambda: {
            "id": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                description="The id of the object.\n",
            ),
        },
    )
    PageInfoType = graphql.GraphQLObjectType(
        name="PageInfo",
        description="Information about pagination in a connection.\n",
        fields=lambda: {
            "hasNextPage": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
                description="When paginating forwards, are there more items?\n",
                resolve=graphql.default_field_resolver,
            ),
            "hasPreviousPage": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLBoolean),
                description="When paginating backwards, are there more items?\n",
                resolve=graphql.default_field_resolver,
            ),
            "startCursor": graphql.GraphQLField(
                type_=CursorType,
                description="When paginating backwards, the cursor to continue.\n",
                resolve=graphql.default_field_resolver,
            ),
            "endCursor": graphql.GraphQLField(
                type_=CursorType,
                description="When paginating forwards, the cursor to continue.\n",
                resolve=graphql.default_field_resolver,
            ),
        },
    )
    CursorType = graphql.GraphQLScalarType(
        name="Cursor",
        serialize=BuilderConfig.Relay.Cursor.serialize,
        parse_value=BuilderConfig.Relay.Cursor.parse_value,
        parse_literal=BuilderConfig.Relay.Cursor.parse_literal,
    )
    QueryType = graphql.GraphQLObjectType(
        name="Query",
        fields=lambda: {
            "version": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=BuilderConfig.Root.Query.version,
            ),
            "node": graphql.GraphQLField(
                type_=NodeType,
                description="Fetches an object given its ID\n",
                args={
                    "id": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLID),
                        out_name="id",
                    ),
                },
                resolve=BuilderConfig.Relay.Query.node,
            ),
            "nodes": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLList(NodeType)),
                description="Fetches objects given their IDs\n",
                args={
                    "ids": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(graphql.GraphQLList(graphql.GraphQLNonNull(graphql.GraphQLID))),
                        out_name="ids",
                    ),
                },
                resolve=BuilderConfig.Relay.Query.nodes,
            ),
            "user": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(UserType),
                args={
                    "id": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(UserIDType),
                        out_name="id",
                    ),
                },
                resolve=BuilderConfig.IAM.User.Query.user,
            ),
            "userConnection": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(UserConnectionType),
                args={
                    "first": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="first",
                    ),
                    "after": graphql.GraphQLArgument(
                        type_=CursorType,
                        out_name="after",
                    ),
                    "last": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="last",
                    ),
                    "before": graphql.GraphQLArgument(
                        type_=CursorType,
                        out_name="before",
                    ),
                    "offset": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="offset",
                    ),
                    "filter": graphql.GraphQLArgument(
                        type_=UserFilterType,
                        out_name="filter",
                    ),
                },
                resolve=BuilderConfig.IAM.User.Query.user_connection,
            ),
            "viewer": graphql.GraphQLField(
                type_=UserType,
                resolve=BuilderConfig.IAM.Viewer.Query.viewer,
            ),
            "order": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(OrderType),
                args={
                    "id": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(OrderIDType),
                        out_name="id",
                    ),
                },
                resolve=BuilderConfig.Orders.Order.Query.order,
            ),
            "orderConnection": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(OrderConnectionType),
                args={
                    "first": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="first",
                    ),
                    "after": graphql.GraphQLArgument(
                        type_=CursorType,
                        out_name="after",
                    ),
                    "last": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="last",
                    ),
                    "before": graphql.GraphQLArgument(
                        type_=CursorType,
                        out_name="before",
                    ),
                    "offset": graphql.GraphQLArgument(
                        type_=graphql.GraphQLInt,
                        out_name="offset",
                    ),
                    "filters": graphql.GraphQLArgument(
                        type_=graphql.GraphQLList(graphql.GraphQLNonNull(OrderFilterType)),
                        out_name="filters",
                    ),
                },
                resolve=BuilderConfig.Orders.Order.Query.order_connection,
            ),
        },
    )
    MutationType = graphql.GraphQLObjectType(
        name="Mutation",
        fields=lambda: {
            "version": graphql.GraphQLField(
                type_=graphql.GraphQLNonNull(graphql.GraphQLString),
                resolve=graphql.default_field_resolver,
            ),
            "bookmarkBookmarkable": graphql.GraphQLField(
                type_=BookmarkableBookmarkPayloadType,
                args={
                    "input": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(BookmarkableBookmarkInputType),
                        out_name="input",
                    ),
                },
                resolve=BuilderConfig.Bookmark.Bookmarkable.Mutation.bookmark_bookmarkable,
            ),
            "unbookmarkBookmarkable": graphql.GraphQLField(
                type_=BookmarkableUnbookmarkPayloadType,
                args={
                    "input": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(BookmarkableUnbookmarkInputType),
                        out_name="input",
                    ),
                },
                resolve=BuilderConfig.Bookmark.Bookmarkable.Mutation.unbookmark_bookmarkable,
            ),
            "orderCreateInDraft": graphql.GraphQLField(
                type_=OrderCreateInDraftPayloadType,
                args={
                    "input": graphql.GraphQLArgument(
                        type_=graphql.GraphQLNonNull(OrderCreateInDraftInputType),
                        out_name="input",
                    ),
                },
                resolve=BuilderConfig.Orders.Order.Mutation.order_create_in_draft,
            ),
        },
    )
    TimestampType = graphql.GraphQLScalarType(
        name="Timestamp",
        description="An [ISO-8601](https://en.wikipedia.org/wiki/ISO_8601) encoded UTC date string.\n",
        serialize=BuilderConfig.Root.Timestamp.serialize,
        parse_value=BuilderConfig.Root.Timestamp.parse_value,
        parse_literal=BuilderConfig.Root.Timestamp.parse_literal,
    )
    DecimalType = graphql.GraphQLScalarType(
        name="Decimal",
        description="Decimal floating point partially implmenting http://speleotrove.com/decimal/decarith.html\n",
        serialize=BuilderConfig.Root.Decimal.serialize,
        parse_value=BuilderConfig.Root.Decimal.parse_value,
        parse_literal=BuilderConfig.Root.Decimal.parse_literal,
    )
    TypeIDType = graphql.GraphQLScalarType(
        name="TypeID",
        serialize=BuilderConfig.Root.TypeID.serialize,
        parse_value=BuilderConfig.Root.TypeID.parse_value,
        parse_literal=BuilderConfig.Root.TypeID.parse_literal,
    )
    return graphql.GraphQLSchema(
        query=QueryType,
        mutation=MutationType,
        types=[
            BookmarkType,
            BookmarkIDType,
            BookmarkIDFilterType,
            BookmarkFilterType,
            BookmarkConnectionType,
            BookmarkEdgeType,
            BookmarkableType,
            BookmarkableIDType,
            BookmarkableIDFilterType,
            BookmarkableBookmarkInputType,
            BookmarkableBookmarkPayloadType,
            BookmarkableUnbookmarkInputType,
            BookmarkableUnbookmarkPayloadType,
            CurrencyType,
            UserType,
            UserIDType,
            UserFilterType,
            UserIDFilterType,
            UserConnectionType,
            UserEdgeType,
            OrderType,
            OrderStatusType,
            OrderProductType,
            OrderProductInputType,
            OrderIDType,
            OrderFilterType,
            OrderIDFilterType,
            OrderConnectionType,
            OrderEdgeType,
            OrderCreateInDraftInputType,
            OrderCreateInDraftPayloadType,
            NodeType,
            PageInfoType,
            CursorType,
            QueryType,
            MutationType,
            TimestampType,
            DecimalType,
            TypeIDType,
        ],
    )
