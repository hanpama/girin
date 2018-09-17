import { GraphQLList, GraphQLScalarType } from 'graphql';

import { getGlobalMetadataStorage, List, TypeExpression, InputType, gql, ObjectType } from '../src';


describe('createFromTypeString', () => {
  const storage = getGlobalMetadataStorage();

  it('is created as expected', () => {
    const boolean = new TypeExpression('Boolean');
    const type: any = boolean.getTypeInstance(storage);
    expect(type.name).toBe('Boolean');
  });

  it('resolves nested expression', () => {
    const complexExpression = new TypeExpression(() => List.of(new TypeExpression('String')));
    const resolved = complexExpression.resolveLazy() as List;

    expect(resolved).toBeInstanceOf(List);
    expect(resolved.innerType).toBeInstanceOf(TypeExpression);
    expect(resolved.innerType.typeArg).toBe('String');

    const built = resolved.getTypeInstance(storage) as GraphQLList<GraphQLScalarType>;
    expect(built).toBeInstanceOf(GraphQLList);
  });

  it('resolves explicit output/input type expression', () => {
    @ObjectType.define(gql`
      type Person {
        name: String!
        email: String
      }
    `)
    @InputType.define(gql`
      input PersonInput {
        name: String!
        email: String
      }
    `)
    class Person {
      name: string;
      email?: string;
    }

    const personOutputExp = new TypeExpression(Person, 'output');
    const personInputExp = new TypeExpression(Person, 'input');
    expect(personOutputExp.getDefinitionEntry(storage)).not.toBe(personInputExp.getDefinitionEntry(storage));
  })
});