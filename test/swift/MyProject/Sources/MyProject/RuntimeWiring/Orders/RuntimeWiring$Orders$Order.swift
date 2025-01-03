extension Runtime.Wiring.Orders.Order {
    init() {
        self.Order = .init(
            id: { source, args, context, info in
                fatalError("Not implemented")
            },
            orderer: { source, args, context, info in
                fatalError("Not implemented")
            },
            viewerHasBookmarked: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.OrderProduct = .init(
        )
        self.OrderConnection = .init(
            edges: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.OrderEdge = .init(
            node: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.Query = .init(
            order: { source, args, context, info in
                fatalError("Not implemented")
            },
            orderConnection: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.User = .init(
            orders: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.OrderCreateInDraftPayload = .init(
        )
        self.Mutation = .init(
            orderCreateInDraft: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
    }
}
