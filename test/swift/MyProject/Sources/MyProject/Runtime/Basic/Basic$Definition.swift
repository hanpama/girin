extension RuntimeSpec.Basic.Definition {
    init() {
        self.BasicObject = .init(
            idNonNullArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            stringNonNullArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            intNonNullArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            floatNonNullArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            booleanNonNullArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            idArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            stringArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            intArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            floatArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            booleanArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicInputArg: { source, args, context, info async throws in
                fatalError("Not implemented")
            },
            basicInputArgWithDefault: { source, args, context, info async throws in
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