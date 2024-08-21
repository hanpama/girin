import unittest
import graphql

from myproject import builder, builder_config


class TestGenerated(unittest.IsolatedAsyncioTestCase):
    async def test_foo(self):
        schema = builder.build_schema(builder_config.BuilderConfig())
        introspection_query = graphql.get_introspection_query()
        result = graphql.execute_sync(schema, graphql.parse(introspection_query))
        print(result)
