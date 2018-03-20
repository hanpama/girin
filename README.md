# Girin: GraphQL framework

Girin is a GraphQL framework written in TypeScript.

* Seamless integration between GraphQL SDL and TypeScript classes
* Modularization of GraphQL type definitions

[![Build Status](https://travis-ci.org/hanpama/girin.svg?branch=master)](https://travis-ci.org/hanpama/girin)


```typescript
@Definition(gql`
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
    return members.find(m => m.id === this.friendId);
  }
}
```

## Installation

```sh
npm install girin graphql
```

## Examples

### Todolist App

Working example of simple todolist

* [Demo](https://todolist.giringraphql.com/)
* [Source Code](https://github.com/hanpama/girin/tree/master/packages/example-todolist-app)


### Starwars Schema

* [Starwars Schema](https://github.com/hanpama/girin/tree/master/packages/girin/test/starwars/starWarsSchema.ts)
* [Starwars Schema Relay](https://github.com/hanpama/girin/tree/master/packages/girin-relay/test/starWarsSchema.ts)
