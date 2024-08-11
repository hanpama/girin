from ...config.Basic.Definition import Module


class Impl(Module):

    class BasicObjectConfig(Module.BasicObject):
        def id_non_null_arg(self, src, info, *args):
            raise NotImplementedError

        def string_non_null_arg(self, src, info, *args):
            raise NotImplementedError

        def int_non_null_arg(self, src, info, *args):
            raise NotImplementedError

        def float_non_null_arg(self, src, info, *args):
            raise NotImplementedError

        def boolean_non_null_arg(self, src, info, *args):
            raise NotImplementedError

        def id_arg(self, src, info, *args):
            raise NotImplementedError

        def string_arg(self, src, info, *args):
            raise NotImplementedError

        def int_arg(self, src, info, *args):
            raise NotImplementedError

        def float_arg(self, src, info, *args):
            raise NotImplementedError

        def boolean_arg(self, src, info, *args):
            raise NotImplementedError


impl = Impl(
    BasicObject=Impl.BasicObjectConfig(),
)
