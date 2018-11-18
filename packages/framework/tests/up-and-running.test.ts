import { defineType, gql } from '@girin/typelink';

import { TestHttpServer, prepareTestEnv, destroyTestEnv } from '../src';


@defineType(gql`
type Query {
  hello: String!
}
`)
export class Query {
  static hello() { return 'World👋'; }
}

describe('server', () => {

  beforeAll(() => { return prepareTestEnv({ Query }).run(); });
  afterAll(destroyTestEnv);

  it('should go up and running', async () => {
    const client = TestHttpServer.object().getClient();
    expect(await client.sendQuery({ query: `{ hello }` })).toEqual({ data: { hello: 'World👋' }});
  });
});
