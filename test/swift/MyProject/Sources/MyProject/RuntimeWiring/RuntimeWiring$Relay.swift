extension Runtime.Wiring.Relay {
    init() {
        self.PageInfo = .init(
        )
        self.Query = .init(
            node: { source, args, context, info in
                fatalError("Not implemented")
            },
            nodes: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
    }
}
