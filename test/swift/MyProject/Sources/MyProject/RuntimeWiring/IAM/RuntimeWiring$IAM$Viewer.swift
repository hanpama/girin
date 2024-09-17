extension Runtime.Wiring.IAM.Viewer {
    init() {
        self.Query = .init(
            viewer: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
    }
}
