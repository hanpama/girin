struct SourceSpec {
    enum Currency {
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

    protocol User: Node {
    }

    typealias UserID = String

    struct UserFilter: Decodable {
        let id: (value: UserIDFilter?, isSet: Bool)
        let q: (value: String?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            id = (value: try container.decode(UserIDFilter?.self, forKey: .id), isSet: container.contains(.id))
            q = (value: try container.decode(String?.self, forKey: .q), isSet: container.contains(.q))
        }
        enum CodingKeys: String, CodingKey {
            case id
            case q
        }
    }


    struct UserIDFilter: Decodable {
        let eq: (value: UserID?, isSet: Bool)
        let in_: (value: [UserID]?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            eq = (value: try container.decode(UserID?.self, forKey: .eq), isSet: container.contains(.eq))
            in_ = (value: try container.decode([UserID]?.self, forKey: .in_), isSet: container.contains(.in_))
        }
        enum CodingKeys: String, CodingKey {
            case eq
            case in_
        }
    }


    protocol UserConnection {
        var edges: [UserEdge] { get }
        var pageInfo: PageInfo { get }
    }

    protocol UserEdge {
        var cursor: Cursor { get }
    }

    protocol Order: Node {
        var createdAt: Timestamp { get }
        var updatedAt: Timestamp { get }
        var ordererId: UserID!? { get }
        var status: OrderStatus { get }
        var destination: String { get }
        var products: [OrderProduct] { get }
        var currency: Currency { get }
        var taxRate: Decimal { get }
        var productsSubtotalAmount: Decimal { get }
        var shippingAmount: Decimal { get }
        var taxAmount: Decimal { get }
        var totalAmount: Decimal { get }
    }

    enum OrderStatus {
        case DRAFT
        case PENDING
        case CONFIRMED
        case CANCELLED
        case SHIPPED
        case DELIVERED
    }

    protocol OrderProduct {
        var description: String { get }
        var quantity: Int { get }
        var unitPrice: Decimal { get }
        var amount: Decimal { get }
    }

    struct OrderProductInput: Decodable {
        let description: (value: String?, isSet: Bool)
        let quantity: (value: Int?, isSet: Bool)
        let unitPrice: (value: Decimal?, isSet: Bool)
        let amount: (value: Decimal?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            description = (value: try container.decode(String?.self, forKey: .description), isSet: container.contains(.description))
            quantity = (value: try container.decode(Int?.self, forKey: .quantity), isSet: container.contains(.quantity))
            unitPrice = (value: try container.decode(Decimal?.self, forKey: .unitPrice), isSet: container.contains(.unitPrice))
            amount = (value: try container.decode(Decimal?.self, forKey: .amount), isSet: container.contains(.amount))
        }
        enum CodingKeys: String, CodingKey {
            case description
            case quantity
            case unitPrice
            case amount
        }
    }


    typealias OrderID = String

    struct OrderFilter: Decodable {
        let id: (value: OrderIDFilter?, isSet: Bool)
        let q: (value: String?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            id = (value: try container.decode(OrderIDFilter?.self, forKey: .id), isSet: container.contains(.id))
            q = (value: try container.decode(String?.self, forKey: .q), isSet: container.contains(.q))
        }
        enum CodingKeys: String, CodingKey {
            case id
            case q
        }
    }


    struct OrderIDFilter: Decodable {
        let eq: (value: OrderID?, isSet: Bool)
        let in_: (value: [OrderID]?, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            eq = (value: try container.decode(OrderID?.self, forKey: .eq), isSet: container.contains(.eq))
            in_ = (value: try container.decode([OrderID]?.self, forKey: .in_), isSet: container.contains(.in_))
        }
        enum CodingKeys: String, CodingKey {
            case eq
            case in_
        }
    }


    protocol OrderConnection {
        var edges: [OrderEdge] { get }
        var pageInfo: PageInfo { get }
    }

    protocol OrderEdge {
        var cursor: Cursor { get }
    }

    struct OrderCreateInDraftInput: Decodable {
        let products: (value: [OrderProductInput], isSet: Bool)
        let currency: (value: Currency, isSet: Bool)
        let taxRate: (value: Decimal, isSet: Bool)
        let shippingAmount: (value: Decimal, isSet: Bool)
        let destination: (value: String, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            products = (value: try container.decode([OrderProductInput].self, forKey: .products), isSet: container.contains(.products))
            currency = (value: try container.decode(Currency.self, forKey: .currency), isSet: container.contains(.currency))
            taxRate = (value: try container.decode(Decimal.self, forKey: .taxRate), isSet: container.contains(.taxRate))
            shippingAmount = (value: try container.decode(Decimal.self, forKey: .shippingAmount), isSet: container.contains(.shippingAmount))
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


    protocol OrderCreateInDraftPayload {
        var orderCreatedInDraft: Order { get }
    }

    protocol Node {
        var id: ID!? { get }
    }

    protocol PageInfo {
        var hasNextPage: Bool { get }
        var hasPreviousPage: Bool { get }
        var startCursor: Cursor? { get }
        var endCursor: Cursor? { get }
    }

    typealias Cursor = String

    protocol Query {
    }

    protocol Mutation {
        var version: String { get }
    }

    typealias Timestamp = String

    typealias Decimal = String

}
