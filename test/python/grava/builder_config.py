# GENERATED. DO NOT EDIT.
# fmt: off
from . import impl
from .spec import Spec



class BuilderConfig:
    class Basic:
        class Definition:
            BasicObject: Spec.Basic.Definition.BasicObjectSpec = impl.Basic.Definition.BasicObjectImpl()
            BasicInterface: Spec.Basic.Definition.BasicInterfaceSpec = impl.Basic.Definition.BasicInterfaceImpl()
            BasicInterfaceImpl: Spec.Basic.Definition.BasicInterfaceImplSpec = impl.Basic.Definition.BasicInterfaceImplImpl()
            BasicScalar: Spec.Basic.Definition.BasicScalarSpec = impl.Basic.Definition.BasicScalarImpl()
        class Extension:
            BasicObject: Spec.Basic.Extension.BasicObjectSpec = impl.Basic.Extension.BasicObjectImpl()
            AnotherInterface: Spec.Basic.Extension.AnotherInterfaceSpec = impl.Basic.Extension.AnotherInterfaceImpl()
            BasicInterface: Spec.Basic.Extension.BasicInterfaceSpec = impl.Basic.Extension.BasicInterfaceImpl()
            BasicInterfaceImpl: Spec.Basic.Extension.BasicInterfaceImplSpec = impl.Basic.Extension.BasicInterfaceImplImpl()
            AnotherType: Spec.Basic.Extension.AnotherTypeSpec = impl.Basic.Extension.AnotherTypeImpl()
            Query: Spec.Basic.Extension.QuerySpec = impl.Basic.Extension.QueryImpl()
    class Deprecation:
        class Definition:
            DeprecatedFieldObject: Spec.Deprecation.Definition.DeprecatedFieldObjectSpec = impl.Deprecation.Definition.DeprecatedFieldObjectImpl()
    class Module:
        class Module:
            class module:
                ModuleB: Spec.Module.Module.module.ModuleBSpec = impl.Module.Module.module.ModuleBImpl()
        class module:
            ModuleA: Spec.Module.module.ModuleASpec = impl.Module.module.ModuleAImpl()
    class Nested1:
        class Nested2:
            class nested2:
                Nested2: Spec.Nested1.Nested2.nested2.Nested2Spec = impl.Nested1.Nested2.nested2.Nested2Impl()
        class nested1:
            Nested1: Spec.Nested1.nested1.Nested1Spec = impl.Nested1.nested1.Nested1Impl()
    class Resolve:
        class Schema:
            GrandParent: Spec.Resolve.Schema.GrandParentSpec = impl.Resolve.Schema.GrandParentImpl()
            Parent: Spec.Resolve.Schema.ParentSpec = impl.Resolve.Schema.ParentImpl()
            Child: Spec.Resolve.Schema.ChildSpec = impl.Resolve.Schema.ChildImpl()
    class root:
        Query: Spec.root.QuerySpec = impl.root.QueryImpl()
        Mutation: Spec.root.MutationSpec = impl.root.MutationImpl()
    class source:
        class source:
            Source: Spec.source.source.SourceSpec = impl.source.source.SourceImpl()
