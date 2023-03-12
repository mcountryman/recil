
using System.Reflection.Metadata;
using Tommy;

public partial class MetadataWriter
{
  public void WriteTypeDefs(MetadataReader reader) =>
    WriteMetadataFile(
      "type_def",
      reader.TypeDefinitions
        .Select(td => reader.GetTypeDefinition(td))
        .Select(typeDef => new TomlTable
        {
          ["flags"] = (int)typeDef.Attributes,
          ["name"] = typeDef.Name.GetValue(),
          ["namespace"] = typeDef.Namespace.GetValue(),
          ["extends"] = typeDef.BaseType.GetTagAndRow(),
          // ["field_list"] = typeDef.GetFields(),
          // ["method_list"] = typeDef.GetMethods(),
        })
      );
}