import { graphql, GraphQLSchema, printSchema } from 'graphql';

import { gql, getGraphQLType } from '../src';
import { ObjectType } from '../src/metadata/ObjectType';


interface MemberSource {
  id: number;
  name: string;
  email: string;
  friendId: number;
}

const members: MemberSource[] = [
  { id: 0, name: 'Key', email: 'k@example.com', friendId: 1, },
  { id: 1, name: 'Jonghyun', email: 'j@example.com', friendId: 0 },
]

@ObjectType.define(gql`
  type Member {
    id: Int!
    name: String!
    email: String!
    friend: Member
  }
`)
class Member {
  id: number;
  name: string;
  email: string;

  private friendId: number;

  friend() {
    return Member.find(this.friendId);
  }

  static find(id: number) {
    const member = members.find(m => m.id === id);
    return member && Object.assign(new Member, member);
  }
}

@ObjectType.define(gql`
  type Query {
    getMember(id: Int!): ${Member}
  }
`)
class Query {
  public static getMember(source: null, { id }: { id: number }) {
    return Member.find(id);
  }
}

const schema = new GraphQLSchema({ query: getGraphQLType(Query) });

describe('Schema generation and query of recursive types', async () => {

  it('generates schema as expected', () => {
    expect(printSchema(schema)).toMatchSnapshot();
  });

  it('passes source and args to its resolver', async () => {
    const result = await graphql({ schema, source: `
      query {
        getMember(id: 0) {
          id
          friend {
            id
            friend {
              id
            }
          }
        }
      }
    `});
    expect(result).toEqual({ data: {
      getMember: {
        id: 0,
        friend: {
          id: 1,
          friend: {
            id: 0
          }
        }
      }
    }});
  });

});
