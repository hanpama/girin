import { GraphQLInputObjectType, GraphQLInputFieldConfigMap } from "graphql";
import { InputFieldMetadata } from "./InputFieldMetadata";
import { DefinitionMetadata, DefinitionMetadataConfig } from "../base/DefinitionMetadata";
import { memoizedGetter as builder } from "../utilities/memoize";
import { Instantiator } from "../types";


export interface InputObjectTypeMetadataConfig extends DefinitionMetadataConfig {}

/**
 * Metadata type for InputObjectType
 */
export class InputObjectTypeMetadata<T extends InputObjectTypeMetadataConfig = InputObjectTypeMetadataConfig> extends DefinitionMetadata<T> {

  protected findInputFieldMetadata(): InputFieldMetadata[] {
    return this.storage.findGenericMetadata(InputFieldMetadata, this.definitionClass);
  }

  /**
   * Build GraphQLInterfaceType instance from metadata.
   */
  @builder
  public get typeInstance() {
    const name = this.typeName;
    const fields = () => this.fields;
    const description = this.description;
    return new GraphQLInputObjectType({ name, fields, description });
  }

  protected get fields(): GraphQLInputFieldConfigMap {
    const inputFieldMetadata = this.findInputFieldMetadata();
    return inputFieldMetadata.reduce((results, metadata) => {
      results[metadata.fieldName] = metadata.inputFieldConfig;
      return results;
    }, {} as GraphQLInputFieldConfigMap);
  }

  /**
   * Get the instantiator function from definition class or return default
   */
  public get instantiate(): Instantiator {
    const { definitionClass } = this;
    const inputFieldMetadata = this.findInputFieldMetadata();

    if (definitionClass.instantiate) {
      return definitionClass.instantiate.bind(definitionClass);
    }
    return function (argsObject: any, context: any, info: any) {
      const instance = Object.create(definitionClass.prototype);
      return inputFieldMetadata.reduce((results, metadata) => {
        results[metadata.fieldName] = metadata.instantiate(argsObject[metadata.fieldName]);
        return results;
      }, instance);
    }
  }
}
