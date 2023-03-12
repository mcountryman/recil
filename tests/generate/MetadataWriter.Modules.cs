
using System.Reflection.Metadata;
using Tommy;

public partial class MetadataWriter
{
  public void WriteModules(MetadataReader reader)
  {
    var module = reader.GetModuleDefinition();

    WriteMetadataFile("module", new TomlTable
    {
      ["name"] = module.Name.GetValue(),
      ["mvid"] = module.Mvid.GetValue(),
      ["enc_id"] = module.GenerationId.GetValue(),
      ["enc_base_id"] = module.BaseGenerationId.GetValue(),
    });
  }
}