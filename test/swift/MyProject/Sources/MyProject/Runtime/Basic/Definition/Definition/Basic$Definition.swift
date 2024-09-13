extension RuntimeSpec.Basic.Definition {
  init() {
    self.BasicObject = .init(
      idNonNullArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      stringNonNullArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      intNonNullArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      floatNonNullArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      booleanNonNullArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      idArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      stringArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      intArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      floatArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      booleanArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicInputArg: { src, info, args async throws in
        fatalError("Not implemented")
      },
      basicInputArgWithDefault: { src, info, args async throws in
        fatalError("Not implemented")
      }
    )
    self.BasicInterface = .init(
    )
    self.BasicInterfaceImpl = .init(
    )
    self.BasicScalar = .init(
      serialize: { val in
        fatalError("Not implemented")
      },
      parseValue: { val in
        fatalError("Not implemented")
      },
      parseLiteral: { val in
        fatalError("Not implemented")
      }
    )
  }
}