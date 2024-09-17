extension Runtime.Wiring.IAM.User {
    init() {
        self.User = .init(
            id: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.UserConnection = .init(
        )
        self.UserEdge = .init(
            node: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.Query = .init(
            user: { source, args, context, info in
                fatalError("Not implemented")
            },
            userConnection: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
    }
}
