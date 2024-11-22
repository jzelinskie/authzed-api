// @generated by protoc-gen-es v1.10.0 with parameter "target=ts"
// @generated from file authzed/api/v0/developer.proto (package authzed.api.v0, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message, proto3 } from "@bufbuild/protobuf";
import { RelationTuple } from "./core_pb.js";

/**
 * @generated from message authzed.api.v0.FormatSchemaRequest
 */
export class FormatSchemaRequest extends Message<FormatSchemaRequest> {
  /**
   * @generated from field: string schema = 1;
   */
  schema = "";

  constructor(data?: PartialMessage<FormatSchemaRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.FormatSchemaRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "schema", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): FormatSchemaRequest {
    return new FormatSchemaRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): FormatSchemaRequest {
    return new FormatSchemaRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): FormatSchemaRequest {
    return new FormatSchemaRequest().fromJsonString(jsonString, options);
  }

  static equals(a: FormatSchemaRequest | PlainMessage<FormatSchemaRequest> | undefined, b: FormatSchemaRequest | PlainMessage<FormatSchemaRequest> | undefined): boolean {
    return proto3.util.equals(FormatSchemaRequest, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.FormatSchemaResponse
 */
export class FormatSchemaResponse extends Message<FormatSchemaResponse> {
  /**
   * @generated from field: authzed.api.v0.DeveloperError error = 1;
   */
  error?: DeveloperError;

  /**
   * @generated from field: string formatted_schema = 2;
   */
  formattedSchema = "";

  constructor(data?: PartialMessage<FormatSchemaResponse>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.FormatSchemaResponse";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "error", kind: "message", T: DeveloperError },
    { no: 2, name: "formatted_schema", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): FormatSchemaResponse {
    return new FormatSchemaResponse().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): FormatSchemaResponse {
    return new FormatSchemaResponse().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): FormatSchemaResponse {
    return new FormatSchemaResponse().fromJsonString(jsonString, options);
  }

  static equals(a: FormatSchemaResponse | PlainMessage<FormatSchemaResponse> | undefined, b: FormatSchemaResponse | PlainMessage<FormatSchemaResponse> | undefined): boolean {
    return proto3.util.equals(FormatSchemaResponse, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.UpgradeSchemaRequest
 */
export class UpgradeSchemaRequest extends Message<UpgradeSchemaRequest> {
  /**
   * @generated from field: repeated string namespace_configs = 1;
   */
  namespaceConfigs: string[] = [];

  constructor(data?: PartialMessage<UpgradeSchemaRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.UpgradeSchemaRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "namespace_configs", kind: "scalar", T: 9 /* ScalarType.STRING */, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): UpgradeSchemaRequest {
    return new UpgradeSchemaRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): UpgradeSchemaRequest {
    return new UpgradeSchemaRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): UpgradeSchemaRequest {
    return new UpgradeSchemaRequest().fromJsonString(jsonString, options);
  }

  static equals(a: UpgradeSchemaRequest | PlainMessage<UpgradeSchemaRequest> | undefined, b: UpgradeSchemaRequest | PlainMessage<UpgradeSchemaRequest> | undefined): boolean {
    return proto3.util.equals(UpgradeSchemaRequest, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.UpgradeSchemaResponse
 */
export class UpgradeSchemaResponse extends Message<UpgradeSchemaResponse> {
  /**
   * @generated from field: authzed.api.v0.DeveloperError error = 1;
   */
  error?: DeveloperError;

  /**
   * @generated from field: string upgraded_schema = 2;
   */
  upgradedSchema = "";

  constructor(data?: PartialMessage<UpgradeSchemaResponse>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.UpgradeSchemaResponse";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "error", kind: "message", T: DeveloperError },
    { no: 2, name: "upgraded_schema", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): UpgradeSchemaResponse {
    return new UpgradeSchemaResponse().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): UpgradeSchemaResponse {
    return new UpgradeSchemaResponse().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): UpgradeSchemaResponse {
    return new UpgradeSchemaResponse().fromJsonString(jsonString, options);
  }

  static equals(a: UpgradeSchemaResponse | PlainMessage<UpgradeSchemaResponse> | undefined, b: UpgradeSchemaResponse | PlainMessage<UpgradeSchemaResponse> | undefined): boolean {
    return proto3.util.equals(UpgradeSchemaResponse, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.ShareRequest
 */
export class ShareRequest extends Message<ShareRequest> {
  /**
   * @generated from field: string schema = 1;
   */
  schema = "";

  /**
   * @generated from field: string relationships_yaml = 2;
   */
  relationshipsYaml = "";

  /**
   * @generated from field: string validation_yaml = 3;
   */
  validationYaml = "";

  /**
   * @generated from field: string assertions_yaml = 4;
   */
  assertionsYaml = "";

  constructor(data?: PartialMessage<ShareRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.ShareRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "schema", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "relationships_yaml", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 3, name: "validation_yaml", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 4, name: "assertions_yaml", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): ShareRequest {
    return new ShareRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): ShareRequest {
    return new ShareRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): ShareRequest {
    return new ShareRequest().fromJsonString(jsonString, options);
  }

  static equals(a: ShareRequest | PlainMessage<ShareRequest> | undefined, b: ShareRequest | PlainMessage<ShareRequest> | undefined): boolean {
    return proto3.util.equals(ShareRequest, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.ShareResponse
 */
export class ShareResponse extends Message<ShareResponse> {
  /**
   * @generated from field: string share_reference = 1;
   */
  shareReference = "";

  constructor(data?: PartialMessage<ShareResponse>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.ShareResponse";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "share_reference", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): ShareResponse {
    return new ShareResponse().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): ShareResponse {
    return new ShareResponse().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): ShareResponse {
    return new ShareResponse().fromJsonString(jsonString, options);
  }

  static equals(a: ShareResponse | PlainMessage<ShareResponse> | undefined, b: ShareResponse | PlainMessage<ShareResponse> | undefined): boolean {
    return proto3.util.equals(ShareResponse, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.LookupShareRequest
 */
export class LookupShareRequest extends Message<LookupShareRequest> {
  /**
   * @generated from field: string share_reference = 1;
   */
  shareReference = "";

  constructor(data?: PartialMessage<LookupShareRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.LookupShareRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "share_reference", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): LookupShareRequest {
    return new LookupShareRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): LookupShareRequest {
    return new LookupShareRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): LookupShareRequest {
    return new LookupShareRequest().fromJsonString(jsonString, options);
  }

  static equals(a: LookupShareRequest | PlainMessage<LookupShareRequest> | undefined, b: LookupShareRequest | PlainMessage<LookupShareRequest> | undefined): boolean {
    return proto3.util.equals(LookupShareRequest, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.LookupShareResponse
 */
export class LookupShareResponse extends Message<LookupShareResponse> {
  /**
   * @generated from field: authzed.api.v0.LookupShareResponse.LookupStatus status = 1;
   */
  status = LookupShareResponse_LookupStatus.UNKNOWN_REFERENCE;

  /**
   * @generated from field: string schema = 2;
   */
  schema = "";

  /**
   * @generated from field: string relationships_yaml = 3;
   */
  relationshipsYaml = "";

  /**
   * @generated from field: string validation_yaml = 4;
   */
  validationYaml = "";

  /**
   * @generated from field: string assertions_yaml = 5;
   */
  assertionsYaml = "";

  constructor(data?: PartialMessage<LookupShareResponse>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.LookupShareResponse";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "status", kind: "enum", T: proto3.getEnumType(LookupShareResponse_LookupStatus) },
    { no: 2, name: "schema", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 3, name: "relationships_yaml", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 4, name: "validation_yaml", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 5, name: "assertions_yaml", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): LookupShareResponse {
    return new LookupShareResponse().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): LookupShareResponse {
    return new LookupShareResponse().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): LookupShareResponse {
    return new LookupShareResponse().fromJsonString(jsonString, options);
  }

  static equals(a: LookupShareResponse | PlainMessage<LookupShareResponse> | undefined, b: LookupShareResponse | PlainMessage<LookupShareResponse> | undefined): boolean {
    return proto3.util.equals(LookupShareResponse, a, b);
  }
}

/**
 * @generated from enum authzed.api.v0.LookupShareResponse.LookupStatus
 */
export enum LookupShareResponse_LookupStatus {
  /**
   * @generated from enum value: UNKNOWN_REFERENCE = 0;
   */
  UNKNOWN_REFERENCE = 0,

  /**
   * @generated from enum value: FAILED_TO_LOOKUP = 1;
   */
  FAILED_TO_LOOKUP = 1,

  /**
   * @generated from enum value: VALID_REFERENCE = 2;
   */
  VALID_REFERENCE = 2,

  /**
   * @generated from enum value: UPGRADED_REFERENCE = 3;
   */
  UPGRADED_REFERENCE = 3,
}
// Retrieve enum metadata with: proto3.getEnumType(LookupShareResponse_LookupStatus)
proto3.util.setEnumType(LookupShareResponse_LookupStatus, "authzed.api.v0.LookupShareResponse.LookupStatus", [
  { no: 0, name: "UNKNOWN_REFERENCE" },
  { no: 1, name: "FAILED_TO_LOOKUP" },
  { no: 2, name: "VALID_REFERENCE" },
  { no: 3, name: "UPGRADED_REFERENCE" },
]);

/**
 * @generated from message authzed.api.v0.RequestContext
 */
export class RequestContext extends Message<RequestContext> {
  /**
   * @generated from field: string schema = 1;
   */
  schema = "";

  /**
   * @generated from field: repeated authzed.api.v0.RelationTuple relationships = 2;
   */
  relationships: RelationTuple[] = [];

  constructor(data?: PartialMessage<RequestContext>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.RequestContext";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "schema", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "relationships", kind: "message", T: RelationTuple, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): RequestContext {
    return new RequestContext().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): RequestContext {
    return new RequestContext().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): RequestContext {
    return new RequestContext().fromJsonString(jsonString, options);
  }

  static equals(a: RequestContext | PlainMessage<RequestContext> | undefined, b: RequestContext | PlainMessage<RequestContext> | undefined): boolean {
    return proto3.util.equals(RequestContext, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.EditCheckRequest
 */
export class EditCheckRequest extends Message<EditCheckRequest> {
  /**
   * @generated from field: authzed.api.v0.RequestContext context = 1;
   */
  context?: RequestContext;

  /**
   * @generated from field: repeated authzed.api.v0.RelationTuple check_relationships = 2;
   */
  checkRelationships: RelationTuple[] = [];

  constructor(data?: PartialMessage<EditCheckRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.EditCheckRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "context", kind: "message", T: RequestContext },
    { no: 2, name: "check_relationships", kind: "message", T: RelationTuple, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): EditCheckRequest {
    return new EditCheckRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): EditCheckRequest {
    return new EditCheckRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): EditCheckRequest {
    return new EditCheckRequest().fromJsonString(jsonString, options);
  }

  static equals(a: EditCheckRequest | PlainMessage<EditCheckRequest> | undefined, b: EditCheckRequest | PlainMessage<EditCheckRequest> | undefined): boolean {
    return proto3.util.equals(EditCheckRequest, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.EditCheckResult
 */
export class EditCheckResult extends Message<EditCheckResult> {
  /**
   * @generated from field: authzed.api.v0.RelationTuple relationship = 1;
   */
  relationship?: RelationTuple;

  /**
   * @generated from field: bool is_member = 2;
   */
  isMember = false;

  /**
   * @generated from field: authzed.api.v0.DeveloperError error = 3;
   */
  error?: DeveloperError;

  constructor(data?: PartialMessage<EditCheckResult>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.EditCheckResult";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "relationship", kind: "message", T: RelationTuple },
    { no: 2, name: "is_member", kind: "scalar", T: 8 /* ScalarType.BOOL */ },
    { no: 3, name: "error", kind: "message", T: DeveloperError },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): EditCheckResult {
    return new EditCheckResult().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): EditCheckResult {
    return new EditCheckResult().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): EditCheckResult {
    return new EditCheckResult().fromJsonString(jsonString, options);
  }

  static equals(a: EditCheckResult | PlainMessage<EditCheckResult> | undefined, b: EditCheckResult | PlainMessage<EditCheckResult> | undefined): boolean {
    return proto3.util.equals(EditCheckResult, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.EditCheckResponse
 */
export class EditCheckResponse extends Message<EditCheckResponse> {
  /**
   * @generated from field: repeated authzed.api.v0.DeveloperError request_errors = 1;
   */
  requestErrors: DeveloperError[] = [];

  /**
   * @generated from field: repeated authzed.api.v0.EditCheckResult check_results = 2;
   */
  checkResults: EditCheckResult[] = [];

  constructor(data?: PartialMessage<EditCheckResponse>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.EditCheckResponse";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "request_errors", kind: "message", T: DeveloperError, repeated: true },
    { no: 2, name: "check_results", kind: "message", T: EditCheckResult, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): EditCheckResponse {
    return new EditCheckResponse().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): EditCheckResponse {
    return new EditCheckResponse().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): EditCheckResponse {
    return new EditCheckResponse().fromJsonString(jsonString, options);
  }

  static equals(a: EditCheckResponse | PlainMessage<EditCheckResponse> | undefined, b: EditCheckResponse | PlainMessage<EditCheckResponse> | undefined): boolean {
    return proto3.util.equals(EditCheckResponse, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.ValidateRequest
 */
export class ValidateRequest extends Message<ValidateRequest> {
  /**
   * @generated from field: authzed.api.v0.RequestContext context = 1;
   */
  context?: RequestContext;

  /**
   * @generated from field: string validation_yaml = 3;
   */
  validationYaml = "";

  /**
   * @generated from field: bool update_validation_yaml = 4;
   */
  updateValidationYaml = false;

  /**
   * @generated from field: string assertions_yaml = 5;
   */
  assertionsYaml = "";

  constructor(data?: PartialMessage<ValidateRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.ValidateRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "context", kind: "message", T: RequestContext },
    { no: 3, name: "validation_yaml", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 4, name: "update_validation_yaml", kind: "scalar", T: 8 /* ScalarType.BOOL */ },
    { no: 5, name: "assertions_yaml", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): ValidateRequest {
    return new ValidateRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): ValidateRequest {
    return new ValidateRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): ValidateRequest {
    return new ValidateRequest().fromJsonString(jsonString, options);
  }

  static equals(a: ValidateRequest | PlainMessage<ValidateRequest> | undefined, b: ValidateRequest | PlainMessage<ValidateRequest> | undefined): boolean {
    return proto3.util.equals(ValidateRequest, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.ValidateResponse
 */
export class ValidateResponse extends Message<ValidateResponse> {
  /**
   * @generated from field: repeated authzed.api.v0.DeveloperError request_errors = 1;
   */
  requestErrors: DeveloperError[] = [];

  /**
   * @generated from field: repeated authzed.api.v0.DeveloperError validation_errors = 2;
   */
  validationErrors: DeveloperError[] = [];

  /**
   * @generated from field: string updated_validation_yaml = 3;
   */
  updatedValidationYaml = "";

  constructor(data?: PartialMessage<ValidateResponse>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.ValidateResponse";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "request_errors", kind: "message", T: DeveloperError, repeated: true },
    { no: 2, name: "validation_errors", kind: "message", T: DeveloperError, repeated: true },
    { no: 3, name: "updated_validation_yaml", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): ValidateResponse {
    return new ValidateResponse().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): ValidateResponse {
    return new ValidateResponse().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): ValidateResponse {
    return new ValidateResponse().fromJsonString(jsonString, options);
  }

  static equals(a: ValidateResponse | PlainMessage<ValidateResponse> | undefined, b: ValidateResponse | PlainMessage<ValidateResponse> | undefined): boolean {
    return proto3.util.equals(ValidateResponse, a, b);
  }
}

/**
 * @generated from message authzed.api.v0.DeveloperError
 */
export class DeveloperError extends Message<DeveloperError> {
  /**
   * @generated from field: string message = 1;
   */
  message = "";

  /**
   * @generated from field: uint32 line = 2;
   */
  line = 0;

  /**
   * @generated from field: uint32 column = 3;
   */
  column = 0;

  /**
   * @generated from field: authzed.api.v0.DeveloperError.Source source = 4;
   */
  source = DeveloperError_Source.UNKNOWN_SOURCE;

  /**
   * @generated from field: authzed.api.v0.DeveloperError.ErrorKind kind = 5;
   */
  kind = DeveloperError_ErrorKind.UNKNOWN_KIND;

  /**
   * @generated from field: repeated string path = 6;
   */
  path: string[] = [];

  /**
   * context holds the context for the error. For schema issues, this will be the
   * name of the object type. For relationship issues, the full relationship string.
   *
   * @generated from field: string context = 7;
   */
  context = "";

  constructor(data?: PartialMessage<DeveloperError>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v0.DeveloperError";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "message", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "line", kind: "scalar", T: 13 /* ScalarType.UINT32 */ },
    { no: 3, name: "column", kind: "scalar", T: 13 /* ScalarType.UINT32 */ },
    { no: 4, name: "source", kind: "enum", T: proto3.getEnumType(DeveloperError_Source) },
    { no: 5, name: "kind", kind: "enum", T: proto3.getEnumType(DeveloperError_ErrorKind) },
    { no: 6, name: "path", kind: "scalar", T: 9 /* ScalarType.STRING */, repeated: true },
    { no: 7, name: "context", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): DeveloperError {
    return new DeveloperError().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): DeveloperError {
    return new DeveloperError().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): DeveloperError {
    return new DeveloperError().fromJsonString(jsonString, options);
  }

  static equals(a: DeveloperError | PlainMessage<DeveloperError> | undefined, b: DeveloperError | PlainMessage<DeveloperError> | undefined): boolean {
    return proto3.util.equals(DeveloperError, a, b);
  }
}

/**
 * @generated from enum authzed.api.v0.DeveloperError.Source
 */
export enum DeveloperError_Source {
  /**
   * @generated from enum value: UNKNOWN_SOURCE = 0;
   */
  UNKNOWN_SOURCE = 0,

  /**
   * @generated from enum value: SCHEMA = 1;
   */
  SCHEMA = 1,

  /**
   * @generated from enum value: RELATIONSHIP = 2;
   */
  RELATIONSHIP = 2,

  /**
   * @generated from enum value: VALIDATION_YAML = 3;
   */
  VALIDATION_YAML = 3,

  /**
   * @generated from enum value: CHECK_WATCH = 4;
   */
  CHECK_WATCH = 4,

  /**
   * @generated from enum value: ASSERTION = 5;
   */
  ASSERTION = 5,
}
// Retrieve enum metadata with: proto3.getEnumType(DeveloperError_Source)
proto3.util.setEnumType(DeveloperError_Source, "authzed.api.v0.DeveloperError.Source", [
  { no: 0, name: "UNKNOWN_SOURCE" },
  { no: 1, name: "SCHEMA" },
  { no: 2, name: "RELATIONSHIP" },
  { no: 3, name: "VALIDATION_YAML" },
  { no: 4, name: "CHECK_WATCH" },
  { no: 5, name: "ASSERTION" },
]);

/**
 * @generated from enum authzed.api.v0.DeveloperError.ErrorKind
 */
export enum DeveloperError_ErrorKind {
  /**
   * @generated from enum value: UNKNOWN_KIND = 0;
   */
  UNKNOWN_KIND = 0,

  /**
   * @generated from enum value: PARSE_ERROR = 1;
   */
  PARSE_ERROR = 1,

  /**
   * @generated from enum value: SCHEMA_ISSUE = 2;
   */
  SCHEMA_ISSUE = 2,

  /**
   * @generated from enum value: DUPLICATE_RELATIONSHIP = 3;
   */
  DUPLICATE_RELATIONSHIP = 3,

  /**
   * @generated from enum value: MISSING_EXPECTED_RELATIONSHIP = 4;
   */
  MISSING_EXPECTED_RELATIONSHIP = 4,

  /**
   * @generated from enum value: EXTRA_RELATIONSHIP_FOUND = 5;
   */
  EXTRA_RELATIONSHIP_FOUND = 5,

  /**
   * @generated from enum value: UNKNOWN_OBJECT_TYPE = 6;
   */
  UNKNOWN_OBJECT_TYPE = 6,

  /**
   * @generated from enum value: UNKNOWN_RELATION = 7;
   */
  UNKNOWN_RELATION = 7,

  /**
   * @generated from enum value: MAXIMUM_RECURSION = 8;
   */
  MAXIMUM_RECURSION = 8,

  /**
   * @generated from enum value: ASSERTION_FAILED = 9;
   */
  ASSERTION_FAILED = 9,
}
// Retrieve enum metadata with: proto3.getEnumType(DeveloperError_ErrorKind)
proto3.util.setEnumType(DeveloperError_ErrorKind, "authzed.api.v0.DeveloperError.ErrorKind", [
  { no: 0, name: "UNKNOWN_KIND" },
  { no: 1, name: "PARSE_ERROR" },
  { no: 2, name: "SCHEMA_ISSUE" },
  { no: 3, name: "DUPLICATE_RELATIONSHIP" },
  { no: 4, name: "MISSING_EXPECTED_RELATIONSHIP" },
  { no: 5, name: "EXTRA_RELATIONSHIP_FOUND" },
  { no: 6, name: "UNKNOWN_OBJECT_TYPE" },
  { no: 7, name: "UNKNOWN_RELATION" },
  { no: 8, name: "MAXIMUM_RECURSION" },
  { no: 9, name: "ASSERTION_FAILED" },
]);
