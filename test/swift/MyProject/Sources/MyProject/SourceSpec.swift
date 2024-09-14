struct SourceSpec {
    protocol BasicObject: BasicUnion {
        var idNonNull: String { get }
        var stringNonNull: String { get }
        var intNonNull: Int { get }
        var floatNonNull: Float { get }
        var booleanNonNull: Bool { get }
        var id: String? { get }
        var string: String? { get }
        var int: Int? { get }
        var float: Float? { get }
        var boolean: Bool? { get }
        var basicScalar: BasicScalar? { get }
        var extendedField: String { get }
    }

    protocol BasicInterface {
        var interfaceField: String? { get }
        var basicScalar: BasicScalar? { get }
        var extendedField: String { get }
        var anotherField: String { get }
    }

    protocol BasicInterfaceImpl: BasicInterface, AnotherInterface, BasicUnion {
        var interfaceField: String? { get }
        var basicScalar: BasicScalar? { get }
        var extendedField: String { get }
        var anotherField: String { get }
    }

    protocol BasicUnion {}

    enum BasicEnum {
        case ENUM_VALUE_1
        case ENUM_VALUE_2
        case EXTENDED_VALUE
    }

    struct BasicInput: Decodable {
        let idNonNull: (value: String, isSet: Bool)
        let stringNonNull: (value: String, isSet: Bool)
        let intNonNull: (value: Int, isSet: Bool)
        let floatNonNull: (value: Float, isSet: Bool)
        let booleanNonNull: (value: Bool, isSet: Bool)
        let basicScalarNotNull: (value: BasicScalar, isSet: Bool)
        let id: (value: String?, isSet: Bool)
        let string: (value: String?, isSet: Bool)
        let int: (value: Int?, isSet: Bool)
        let float: (value: Float?, isSet: Bool)
        let boolean: (value: Bool?, isSet: Bool)
        let basicScalar: (value: BasicScalar?, isSet: Bool)
        let extendedField: (value: String, isSet: Bool)
        init(from decoder: Decoder) throws {
            let container = try decoder.container(keyedBy: CodingKeys.self)
            idNonNull = (value: try container.decode(String.self, forKey: .idNonNull), isSet: container.contains(.idNonNull))
            stringNonNull = (value: try container.decode(String.self, forKey: .stringNonNull), isSet: container.contains(.stringNonNull))
            intNonNull = (value: try container.decode(Int.self, forKey: .intNonNull), isSet: container.contains(.intNonNull))
            floatNonNull = (value: try container.decode(Float.self, forKey: .floatNonNull), isSet: container.contains(.floatNonNull))
            booleanNonNull = (value: try container.decode(Bool.self, forKey: .booleanNonNull), isSet: container.contains(.booleanNonNull))
            basicScalarNotNull = (value: try container.decode(BasicScalar.self, forKey: .basicScalarNotNull), isSet: container.contains(.basicScalarNotNull))
            id = (value: try container.decode(String?.self, forKey: .id), isSet: container.contains(.id))
            string = (value: try container.decode(String?.self, forKey: .string), isSet: container.contains(.string))
            int = (value: try container.decode(Int?.self, forKey: .int), isSet: container.contains(.int))
            float = (value: try container.decode(Float?.self, forKey: .float), isSet: container.contains(.float))
            boolean = (value: try container.decode(Bool?.self, forKey: .boolean), isSet: container.contains(.boolean))
            basicScalar = (value: try container.decode(BasicScalar?.self, forKey: .basicScalar), isSet: container.contains(.basicScalar))
            extendedField = (value: try container.decode(String.self, forKey: .extendedField), isSet: container.contains(.extendedField))
        }
        enum CodingKeys: String, CodingKey {
            case idNonNull
            case stringNonNull
            case intNonNull
            case floatNonNull
            case booleanNonNull
            case basicScalarNotNull
            case id
            case string
            case int
            case float
            case boolean
            case basicScalar
            case extendedField
        }
    }


    typealias BasicScalar = String

    protocol AnotherInterface {
        var anotherField: String { get }
    }

    protocol AnotherType: BasicUnion {
        var anotherField: String { get }
    }

    protocol DeprecatedFieldObject {
        var deprecatedField: String? { get }
    }

    protocol ModuleB {
        var id: String { get }
        var name: String { get }
    }

    protocol ModuleA {
        var id: String { get }
        var name: String { get }
    }

    protocol Nested2 {
        var value: String? { get }
    }

    protocol Nested1 {
        var nested2: Nested2? { get }
    }

    protocol GrandParent {
    }

    protocol Parent {
    }

    protocol Child: GrandParent, Parent {
    }

    protocol GraphQLObject {
        var foo: String { get }
    }

    protocol graphql {
        var bar: String { get }
    }

    protocol typing {
        var baz: String { get }
    }

    protocol Query {
        var version: String? { get }
    }

    protocol Mutation {
        var version: String? { get }
    }

    typealias DateTime = String

    protocol Source {
    }

}
