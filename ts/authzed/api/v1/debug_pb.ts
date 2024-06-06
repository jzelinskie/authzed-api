// @generated by protoc-gen-es v1.10.0 with parameter "target=ts"
// @generated from file authzed/api/v1/debug.proto (package authzed.api.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Duration, Message, proto3, Struct } from "@bufbuild/protobuf";
import { ObjectReference, PartialCaveatInfo, SubjectReference } from "./core_pb.js";

/**
 * DebugInformation defines debug information returned by an API call in a footer when
 * requested with a specific debugging header.
 *
 * The specific debug information returned will depend on the type of the API call made.
 *
 * See the github.com/authzed/authzed-go project for the specific header and footer names.
 *
 * @generated from message authzed.api.v1.DebugInformation
 */
export class DebugInformation extends Message<DebugInformation> {
  /**
   * check holds debug information about a check request.
   *
   * @generated from field: authzed.api.v1.CheckDebugTrace check = 1;
   */
  check?: CheckDebugTrace;

  /**
   * schema_used holds the schema used for the request.
   *
   * @generated from field: string schema_used = 2;
   */
  schemaUsed = "";

  constructor(data?: PartialMessage<DebugInformation>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v1.DebugInformation";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "check", kind: "message", T: CheckDebugTrace },
    { no: 2, name: "schema_used", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): DebugInformation {
    return new DebugInformation().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): DebugInformation {
    return new DebugInformation().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): DebugInformation {
    return new DebugInformation().fromJsonString(jsonString, options);
  }

  static equals(a: DebugInformation | PlainMessage<DebugInformation> | undefined, b: DebugInformation | PlainMessage<DebugInformation> | undefined): boolean {
    return proto3.util.equals(DebugInformation, a, b);
  }
}

/**
 * CheckDebugTrace is a recursive trace of the requests made for resolving a CheckPermission
 * API call.
 *
 * @generated from message authzed.api.v1.CheckDebugTrace
 */
export class CheckDebugTrace extends Message<CheckDebugTrace> {
  /**
   * resource holds the resource on which the Check was performed.
   *
   * @generated from field: authzed.api.v1.ObjectReference resource = 1;
   */
  resource?: ObjectReference;

  /**
   * permission holds the name of the permission or relation on which the Check was performed.
   *
   * @generated from field: string permission = 2;
   */
  permission = "";

  /**
   * permission_type holds information indicating whether it was a permission or relation.
   *
   * @generated from field: authzed.api.v1.CheckDebugTrace.PermissionType permission_type = 3;
   */
  permissionType = CheckDebugTrace_PermissionType.UNSPECIFIED;

  /**
   * subject holds the subject on which the Check was performed. This will be static across all calls within
   * the same Check tree.
   *
   * @generated from field: authzed.api.v1.SubjectReference subject = 4;
   */
  subject?: SubjectReference;

  /**
   * result holds the result of the Check call.
   *
   * @generated from field: authzed.api.v1.CheckDebugTrace.Permissionship result = 5;
   */
  result = CheckDebugTrace_Permissionship.UNSPECIFIED;

  /**
   * caveat_evaluation_info holds information about the caveat evaluated for this step of the trace.
   *
   * @generated from field: authzed.api.v1.CaveatEvalInfo caveat_evaluation_info = 8;
   */
  caveatEvaluationInfo?: CaveatEvalInfo;

  /**
   * duration holds the time spent executing this Check operation.
   *
   * @generated from field: google.protobuf.Duration duration = 9;
   */
  duration?: Duration;

  /**
   * resolution holds information about how the problem was resolved.
   *
   * @generated from oneof authzed.api.v1.CheckDebugTrace.resolution
   */
  resolution: {
    /**
     * was_cached_result, if true, indicates that the result was found in the cache and returned directly.
     *
     * @generated from field: bool was_cached_result = 6;
     */
    value: boolean;
    case: "wasCachedResult";
  } | {
    /**
     * sub_problems holds the sub problems that were executed to resolve the answer to this Check. An empty list
     * and a permissionship of PERMISSIONSHIP_HAS_PERMISSION indicates the subject was found within this relation.
     *
     * @generated from field: authzed.api.v1.CheckDebugTrace.SubProblems sub_problems = 7;
     */
    value: CheckDebugTrace_SubProblems;
    case: "subProblems";
  } | { case: undefined; value?: undefined } = { case: undefined };

  constructor(data?: PartialMessage<CheckDebugTrace>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v1.CheckDebugTrace";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "resource", kind: "message", T: ObjectReference },
    { no: 2, name: "permission", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 3, name: "permission_type", kind: "enum", T: proto3.getEnumType(CheckDebugTrace_PermissionType) },
    { no: 4, name: "subject", kind: "message", T: SubjectReference },
    { no: 5, name: "result", kind: "enum", T: proto3.getEnumType(CheckDebugTrace_Permissionship) },
    { no: 8, name: "caveat_evaluation_info", kind: "message", T: CaveatEvalInfo },
    { no: 9, name: "duration", kind: "message", T: Duration },
    { no: 6, name: "was_cached_result", kind: "scalar", T: 8 /* ScalarType.BOOL */, oneof: "resolution" },
    { no: 7, name: "sub_problems", kind: "message", T: CheckDebugTrace_SubProblems, oneof: "resolution" },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): CheckDebugTrace {
    return new CheckDebugTrace().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): CheckDebugTrace {
    return new CheckDebugTrace().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): CheckDebugTrace {
    return new CheckDebugTrace().fromJsonString(jsonString, options);
  }

  static equals(a: CheckDebugTrace | PlainMessage<CheckDebugTrace> | undefined, b: CheckDebugTrace | PlainMessage<CheckDebugTrace> | undefined): boolean {
    return proto3.util.equals(CheckDebugTrace, a, b);
  }
}

/**
 * @generated from enum authzed.api.v1.CheckDebugTrace.PermissionType
 */
export enum CheckDebugTrace_PermissionType {
  /**
   * @generated from enum value: PERMISSION_TYPE_UNSPECIFIED = 0;
   */
  UNSPECIFIED = 0,

  /**
   * @generated from enum value: PERMISSION_TYPE_RELATION = 1;
   */
  RELATION = 1,

  /**
   * @generated from enum value: PERMISSION_TYPE_PERMISSION = 2;
   */
  PERMISSION = 2,
}
// Retrieve enum metadata with: proto3.getEnumType(CheckDebugTrace_PermissionType)
proto3.util.setEnumType(CheckDebugTrace_PermissionType, "authzed.api.v1.CheckDebugTrace.PermissionType", [
  { no: 0, name: "PERMISSION_TYPE_UNSPECIFIED" },
  { no: 1, name: "PERMISSION_TYPE_RELATION" },
  { no: 2, name: "PERMISSION_TYPE_PERMISSION" },
]);

/**
 * @generated from enum authzed.api.v1.CheckDebugTrace.Permissionship
 */
export enum CheckDebugTrace_Permissionship {
  /**
   * @generated from enum value: PERMISSIONSHIP_UNSPECIFIED = 0;
   */
  UNSPECIFIED = 0,

  /**
   * @generated from enum value: PERMISSIONSHIP_NO_PERMISSION = 1;
   */
  NO_PERMISSION = 1,

  /**
   * @generated from enum value: PERMISSIONSHIP_HAS_PERMISSION = 2;
   */
  HAS_PERMISSION = 2,

  /**
   * @generated from enum value: PERMISSIONSHIP_CONDITIONAL_PERMISSION = 3;
   */
  CONDITIONAL_PERMISSION = 3,
}
// Retrieve enum metadata with: proto3.getEnumType(CheckDebugTrace_Permissionship)
proto3.util.setEnumType(CheckDebugTrace_Permissionship, "authzed.api.v1.CheckDebugTrace.Permissionship", [
  { no: 0, name: "PERMISSIONSHIP_UNSPECIFIED" },
  { no: 1, name: "PERMISSIONSHIP_NO_PERMISSION" },
  { no: 2, name: "PERMISSIONSHIP_HAS_PERMISSION" },
  { no: 3, name: "PERMISSIONSHIP_CONDITIONAL_PERMISSION" },
]);

/**
 * @generated from message authzed.api.v1.CheckDebugTrace.SubProblems
 */
export class CheckDebugTrace_SubProblems extends Message<CheckDebugTrace_SubProblems> {
  /**
   * @generated from field: repeated authzed.api.v1.CheckDebugTrace traces = 1;
   */
  traces: CheckDebugTrace[] = [];

  constructor(data?: PartialMessage<CheckDebugTrace_SubProblems>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v1.CheckDebugTrace.SubProblems";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "traces", kind: "message", T: CheckDebugTrace, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): CheckDebugTrace_SubProblems {
    return new CheckDebugTrace_SubProblems().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): CheckDebugTrace_SubProblems {
    return new CheckDebugTrace_SubProblems().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): CheckDebugTrace_SubProblems {
    return new CheckDebugTrace_SubProblems().fromJsonString(jsonString, options);
  }

  static equals(a: CheckDebugTrace_SubProblems | PlainMessage<CheckDebugTrace_SubProblems> | undefined, b: CheckDebugTrace_SubProblems | PlainMessage<CheckDebugTrace_SubProblems> | undefined): boolean {
    return proto3.util.equals(CheckDebugTrace_SubProblems, a, b);
  }
}

/**
 * CaveatEvalInfo holds information about a caveat expression that was evaluated.
 *
 * @generated from message authzed.api.v1.CaveatEvalInfo
 */
export class CaveatEvalInfo extends Message<CaveatEvalInfo> {
  /**
   * expression is the expression that was evaluated.
   *
   * @generated from field: string expression = 1;
   */
  expression = "";

  /**
   * result is the result of the evaluation.
   *
   * @generated from field: authzed.api.v1.CaveatEvalInfo.Result result = 2;
   */
  result = CaveatEvalInfo_Result.UNSPECIFIED;

  /**
   * context consists of any named values that were used for evaluating the caveat expression.
   *
   * @generated from field: google.protobuf.Struct context = 3;
   */
  context?: Struct;

  /**
   * partial_caveat_info holds information of a partially-evaluated caveated response, if applicable.
   *
   * @generated from field: authzed.api.v1.PartialCaveatInfo partial_caveat_info = 4;
   */
  partialCaveatInfo?: PartialCaveatInfo;

  /**
   * caveat_name is the name of the caveat that was executed, if applicable.
   *
   * @generated from field: string caveat_name = 5;
   */
  caveatName = "";

  constructor(data?: PartialMessage<CaveatEvalInfo>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "authzed.api.v1.CaveatEvalInfo";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "expression", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "result", kind: "enum", T: proto3.getEnumType(CaveatEvalInfo_Result) },
    { no: 3, name: "context", kind: "message", T: Struct },
    { no: 4, name: "partial_caveat_info", kind: "message", T: PartialCaveatInfo },
    { no: 5, name: "caveat_name", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): CaveatEvalInfo {
    return new CaveatEvalInfo().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): CaveatEvalInfo {
    return new CaveatEvalInfo().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): CaveatEvalInfo {
    return new CaveatEvalInfo().fromJsonString(jsonString, options);
  }

  static equals(a: CaveatEvalInfo | PlainMessage<CaveatEvalInfo> | undefined, b: CaveatEvalInfo | PlainMessage<CaveatEvalInfo> | undefined): boolean {
    return proto3.util.equals(CaveatEvalInfo, a, b);
  }
}

/**
 * @generated from enum authzed.api.v1.CaveatEvalInfo.Result
 */
export enum CaveatEvalInfo_Result {
  /**
   * @generated from enum value: RESULT_UNSPECIFIED = 0;
   */
  UNSPECIFIED = 0,

  /**
   * @generated from enum value: RESULT_UNEVALUATED = 1;
   */
  UNEVALUATED = 1,

  /**
   * @generated from enum value: RESULT_FALSE = 2;
   */
  FALSE = 2,

  /**
   * @generated from enum value: RESULT_TRUE = 3;
   */
  TRUE = 3,

  /**
   * @generated from enum value: RESULT_MISSING_SOME_CONTEXT = 4;
   */
  MISSING_SOME_CONTEXT = 4,
}
// Retrieve enum metadata with: proto3.getEnumType(CaveatEvalInfo_Result)
proto3.util.setEnumType(CaveatEvalInfo_Result, "authzed.api.v1.CaveatEvalInfo.Result", [
  { no: 0, name: "RESULT_UNSPECIFIED" },
  { no: 1, name: "RESULT_UNEVALUATED" },
  { no: 2, name: "RESULT_FALSE" },
  { no: 3, name: "RESULT_TRUE" },
  { no: 4, name: "RESULT_MISSING_SOME_CONTEXT" },
]);

