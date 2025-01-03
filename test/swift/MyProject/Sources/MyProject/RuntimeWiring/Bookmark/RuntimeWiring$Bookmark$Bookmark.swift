extension Runtime.Wiring.Bookmark.Bookmark {
    init() {
        self.Bookmark = .init(
            bookmarker: { source, args, context, info in
                fatalError("Not implemented")
            },
            bookmarkable: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.BookmarkConnection = .init(
            edges: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.BookmarkEdge = .init(
            node: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
    }
}
