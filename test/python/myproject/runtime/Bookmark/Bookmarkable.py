from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class BookmarkableID(runtime_spec.Bookmark.Bookmarkable.BookmarkableID):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:
        raise NotImplementedError()


@typing.final
class BookmarkableBookmarkPayload(runtime_spec.Bookmark.Bookmarkable.BookmarkableBookmarkPayload):
    async def bookmarkable_bookmarked(self, obj: source_spec.BookmarkableBookmarkPayloadSource, info: graphql.GraphQLResolveInfo) -> source_spec.BookmarkableSource:
        raise NotImplementedError()



@typing.final
class BookmarkableUnbookmarkPayload(runtime_spec.Bookmark.Bookmarkable.BookmarkableUnbookmarkPayload):
    async def bookmarkable_unbookmarked(self, obj: source_spec.BookmarkableUnbookmarkPayloadSource, info: graphql.GraphQLResolveInfo) -> source_spec.BookmarkableSource:
        raise NotImplementedError()



@typing.final
class Mutation(runtime_spec.Bookmark.Bookmarkable.Mutation):
    async def bookmark_bookmarkable(self, obj: source_spec.MutationSource, info: graphql.GraphQLResolveInfo, input: source_spec.BookmarkableBookmarkInputSource) -> source_spec.BookmarkableBookmarkPayloadSource | None:
        raise NotImplementedError()

    async def unbookmark_bookmarkable(self, obj: source_spec.MutationSource, info: graphql.GraphQLResolveInfo, input: source_spec.BookmarkableUnbookmarkInputSource) -> source_spec.BookmarkableUnbookmarkPayloadSource | None:
        raise NotImplementedError()
