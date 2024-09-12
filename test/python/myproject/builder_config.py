# GENERATED. DO NOT EDIT.
# fmt: off
from . import impl


class BuilderConfig:
    class Basic:
        class Definition:
            BasicObject = impl.Basic.Definition.BasicObject()
            BasicInterface = impl.Basic.Definition.BasicInterface()
            BasicInterfaceImpl = impl.Basic.Definition.BasicInterfaceImpl()
            BasicScalar = impl.Basic.Definition.BasicScalar()
        class Extension:
            BasicObject = impl.Basic.Extension.BasicObject()
            AnotherInterface = impl.Basic.Extension.AnotherInterface()
            BasicInterface = impl.Basic.Extension.BasicInterface()
            BasicInterfaceImpl = impl.Basic.Extension.BasicInterfaceImpl()
            AnotherType = impl.Basic.Extension.AnotherType()
            Query = impl.Basic.Extension.Query()
    class Deprecation:
        class Definition:
            DeprecatedFieldObject = impl.Deprecation.Definition.DeprecatedFieldObject()
    class Module:
        class Module:
            class module:
                ModuleB = impl.Module.Module.module.ModuleB()
        class module:
            ModuleA = impl.Module.module.ModuleA()
    class Nested1:
        class Nested2:
            class nested2:
                Nested2 = impl.Nested1.Nested2.nested2.Nested2()
        class nested1:
            Nested1 = impl.Nested1.nested1.Nested1()
    class Resolve:
        class Schema:
            GrandParent = impl.Resolve.Schema.GrandParent()
            Parent = impl.Resolve.Schema.Parent()
            Child = impl.Resolve.Schema.Child()
    class graphql_:
        GraphQLObject = impl.graphql_.GraphQLObject()
        graphql_ = impl.graphql_.graphql_()
        typing_ = impl.graphql_.typing_()
    class root:
        Query = impl.root.Query()
        Mutation = impl.root.Mutation()
    class source:
        class source:
            Source = impl.source.source.Source()
