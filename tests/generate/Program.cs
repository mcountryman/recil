using System.Runtime.CompilerServices;

await MetadataWriter.WriteMetadata(R("../inputs/Newtonsoft.Json.dll"), R("../inputs"));

string R(string path, [CallerFilePath] string filePath = "") =>
Path.Combine(Path.GetDirectoryName(filePath)!, path);