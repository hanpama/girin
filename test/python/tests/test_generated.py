import unittest
import graphql
import typing

from myproject import builder, builder_config


class TestGenerated(unittest.IsolatedAsyncioTestCase):
    async def test_foo(self):
        schema = builder.build_schema(builder_config.BuilderConfig())
        res = graphql.graphql(
            schema=schema,
            source="""{ extendedHello(name: "World") }""",
        )
        if isinstance(res, typing.Awaitable):
            res = await res

        assert res.errors is None
        assert res.data
        assert res.data["extendedHello"] == "Hello, World!"
