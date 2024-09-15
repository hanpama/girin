extension Runtime.Wiring.Basic.Extension {
    init() {
        self.BasicObject = .init(
            extendedFieldWithArg: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.AnotherInterface = .init(
        )
        self.BasicInterface = .init(
            extendedFieldWithArg: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.BasicInterfaceImpl = .init(
            extendedFieldWithArg: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.AnotherType = .init(
        )
        self.Query = .init(
            extendedHello: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
    }
}
