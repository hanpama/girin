# GENERATED. DO NOT EDIT.
# fmt: off
from . import runtime


class BuilderConfig:
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
        Node = runtime.Relay.Node()
        PageInfo = runtime.Relay.PageInfo()
        Query = runtime.Relay.Query()
        Cursor = runtime.Relay.Cursor()

    class Root:
        Query = runtime.Root.Query()
        Mutation = runtime.Root.Mutation()
        Timestamp = runtime.Root.Timestamp()
        Decimal = runtime.Root.Decimal()
