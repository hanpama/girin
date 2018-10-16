# Girin: GraphQL server framework

Build better GraphQL schema with less code
* defining schema, not generating it
* modularizing your GraphQL server

[![npm version](https://badge.fury.io/js/girin.svg)](https://badge.fury.io/js/girin)
[![Build Status](https://travis-ci.org/hanpama/girin.svg?branch=master)](https://travis-ci.org/hanpama/girin)
[![codecov](https://codecov.io/gh/hanpama/girin/branch/master/graph/badge.svg)](https://codecov.io/gh/hanpama/girin)
[![lerna](https://img.shields.io/badge/maintained%20with-lerna-cc00ff.svg)](https://lernajs.io/)

> This project is in development. not stable!

## Features

* Use GraphQL Schema Definition Language to expose classes to API
* Lightweight MongoDB model and relay compliant pagination implementation
* Password authentication based on JWT
* Built in GraphQL MongoDB subscriptions (WIP)
* Cache control and file upload powered by apollo-server


## How it looks like

```ts
import { girin } from '@girin/framework';
import { defineType, gql } from '@girin/typelink';

@defineType(gql`
  type Query {
    hello: String!
  }
`)
class Query {
  static hello() {
    return 'World!'
  }
}

girin({ SCHEMA: { Query } }).run();
```
<!--
## Get started

```sh
npm install girin graphql
```

`@girin/framework` package is for bootstrapping server with your schema.

`@girin/typelink` package provides decorator and `gql` template tag,
a SDL parser for linking class to GraphQL type. -->
