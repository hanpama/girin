import { GraphQLFieldConfig, GraphQLFieldConfigMap, GraphQLInterfaceType, GraphQLTypeResolver, GraphQLOutputType } from 'graphql';

import { Definition, DefinitionConfig, DefinitionKind } from '../metadata';
import { Field } from '../reference';
import { TypeResolvingContext } from '../type-expression';


export interface InterfaceTypeConfig extends DefinitionConfig {
  resolveType?: GraphQLTypeResolver<any, any>;
  description?: string;
}

/**
 * Metadata type for InterfaceType
 */
export class InterfaceType<T extends InterfaceTypeConfig = InterfaceTypeConfig> extends Definition<T> {
  public get kind(): DefinitionKind { return 'output'; }

  public buildFieldConfig(context: TypeResolvingContext, field: Field): GraphQLFieldConfig<any, any> {
    const { description, deprecationReason } = field;

    return {
      type: field.resolveType(context) as GraphQLOutputType,
      args: field.buildArgumentMap(context),
      resolve: field.buildResolver(context),
      description,
      deprecationReason,
    };
  }

  public buildFieldConfigMap(context: TypeResolvingContext): GraphQLFieldConfigMap<any, any> {
    const fields = [
      ...context.storage.findExtendReferences(Field, this.definitionName),
      ...context.storage.findDirectReferences(Field, this.definitionClass),
    ];
    return (
      fields.reduce((results, field) => {
        const name = field.fieldName;

        results[name] = this.buildFieldConfig(context, field);
        return results;
      }, {} as GraphQLFieldConfigMap<any, any>)
    );
  }

  public buildTypeInstance(context: TypeResolvingContext): GraphQLInterfaceType {
    const name = this.typeName(context);
    const description = this.description(context);
    const fields = this.buildFieldConfigMap.bind(this, context);

    return new GraphQLInterfaceType({ name, fields, description });
  }
}