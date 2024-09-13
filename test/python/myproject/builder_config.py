# GENERATED. DO NOT EDIT.
# fmt: off
from . import runtime


class BuilderConfig:
    class Basic:
        class Definition:
            BasicObject = runtime.Basic.Definition.BasicObject()
            BasicInterface = runtime.Basic.Definition.BasicInterface()
            BasicInterfaceImpl = runtime.Basic.Definition.BasicInterfaceImpl()
            BasicScalar = runtime.Basic.Definition.BasicScalar()

        class Extension:
            BasicObject = runtime.Basic.Extension.BasicObject()
            AnotherInterface = runtime.Basic.Extension.AnotherInterface()
            BasicInterface = runtime.Basic.Extension.BasicInterface()
            BasicInterfaceImpl = runtime.Basic.Extension.BasicInterfaceImpl()
            AnotherType = runtime.Basic.Extension.AnotherType()
            Query = runtime.Basic.Extension.Query()

    class Deprecation:
        class Definition:
            DeprecatedFieldObject = runtime.Deprecation.Definition.DeprecatedFieldObject()

    class Module:
        class Module:
            class module:
                ModuleB = runtime.Module.Module.module.ModuleB()

        class module:
            ModuleA = runtime.Module.module.ModuleA()

    class Nested1:
        class Nested2:
            class nested2:
                Nested2 = runtime.Nested1.Nested2.nested2.Nested2()

        class nested1:
            Nested1 = runtime.Nested1.nested1.Nested1()

    class Resolve:
        class Schema:
            GrandParent = runtime.Resolve.Schema.GrandParent()
            Parent = runtime.Resolve.Schema.Parent()
            Child = runtime.Resolve.Schema.Child()

    class graphql_:
        GraphQLObject = runtime.graphql_.GraphQLObject()
        graphql_ = runtime.graphql_.graphql_()
        typing_ = runtime.graphql_.typing_()

    class root:
        Query = runtime.root.Query()
        Mutation = runtime.root.Mutation()
        DateTime = runtime.root.DateTime()

    class source:
        class source:
            Source = runtime.source.source.Source()
