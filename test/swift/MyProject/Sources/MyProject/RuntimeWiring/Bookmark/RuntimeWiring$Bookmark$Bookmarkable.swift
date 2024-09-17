extension Runtime.Wiring.Bookmark.Bookmarkable {
    init() {
        self.BookmarkableBookmarkPayload = .init(
            bookmarkableBookmarked: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.BookmarkableUnbookmarkPayload = .init(
            bookmarkableUnbookmarked: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
        self.Mutation = .init(
            bookmarkBookmarkable: { source, args, context, info in
                fatalError("Not implemented")
            },
            unbookmarkBookmarkable: { source, args, context, info in
                fatalError("Not implemented")
            }
        )
    }
}
