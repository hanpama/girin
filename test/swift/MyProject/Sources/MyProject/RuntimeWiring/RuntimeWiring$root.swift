extension Runtime.Wiring.Root {
    init() {
        self.Query = .init(
            version: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.Mutation = .init(
        )
    }
}
