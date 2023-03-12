using System.Reflection.Metadata;
using System.Reflection.PortableExecutable;
using Tommy;

public partial class MetadataWriter : IDisposable
{
  private readonly string _outputDirectory;
  private readonly string _assemblyPath;
  private readonly TomlTable _metadata = new TomlTable();

  public MetadataWriter(string assemblyPath, string outputDirectory)
  {
    _assemblyPath = assemblyPath;
    _outputDirectory = outputDirectory;
  }

  private Task WriteFiles()
  {
    using var fs = new FileStream(_assemblyPath, FileMode.Open, FileAccess.Read, FileShare.ReadWrite);
    using var peReader = new PEReader(fs);

    var reader = peReader.GetMetadataReader();

    WriteModules(reader);
    WriteTypeRefs(reader);
    WriteTypeDefs(reader);

    var memory = new MemoryStream();
    var writer = new StreamWriter(memory);
    var fileName = Path.GetFileNameWithoutExtension(_assemblyPath) + ".toml";
    var path = Path.Combine(_outputDirectory, fileName);

    _metadata.WriteTo(writer);
    writer.Flush();

    return File.WriteAllBytesAsync(path, memory.ToArray());
  }

  private void WriteMetadataFile(string metadataName, IEnumerable<TomlTable> metadata) =>
    WriteMetadataFile(metadataName, metadata.ToArray());

  private void WriteMetadataFile(string metadataName, params TomlTable[] metadata)
  {
    var array = new TomlArray { IsTableArray = true };

    foreach (var item in metadata)
    {
      array.Add(item);
    }

    _metadata[metadataName] = array;
  }

  public static Task WriteMetadata(string assemblyPath, string outputDirectory) =>
    new MetadataWriter(assemblyPath, outputDirectory).WriteFiles();

  public void Dispose()
  {
    throw new NotImplementedException();
  }
}