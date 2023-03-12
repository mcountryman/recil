
using System.Reflection;
using System.Reflection.Metadata;
using Tommy;

public static class MetadataExtensions
{
  public static uint GetValue(this StringHandle handle)
  {
    return (uint)Priv.GetField(handle, "_value")!;
  }

  public static uint GetValue(this BlobHandle handle)
  {
    return (uint)Priv.GetField(handle, "_value")!;
  }

  public static int GetValue(this GuidHandle handle)
  {
    return (int)Priv.GetField(handle, "_index")!;
  }

  public static TomlTable GetTagAndRow(this EntityHandle handle)
  {
    var table = new TomlTable();
    table["kind"] = Enum.GetName(typeof(HandleKind), handle.Kind);
    table["row_id"] = (int)Priv.GetProperty(handle, "RowId")!;
    return table;
  }

  private static FieldInfo GetPrivField(Type type, string name) =>
    type.GetField(name, BindingFlags.NonPublic | BindingFlags.Instance) ?? throw new Exception();
  private static PropertyInfo GetPrivProperty(Type type, string name) =>
    type.GetProperty(name, BindingFlags.NonPublic | BindingFlags.Instance) ?? throw new Exception();
}

static class Priv
{
  static Dictionary<Type, Dictionary<string, FieldInfo>> _fields = new Dictionary<Type, Dictionary<string, FieldInfo>>();
  static Dictionary<Type, Dictionary<string, PropertyInfo>> _props = new Dictionary<Type, Dictionary<string, PropertyInfo>>();

  public static object GetField<T>(T type, string name)
  {
    if (!_fields.TryGetValue(typeof(T), out var fields))
    {
      fields = new Dictionary<string, FieldInfo>();
      _fields[typeof(T)] = fields;
    }

    if (!fields.TryGetValue(name, out var field))
    {
      field = typeof(T).GetField(name, BindingFlags.NonPublic | BindingFlags.Instance) ?? throw new Exception();
      fields[name] = field;
    }

    return field.GetValue(type)!;
  }

  public static object GetProperty<T>(T type, string name)
  {
    if (!_props.TryGetValue(typeof(T), out var props))
    {
      props = new Dictionary<string, PropertyInfo>();
      _props[typeof(T)] = props;
    }

    if (!props.TryGetValue(name, out var prop))
    {
      prop = typeof(T).GetProperty(name, BindingFlags.NonPublic | BindingFlags.Instance) ?? throw new Exception();
      props[name] = prop;
    }

    return prop.GetValue(type)!;
  }
}