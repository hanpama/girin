# GENERATED. DO NOT EDIT.
# fmt: off
from . import runtime


class BuilderConfig:
    class Bookmark:
        class Bookmark:
            Bookmark = runtime.Bookmark.Bookmark.Bookmark()
            BookmarkID = runtime.Bookmark.Bookmark.BookmarkID()
            BookmarkConnection = runtime.Bookmark.Bookmark.BookmarkConnection()
            BookmarkEdge = runtime.Bookmark.Bookmark.BookmarkEdge()

        class Bookmarkable:
            BookmarkableID = runtime.Bookmark.Bookmarkable.BookmarkableID()
            BookmarkableBookmarkPayload = runtime.Bookmark.Bookmarkable.BookmarkableBookmarkPayload()
            BookmarkableUnbookmarkPayload = runtime.Bookmark.Bookmarkable.BookmarkableUnbookmarkPayload()
            Mutation = runtime.Bookmark.Bookmarkable.Mutation()

    class Currency:
        pass

    class IAM:
        class User:
            User = runtime.IAM.User.User()
            UserID = runtime.IAM.User.UserID()
            UserConnection = runtime.IAM.User.UserConnection()
            UserEdge = runtime.IAM.User.UserEdge()
            Query = runtime.IAM.User.Query()

        class Viewer:
            Query = runtime.IAM.Viewer.Query()

    class Orders:
        class Order:
            Order = runtime.Orders.Order.Order()
            OrderProduct = runtime.Orders.Order.OrderProduct()
            OrderID = runtime.Orders.Order.OrderID()
            OrderConnection = runtime.Orders.Order.OrderConnection()
            OrderEdge = runtime.Orders.Order.OrderEdge()
            Query = runtime.Orders.Order.Query()
            User = runtime.Orders.Order.User()
            OrderCreateInDraftPayload = runtime.Orders.Order.OrderCreateInDraftPayload()
            Mutation = runtime.Orders.Order.Mutation()

    class Relay:
        PageInfo = runtime.Relay.PageInfo()
        Query = runtime.Relay.Query()
        Cursor = runtime.Relay.Cursor()

    class Root:
        Query = runtime.Root.Query()
        Mutation = runtime.Root.Mutation()
        Timestamp = runtime.Root.Timestamp()
        Decimal = runtime.Root.Decimal()
        TypeID = runtime.Root.TypeID()
