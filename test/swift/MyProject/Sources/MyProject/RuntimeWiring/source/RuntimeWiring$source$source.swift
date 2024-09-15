extension Runtime.Wiring.source.source {
    init() {
        self.Source = .init(
            a: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
    }
}
