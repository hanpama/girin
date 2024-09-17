import Foundation

struct SourceSpec {
    protocol BookmarkSource: NodeSource {
        var id: String { get }
        var createdAt: TimestampSource { get }
        var bookmarkerId: String { get }
        var bookmarkableId: BookmarkableIDSource { get }
        var bookmarkableTypeId: TypeIDSource { get }
    }

    typealias BookmarkIDSource = UUID

    struct BookmarkIDFilterSource: Decodable {
        let eq: (value: BookmarkIDSource?, isSet: Bool)
        let in_: (value: [BookmarkIDSource]?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            eq = (value: try container.decode(BookmarkIDSource?.self, forKey: .eq), isSet: container.contains(.eq))
            in_ = (value: try container.decode([BookmarkIDSource]?.self, forKey: .in_), isSet: container.contains(.in_))
        }
        enum CodingKeys: String, CodingKey {
            case eq
            case in_
        }
    }


    struct BookmarkFilterSource: Decodable {
        let id: (value: BookmarkIDFilterSource?, isSet: Bool)
        let bookmarkableId: (value: BookmarkableIDFilterSource?, isSet: Bool)
        let bookmarkerId: (value: UserIDFilterSource?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            id = (value: try container.decode(BookmarkIDFilterSource?.self, forKey: .id), isSet: container.contains(.id))
            bookmarkableId = (value: try container.decode(BookmarkableIDFilterSource?.self, forKey: .bookmarkableId), isSet: container.contains(.bookmarkableId))
            bookmarkerId = (value: try container.decode(UserIDFilterSource?.self, forKey: .bookmarkerId), isSet: container.contains(.bookmarkerId))
        }
        enum CodingKeys: String, CodingKey {
            case id
            case bookmarkableId
            case bookmarkerId
        }
    }


    protocol BookmarkConnectionSource {
        var edges: [BookmarkEdgeSource] { get }
        var pageInfo: PageInfoSource { get }
    }

    protocol BookmarkEdgeSource {
        var cursor: CursorSource { get }
    }

    protocol BookmarkableSource {
    }

    typealias BookmarkableIDSource = BookmarkableID

    struct BookmarkableIDFilterSource: Decodable {
        let eq: (value: BookmarkableIDSource?, isSet: Bool)
        let in_: (value: [BookmarkableIDSource]?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            eq = (value: try container.decode(BookmarkableIDSource?.self, forKey: .eq), isSet: container.contains(.eq))
            in_ = (value: try container.decode([BookmarkableIDSource]?.self, forKey: .in_), isSet: container.contains(.in_))
        }
        enum CodingKeys: String, CodingKey {
            case eq
            case in_
        }
    }


    struct BookmarkableBookmarkInputSource: Decodable {
        let bookmarkableId: (value: BookmarkableIDSource, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            bookmarkableId = (value: try container.decode(BookmarkableIDSource.self, forKey: .bookmarkableId), isSet: container.contains(.bookmarkableId))
        }
        enum CodingKeys: String, CodingKey {
            case bookmarkableId
        }
    }


    protocol BookmarkableBookmarkPayloadSource {
        var bookmarkableBookmarkedId: BookmarkableIDSource { get }
    }

    struct BookmarkableUnbookmarkInputSource: Decodable {
        let bookmarkableId: (value: BookmarkableIDSource, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            bookmarkableId = (value: try container.decode(BookmarkableIDSource.self, forKey: .bookmarkableId), isSet: container.contains(.bookmarkableId))
        }
        enum CodingKeys: String, CodingKey {
            case bookmarkableId
        }
    }


    protocol BookmarkableUnbookmarkPayloadSource {
        var bookmarkableUnbookmarkedId: BookmarkableIDSource { get }
    }

    enum CurrencySource: String, Codable {
        case AED
        case AFN
        case ALL
        case AMD
        case ANG
        case AOA
        case ARS
        case AUD
        case AWG
        case AZN
        case BAM
        case BBD
        case BDT
        case BGN
        case BHD
        case BIF
        case BMD
        case BND
        case BOB
        case BOV
        case BRL
        case BSD
        case BTN
        case BWP
        case BYN
        case BZD
        case CAD
        case CDF
        case CHE
        case CHF
        case CHW
        case CLF
        case CLP
        case CNY
        case COP
        case COU
        case CRC
        case CUC
        case CUP
        case CVE
        case CZK
        case DJF
        case DKK
        case DOP
        case DZD
        case EGP
        case ERN
        case ETB
        case EUR
        case FJD
        case FKP
        case GBP
        case GEL
        case GHS
        case GIP
        case GMD
        case GNF
        case GTQ
        case GYD
        case HKD
        case HNL
        case HRK
        case HTG
        case HUF
        case IDR
        case ILS
        case INR
        case IQD
        case IRR
        case ISK
        case JMD
        case JOD
        case JPY
        case KES
        case KGS
        case KHR
        case KMF
        case KPW
        case KRW
        case KWD
        case KYD
        case KZT
        case LAK
        case LBP
        case LKR
        case LRD
        case LSL
        case LYD
        case MAD
        case MDL
        case MGA
        case MKD
        case MMK
        case MNT
        case MOP
        case MRU
        case MUR
        case MVR
        case MWK
        case MXN
        case MXV
        case MYR
        case MZN
        case NAD
        case NGN
        case NIO
        case NOK
        case NPR
        case NZD
        case OMR
        case PAB
        case PEN
        case PGK
        case PHP
        case PKR
        case PLN
        case PYG
        case QAR
        case RON
        case RSD
        case RUB
        case RWF
        case SAR
        case SBD
        case SCR
        case SDG
        case SEK
        case SGD
        case SHP
        case SLL
        case SOS
        case SRD
        case SSP
        case STN
        case SVC
        case SYP
        case SZL
        case THB
        case TJS
        case TMT
        case TND
        case TOP
        case TRY
        case TTD
        case TWD
        case TZS
        case UAH
        case UGX
        case USD
        case USN
        case UYI
        case UYU
        case UYW
        case UZS
        case VED
        case VES
        case VND
        case VUV
        case WST
        case XAF
        case XAG
        case XAU
        case XBA
        case XBB
        case XBC
        case XBD
        case XCD
        case XDR
        case XOF
        case XPD
        case XPF
        case XPT
        case XSU
        case XTS
        case XUA
        case XXX
        case YER
        case ZAR
        case ZMW
        case ZWL
    }

    protocol UserSource: NodeSource {
    }

    typealias UserIDSource = UUID

    struct UserFilterSource: Decodable {
        let id: (value: UserIDFilterSource?, isSet: Bool)
        let q: (value: String?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            id = (value: try container.decode(UserIDFilterSource?.self, forKey: .id), isSet: container.contains(.id))
            q = (value: try container.decode(String?.self, forKey: .q), isSet: container.contains(.q))
        }
        enum CodingKeys: String, CodingKey {
            case id
            case q
        }
    }


    struct UserIDFilterSource: Decodable {
        let eq: (value: UserIDSource?, isSet: Bool)
        let in_: (value: [UserIDSource]?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            eq = (value: try container.decode(UserIDSource?.self, forKey: .eq), isSet: container.contains(.eq))
            in_ = (value: try container.decode([UserIDSource]?.self, forKey: .in_), isSet: container.contains(.in_))
        }
        enum CodingKeys: String, CodingKey {
            case eq
            case in_
        }
    }


    protocol UserConnectionSource {
        var edges: [UserEdgeSource] { get }
        var pageInfo: PageInfoSource { get }
    }

    protocol UserEdgeSource {
        var cursor: CursorSource { get }
    }

    protocol OrderSource: NodeSource, BookmarkableSource {
        var createdAt: TimestampSource { get }
        var updatedAt: TimestampSource { get }
        var ordererId: UserIDSource { get }
        var status: OrderStatusSource { get }
        var destination: String { get }
        var products: [OrderProductSource] { get }
        var currency: CurrencySource { get }
        var taxRate: DecimalSource { get }
        var productsSubtotalAmount: DecimalSource { get }
        var shippingAmount: DecimalSource { get }
        var taxAmount: DecimalSource { get }
        var totalAmount: DecimalSource { get }
    }

    enum OrderStatusSource: String, Codable {
        case DRAFT
        case PENDING
        case CONFIRMED
        case CANCELLED
        case SHIPPED
        case DELIVERED
    }

    protocol OrderProductSource {
        var description: String { get }
        var quantity: Int { get }
        var unitPrice: DecimalSource { get }
        var amount: DecimalSource { get }
    }

    struct OrderProductInputSource: Decodable {
        let description: (value: String?, isSet: Bool)
        let quantity: (value: Int?, isSet: Bool)
        let unitPrice: (value: DecimalSource?, isSet: Bool)
        let amount: (value: DecimalSource?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            description = (value: try container.decode(String?.self, forKey: .description), isSet: container.contains(.description))
            quantity = (value: try container.decode(Int?.self, forKey: .quantity), isSet: container.contains(.quantity))
            unitPrice = (value: try container.decode(DecimalSource?.self, forKey: .unitPrice), isSet: container.contains(.unitPrice))
            amount = (value: try container.decode(DecimalSource?.self, forKey: .amount), isSet: container.contains(.amount))
        }
        enum CodingKeys: String, CodingKey {
            case description
            case quantity
            case unitPrice
            case amount
        }
    }


    typealias OrderIDSource = UUID

    struct OrderFilterSource: Decodable {
        let id: (value: OrderIDFilterSource?, isSet: Bool)
        let q: (value: String?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            id = (value: try container.decode(OrderIDFilterSource?.self, forKey: .id), isSet: container.contains(.id))
            q = (value: try container.decode(String?.self, forKey: .q), isSet: container.contains(.q))
        }
        enum CodingKeys: String, CodingKey {
            case id
            case q
        }
    }


    struct OrderIDFilterSource: Decodable {
        let eq: (value: OrderIDSource?, isSet: Bool)
        let in_: (value: [OrderIDSource]?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            eq = (value: try container.decode(OrderIDSource?.self, forKey: .eq), isSet: container.contains(.eq))
            in_ = (value: try container.decode([OrderIDSource]?.self, forKey: .in_), isSet: container.contains(.in_))
        }
        enum CodingKeys: String, CodingKey {
            case eq
            case in_
        }
    }


    protocol OrderConnectionSource {
        var edges: [OrderEdgeSource] { get }
        var pageInfo: PageInfoSource { get }
    }

    protocol OrderEdgeSource {
        var cursor: CursorSource { get }
    }

    struct OrderCreateInDraftInputSource: Decodable {
        let products: (value: [OrderProductInputSource], isSet: Bool)
        let currency: (value: CurrencySource, isSet: Bool)
        let taxRate: (value: DecimalSource, isSet: Bool)
        let shippingAmount: (value: DecimalSource, isSet: Bool)
        let destination: (value: String, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            products = (value: try container.decode([OrderProductInputSource].self, forKey: .products), isSet: container.contains(.products))
            currency = (value: try container.decode(CurrencySource.self, forKey: .currency), isSet: container.contains(.currency))
            taxRate = (value: try container.decode(DecimalSource.self, forKey: .taxRate), isSet: container.contains(.taxRate))
            shippingAmount = (value: try container.decode(DecimalSource.self, forKey: .shippingAmount), isSet: container.contains(.shippingAmount))
            destination = (value: try container.decode(String.self, forKey: .destination), isSet: container.contains(.destination))
        }
        enum CodingKeys: String, CodingKey {
            case products
            case currency
            case taxRate
            case shippingAmount
            case destination
        }
    }


    protocol OrderCreateInDraftPayloadSource {
        var orderCreatedInDraft: OrderSource { get }
    }

    protocol NodeSource {
    }

    protocol PageInfoSource {
        var hasNextPage: Bool { get }
        var hasPreviousPage: Bool { get }
        var startCursor: CursorSource? { get }
        var endCursor: CursorSource? { get }
    }

    typealias CursorSource = String

    protocol QuerySource {
    }

    protocol MutationSource {
        var version: String { get }
    }

    typealias TimestampSource = Foundation.Date

    typealias DecimalSource = Foundation.Decimal

    typealias TypeIDSource = Foundation.UUID

}
