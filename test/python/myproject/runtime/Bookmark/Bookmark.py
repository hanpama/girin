from ... import runtime_spec
from ... import source_spec
import graphql
import typing


@typing.final
class Bookmark(runtime_spec.Bookmark.Bookmark.Bookmark):
    async def bookmarker(self, obj: BookmarkSource, info: graphql.GraphQLResolveInfo, ) -> source_spec.UserSource:
        raise NotImplementedError()

    async def bookmarkable(self, obj: BookmarkSource, info: graphql.GraphQLResolveInfo, ) -> source_spec.BookmarkableSource:
        raise NotImplementedError()



@typing.final
class BookmarkID(runtime_spec.Bookmark.Bookmark.BookmarkID):
    def serialize(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_value(self, value: typing.Any) -> typing.Any:
        raise NotImplementedError()

    def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:
        raise NotImplementedError()


@typing.final
class BookmarkConnection(runtime_spec.Bookmark.Bookmark.BookmarkConnection):
    pass


@typing.final
class BookmarkEdge(runtime_spec.Bookmark.Bookmark.BookmarkEdge):
    async def node(self, obj: BookmarkEdgeSource, info: graphql.GraphQLResolveInfo, ) -> source_spec.BookmarkSource | None:
        raise NotImplementedError()
