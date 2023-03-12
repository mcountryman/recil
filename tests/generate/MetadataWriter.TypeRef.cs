
using System.Reflection.Metadata;
using Tommy;

public partial class MetadataWriter
{
  public void WriteTypeRefs(MetadataReader reader) =>
    WriteMetadataFile(
      "type_ref",
      reader.TypeReferences
        .Select(tr => reader.GetTypeReference(tr))
        .Select(typeRef => new TomlTable
        {
          ["name"] = typeRef.Name.GetValue(),
          ["namespace"] = typeRef.Namespace.GetValue(),
          ["resolution_scope"] = typeRef.ResolutionScope.GetTagAndRow(),
        })
      );
}