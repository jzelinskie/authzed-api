// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: authzed/api/v1alpha1/schema.proto
// </auto-generated>
#pragma warning disable 1591, 0612, 3021, 8981
#region Designer generated code

using pb = global::Google.Protobuf;
using pbc = global::Google.Protobuf.Collections;
using pbr = global::Google.Protobuf.Reflection;
using scg = global::System.Collections.Generic;
namespace Authzed.Api.V1Alpha1 {

  /// <summary>Holder for reflection information generated from authzed/api/v1alpha1/schema.proto</summary>
  public static partial class SchemaReflection {

    #region Descriptor
    /// <summary>File descriptor for authzed/api/v1alpha1/schema.proto</summary>
    public static pbr::FileDescriptor Descriptor {
      get { return descriptor; }
    }
    private static pbr::FileDescriptor descriptor;

    static SchemaReflection() {
      byte[] descriptorData = global::System.Convert.FromBase64String(
          string.Concat(
            "CiFhdXRoemVkL2FwaS92MWFscGhhMS9zY2hlbWEucHJvdG8SFGF1dGh6ZWQu",
            "YXBpLnYxYWxwaGExGhd2YWxpZGF0ZS92YWxpZGF0ZS5wcm90byKdAQoRUmVh",
            "ZFNjaGVtYVJlcXVlc3QShwEKGG9iamVjdF9kZWZpbml0aW9uc19uYW1lcxgB",
            "IAMoCUJN+kJKkgFHIkVyQyiAATI+XihbYS16XVthLXowLTlfXXsxLDYyfVth",
            "LXowLTldLykqW2Etel1bYS16MC05X117MSw2Mn1bYS16MC05XSRSFm9iamVj",
            "dERlZmluaXRpb25zTmFtZXMihwEKElJlYWRTY2hlbWFSZXNwb25zZRItChJv",
            "YmplY3RfZGVmaW5pdGlvbnMYASADKAlSEW9iamVjdERlZmluaXRpb25zEkIK",
            "HWNvbXB1dGVkX2RlZmluaXRpb25zX3JldmlzaW9uGAIgASgJUhtjb21wdXRl",
            "ZERlZmluaXRpb25zUmV2aXNpb24ilAEKEldyaXRlU2NoZW1hUmVxdWVzdBIh",
            "CgZzY2hlbWEYASABKAlCCfpCBnIEKICAEFIGc2NoZW1hElsKKm9wdGlvbmFs",
            "X2RlZmluaXRpb25zX3JldmlzaW9uX3ByZWNvbmRpdGlvbhgCIAEoCVInb3B0",
            "aW9uYWxEZWZpbml0aW9uc1JldmlzaW9uUHJlY29uZGl0aW9uIpMBChNXcml0",
            "ZVNjaGVtYVJlc3BvbnNlEjgKGG9iamVjdF9kZWZpbml0aW9uc19uYW1lcxgB",
            "IAMoCVIWb2JqZWN0RGVmaW5pdGlvbnNOYW1lcxJCCh1jb21wdXRlZF9kZWZp",
            "bml0aW9uc19yZXZpc2lvbhgCIAEoCVIbY29tcHV0ZWREZWZpbml0aW9uc1Jl",
            "dmlzaW9uMtgBCg1TY2hlbWFTZXJ2aWNlEmEKClJlYWRTY2hlbWESJy5hdXRo",
            "emVkLmFwaS52MWFscGhhMS5SZWFkU2NoZW1hUmVxdWVzdBooLmF1dGh6ZWQu",
            "YXBpLnYxYWxwaGExLlJlYWRTY2hlbWFSZXNwb25zZSIAEmQKC1dyaXRlU2No",
            "ZW1hEiguYXV0aHplZC5hcGkudjFhbHBoYTEuV3JpdGVTY2hlbWFSZXF1ZXN0",
            "GikuYXV0aHplZC5hcGkudjFhbHBoYTEuV3JpdGVTY2hlbWFSZXNwb25zZSIA",
            "QlQKGGNvbS5hdXRoemVkLmFwaS52MWFscGhhMVo4Z2l0aHViLmNvbS9hdXRo",
            "emVkL2F1dGh6ZWQtZ28vcHJvdG8vYXV0aHplZC9hcGkvdjFhbHBoYTFiBnBy",
            "b3RvMw=="));
      descriptor = pbr::FileDescriptor.FromGeneratedCode(descriptorData,
          new pbr::FileDescriptor[] { global::Validate.ValidateReflection.Descriptor, },
          new pbr::GeneratedClrTypeInfo(null, null, new pbr::GeneratedClrTypeInfo[] {
            new pbr::GeneratedClrTypeInfo(typeof(global::Authzed.Api.V1Alpha1.ReadSchemaRequest), global::Authzed.Api.V1Alpha1.ReadSchemaRequest.Parser, new[]{ "ObjectDefinitionsNames" }, null, null, null, null),
            new pbr::GeneratedClrTypeInfo(typeof(global::Authzed.Api.V1Alpha1.ReadSchemaResponse), global::Authzed.Api.V1Alpha1.ReadSchemaResponse.Parser, new[]{ "ObjectDefinitions", "ComputedDefinitionsRevision" }, null, null, null, null),
            new pbr::GeneratedClrTypeInfo(typeof(global::Authzed.Api.V1Alpha1.WriteSchemaRequest), global::Authzed.Api.V1Alpha1.WriteSchemaRequest.Parser, new[]{ "Schema", "OptionalDefinitionsRevisionPrecondition" }, null, null, null, null),
            new pbr::GeneratedClrTypeInfo(typeof(global::Authzed.Api.V1Alpha1.WriteSchemaResponse), global::Authzed.Api.V1Alpha1.WriteSchemaResponse.Parser, new[]{ "ObjectDefinitionsNames", "ComputedDefinitionsRevision" }, null, null, null, null)
          }));
    }
    #endregion

  }
  #region Messages
  /// <summary>
  /// ReadSchemaRequest is the required data to read Object Definitions from
  /// a Schema.
  /// </summary>
  [global::System.Diagnostics.DebuggerDisplayAttribute("{ToString(),nq}")]
  public sealed partial class ReadSchemaRequest : pb::IMessage<ReadSchemaRequest>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<ReadSchemaRequest> _parser = new pb::MessageParser<ReadSchemaRequest>(() => new ReadSchemaRequest());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<ReadSchemaRequest> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::Authzed.Api.V1Alpha1.SchemaReflection.Descriptor.MessageTypes[0]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ReadSchemaRequest() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ReadSchemaRequest(ReadSchemaRequest other) : this() {
      objectDefinitionsNames_ = other.objectDefinitionsNames_.Clone();
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ReadSchemaRequest Clone() {
      return new ReadSchemaRequest(this);
    }

    /// <summary>Field number for the "object_definitions_names" field.</summary>
    public const int ObjectDefinitionsNamesFieldNumber = 1;
    private static readonly pb::FieldCodec<string> _repeated_objectDefinitionsNames_codec
        = pb::FieldCodec.ForString(10);
    private readonly pbc::RepeatedField<string> objectDefinitionsNames_ = new pbc::RepeatedField<string>();
    /// <summary>
    /// The list of names of the Object Definitions that are being requested.
    ///
    /// These names must be fully qualified with their namespace (e.g.
    /// myblog/post).
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public pbc::RepeatedField<string> ObjectDefinitionsNames {
      get { return objectDefinitionsNames_; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as ReadSchemaRequest);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(ReadSchemaRequest other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if(!objectDefinitionsNames_.Equals(other.objectDefinitionsNames_)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      hash ^= objectDefinitionsNames_.GetHashCode();
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      objectDefinitionsNames_.WriteTo(output, _repeated_objectDefinitionsNames_codec);
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      objectDefinitionsNames_.WriteTo(ref output, _repeated_objectDefinitionsNames_codec);
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      size += objectDefinitionsNames_.CalculateSize(_repeated_objectDefinitionsNames_codec);
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(ReadSchemaRequest other) {
      if (other == null) {
        return;
      }
      objectDefinitionsNames_.Add(other.objectDefinitionsNames_);
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
      if ((tag & 7) == 4) {
        // Abort on any end group tag.
        return;
      }
      switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 10: {
            objectDefinitionsNames_.AddEntriesFrom(input, _repeated_objectDefinitionsNames_codec);
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
      if ((tag & 7) == 4) {
        // Abort on any end group tag.
        return;
      }
      switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10: {
            objectDefinitionsNames_.AddEntriesFrom(ref input, _repeated_objectDefinitionsNames_codec);
            break;
          }
        }
      }
    }
    #endif

  }

  /// <summary>
  /// ReadSchemaResponse is the resulting data after having read the Object
  /// Definitions from a Schema.
  /// </summary>
  [global::System.Diagnostics.DebuggerDisplayAttribute("{ToString(),nq}")]
  public sealed partial class ReadSchemaResponse : pb::IMessage<ReadSchemaResponse>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<ReadSchemaResponse> _parser = new pb::MessageParser<ReadSchemaResponse>(() => new ReadSchemaResponse());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<ReadSchemaResponse> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::Authzed.Api.V1Alpha1.SchemaReflection.Descriptor.MessageTypes[1]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ReadSchemaResponse() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ReadSchemaResponse(ReadSchemaResponse other) : this() {
      objectDefinitions_ = other.objectDefinitions_.Clone();
      computedDefinitionsRevision_ = other.computedDefinitionsRevision_;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public ReadSchemaResponse Clone() {
      return new ReadSchemaResponse(this);
    }

    /// <summary>Field number for the "object_definitions" field.</summary>
    public const int ObjectDefinitionsFieldNumber = 1;
    private static readonly pb::FieldCodec<string> _repeated_objectDefinitions_codec
        = pb::FieldCodec.ForString(10);
    private readonly pbc::RepeatedField<string> objectDefinitions_ = new pbc::RepeatedField<string>();
    /// <summary>
    /// The Object Definitions that were requested.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public pbc::RepeatedField<string> ObjectDefinitions {
      get { return objectDefinitions_; }
    }

    /// <summary>Field number for the "computed_definitions_revision" field.</summary>
    public const int ComputedDefinitionsRevisionFieldNumber = 2;
    private string computedDefinitionsRevision_ = "";
    /// <summary>
    /// The computed revision of the returned object definitions.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string ComputedDefinitionsRevision {
      get { return computedDefinitionsRevision_; }
      set {
        computedDefinitionsRevision_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as ReadSchemaResponse);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(ReadSchemaResponse other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if(!objectDefinitions_.Equals(other.objectDefinitions_)) return false;
      if (ComputedDefinitionsRevision != other.ComputedDefinitionsRevision) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      hash ^= objectDefinitions_.GetHashCode();
      if (ComputedDefinitionsRevision.Length != 0) hash ^= ComputedDefinitionsRevision.GetHashCode();
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      objectDefinitions_.WriteTo(output, _repeated_objectDefinitions_codec);
      if (ComputedDefinitionsRevision.Length != 0) {
        output.WriteRawTag(18);
        output.WriteString(ComputedDefinitionsRevision);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      objectDefinitions_.WriteTo(ref output, _repeated_objectDefinitions_codec);
      if (ComputedDefinitionsRevision.Length != 0) {
        output.WriteRawTag(18);
        output.WriteString(ComputedDefinitionsRevision);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      size += objectDefinitions_.CalculateSize(_repeated_objectDefinitions_codec);
      if (ComputedDefinitionsRevision.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(ComputedDefinitionsRevision);
      }
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(ReadSchemaResponse other) {
      if (other == null) {
        return;
      }
      objectDefinitions_.Add(other.objectDefinitions_);
      if (other.ComputedDefinitionsRevision.Length != 0) {
        ComputedDefinitionsRevision = other.ComputedDefinitionsRevision;
      }
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
      if ((tag & 7) == 4) {
        // Abort on any end group tag.
        return;
      }
      switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 10: {
            objectDefinitions_.AddEntriesFrom(input, _repeated_objectDefinitions_codec);
            break;
          }
          case 18: {
            ComputedDefinitionsRevision = input.ReadString();
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
      if ((tag & 7) == 4) {
        // Abort on any end group tag.
        return;
      }
      switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10: {
            objectDefinitions_.AddEntriesFrom(ref input, _repeated_objectDefinitions_codec);
            break;
          }
          case 18: {
            ComputedDefinitionsRevision = input.ReadString();
            break;
          }
        }
      }
    }
    #endif

  }

  /// <summary>
  /// WriteSchemaRequest is the required data used to "upsert" the Schema of a
  /// Permissions System.
  /// </summary>
  [global::System.Diagnostics.DebuggerDisplayAttribute("{ToString(),nq}")]
  public sealed partial class WriteSchemaRequest : pb::IMessage<WriteSchemaRequest>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<WriteSchemaRequest> _parser = new pb::MessageParser<WriteSchemaRequest>(() => new WriteSchemaRequest());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<WriteSchemaRequest> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::Authzed.Api.V1Alpha1.SchemaReflection.Descriptor.MessageTypes[2]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public WriteSchemaRequest() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public WriteSchemaRequest(WriteSchemaRequest other) : this() {
      schema_ = other.schema_;
      optionalDefinitionsRevisionPrecondition_ = other.optionalDefinitionsRevisionPrecondition_;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public WriteSchemaRequest Clone() {
      return new WriteSchemaRequest(this);
    }

    /// <summary>Field number for the "schema" field.</summary>
    public const int SchemaFieldNumber = 1;
    private string schema_ = "";
    /// <summary>
    /// The Schema containing one or more Object Definitions that will be written
    /// to the Permissions System.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string Schema {
      get { return schema_; }
      set {
        schema_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    /// <summary>Field number for the "optional_definitions_revision_precondition" field.</summary>
    public const int OptionalDefinitionsRevisionPreconditionFieldNumber = 2;
    private string optionalDefinitionsRevisionPrecondition_ = "";
    /// <summary>
    /// If specified, the existing revision of object definitions in the schema that must be present for
    /// the write to succeed. If the revision specified differs (i.e. the underlying schema has changed),
    /// the write call will fail with a FAILED_PRECONDITION error.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string OptionalDefinitionsRevisionPrecondition {
      get { return optionalDefinitionsRevisionPrecondition_; }
      set {
        optionalDefinitionsRevisionPrecondition_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as WriteSchemaRequest);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(WriteSchemaRequest other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if (Schema != other.Schema) return false;
      if (OptionalDefinitionsRevisionPrecondition != other.OptionalDefinitionsRevisionPrecondition) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      if (Schema.Length != 0) hash ^= Schema.GetHashCode();
      if (OptionalDefinitionsRevisionPrecondition.Length != 0) hash ^= OptionalDefinitionsRevisionPrecondition.GetHashCode();
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      if (Schema.Length != 0) {
        output.WriteRawTag(10);
        output.WriteString(Schema);
      }
      if (OptionalDefinitionsRevisionPrecondition.Length != 0) {
        output.WriteRawTag(18);
        output.WriteString(OptionalDefinitionsRevisionPrecondition);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      if (Schema.Length != 0) {
        output.WriteRawTag(10);
        output.WriteString(Schema);
      }
      if (OptionalDefinitionsRevisionPrecondition.Length != 0) {
        output.WriteRawTag(18);
        output.WriteString(OptionalDefinitionsRevisionPrecondition);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      if (Schema.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(Schema);
      }
      if (OptionalDefinitionsRevisionPrecondition.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(OptionalDefinitionsRevisionPrecondition);
      }
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(WriteSchemaRequest other) {
      if (other == null) {
        return;
      }
      if (other.Schema.Length != 0) {
        Schema = other.Schema;
      }
      if (other.OptionalDefinitionsRevisionPrecondition.Length != 0) {
        OptionalDefinitionsRevisionPrecondition = other.OptionalDefinitionsRevisionPrecondition;
      }
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
      if ((tag & 7) == 4) {
        // Abort on any end group tag.
        return;
      }
      switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 10: {
            Schema = input.ReadString();
            break;
          }
          case 18: {
            OptionalDefinitionsRevisionPrecondition = input.ReadString();
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
      if ((tag & 7) == 4) {
        // Abort on any end group tag.
        return;
      }
      switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10: {
            Schema = input.ReadString();
            break;
          }
          case 18: {
            OptionalDefinitionsRevisionPrecondition = input.ReadString();
            break;
          }
        }
      }
    }
    #endif

  }

  /// <summary>
  /// WriteSchemaResponse is the resulting data after having written a Schema to
  /// a Permissions System.
  /// </summary>
  [global::System.Diagnostics.DebuggerDisplayAttribute("{ToString(),nq}")]
  public sealed partial class WriteSchemaResponse : pb::IMessage<WriteSchemaResponse>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<WriteSchemaResponse> _parser = new pb::MessageParser<WriteSchemaResponse>(() => new WriteSchemaResponse());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<WriteSchemaResponse> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::Authzed.Api.V1Alpha1.SchemaReflection.Descriptor.MessageTypes[3]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public WriteSchemaResponse() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public WriteSchemaResponse(WriteSchemaResponse other) : this() {
      objectDefinitionsNames_ = other.objectDefinitionsNames_.Clone();
      computedDefinitionsRevision_ = other.computedDefinitionsRevision_;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public WriteSchemaResponse Clone() {
      return new WriteSchemaResponse(this);
    }

    /// <summary>Field number for the "object_definitions_names" field.</summary>
    public const int ObjectDefinitionsNamesFieldNumber = 1;
    private static readonly pb::FieldCodec<string> _repeated_objectDefinitionsNames_codec
        = pb::FieldCodec.ForString(10);
    private readonly pbc::RepeatedField<string> objectDefinitionsNames_ = new pbc::RepeatedField<string>();
    /// <summary>
    /// The names of the Object Definitions that were written.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public pbc::RepeatedField<string> ObjectDefinitionsNames {
      get { return objectDefinitionsNames_; }
    }

    /// <summary>Field number for the "computed_definitions_revision" field.</summary>
    public const int ComputedDefinitionsRevisionFieldNumber = 2;
    private string computedDefinitionsRevision_ = "";
    /// <summary>
    /// The computed revision of the written object definitions.
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string ComputedDefinitionsRevision {
      get { return computedDefinitionsRevision_; }
      set {
        computedDefinitionsRevision_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as WriteSchemaResponse);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(WriteSchemaResponse other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if(!objectDefinitionsNames_.Equals(other.objectDefinitionsNames_)) return false;
      if (ComputedDefinitionsRevision != other.ComputedDefinitionsRevision) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      hash ^= objectDefinitionsNames_.GetHashCode();
      if (ComputedDefinitionsRevision.Length != 0) hash ^= ComputedDefinitionsRevision.GetHashCode();
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      objectDefinitionsNames_.WriteTo(output, _repeated_objectDefinitionsNames_codec);
      if (ComputedDefinitionsRevision.Length != 0) {
        output.WriteRawTag(18);
        output.WriteString(ComputedDefinitionsRevision);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      objectDefinitionsNames_.WriteTo(ref output, _repeated_objectDefinitionsNames_codec);
      if (ComputedDefinitionsRevision.Length != 0) {
        output.WriteRawTag(18);
        output.WriteString(ComputedDefinitionsRevision);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      size += objectDefinitionsNames_.CalculateSize(_repeated_objectDefinitionsNames_codec);
      if (ComputedDefinitionsRevision.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(ComputedDefinitionsRevision);
      }
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(WriteSchemaResponse other) {
      if (other == null) {
        return;
      }
      objectDefinitionsNames_.Add(other.objectDefinitionsNames_);
      if (other.ComputedDefinitionsRevision.Length != 0) {
        ComputedDefinitionsRevision = other.ComputedDefinitionsRevision;
      }
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
      if ((tag & 7) == 4) {
        // Abort on any end group tag.
        return;
      }
      switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 10: {
            objectDefinitionsNames_.AddEntriesFrom(input, _repeated_objectDefinitionsNames_codec);
            break;
          }
          case 18: {
            ComputedDefinitionsRevision = input.ReadString();
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
      if ((tag & 7) == 4) {
        // Abort on any end group tag.
        return;
      }
      switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10: {
            objectDefinitionsNames_.AddEntriesFrom(ref input, _repeated_objectDefinitionsNames_codec);
            break;
          }
          case 18: {
            ComputedDefinitionsRevision = input.ReadString();
            break;
          }
        }
      }
    }
    #endif

  }

  #endregion

}

#endregion Designer generated code