struct SourceSpec {
  protocol BasicObject: BasicUnion {
    var idNonNull: Any { get }
    var stringNonNull: String { get }
    var intNonNull: Int { get }
    var floatNonNull: Float { get }
    var booleanNonNull: Bool { get }
    var id: Any? { get }
    var string: String? { get }
    var int: Int? { get }
    var float: Float? { get }
    var boolean: Bool? { get }
    var basicScalar: BasicScalar? { get }
    var extendedField: Any { get }
  }

  protocol BasicInterface {
    var interfaceField: String? { get }
    var basicScalar: BasicScalar? { get }
    var extendedField: Any { get }
    var anotherField: Any { get }
  }

  protocol BasicInterfaceImpl: BasicInterface, AnotherInterface, BasicUnion {
    var interfaceField: String? { get }
    var basicScalar: BasicScalar? { get }
    var extendedField: Any { get }
    var anotherField: Any { get }
  }

  protocol BasicUnion {}

  enum BasicEnum {
    case ENUM_VALUE_1
    case ENUM_VALUE_2
    case EXTENDED_VALUE
  }

  struct BasicInput {
    let idNonNull: (value: Any, isSet: Bool)
    let stringNonNull: (value: String, isSet: Bool)
    let intNonNull: (value: Int, isSet: Bool)
    let floatNonNull: (value: Float, isSet: Bool)
    let booleanNonNull: (value: Bool, isSet: Bool)
    let basicScalarNotNull: (value: BasicScalar, isSet: Bool)
    let id: (value: Any?, isSet: Bool)
    let string: (value: String?, isSet: Bool)
    let int: (value: Int?, isSet: Bool)
    let float: (value: Float?, isSet: Bool)
    let boolean: (value: Bool?, isSet: Bool)
    let basicScalar: (value: BasicScalar?, isSet: Bool)
    let extendedField: (value: Any, isSet: Bool)
  }


  typealias BasicScalar = Any

  protocol AnotherInterface {
    var anotherField: Any { get }
  }

  protocol AnotherType: BasicUnion {
    var anotherField: Any { get }
  }

  protocol DeprecatedFieldObject {
    var deprecatedField: String? { get }
  }

  protocol ModuleB {
    var id: Any { get }
    var name: String { get }
  }

  protocol ModuleA {
    var id: Any { get }
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

  typealias DateTime = Any

  protocol Source {
  }

}