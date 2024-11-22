# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: authzed/api/v0/developer.proto
# Protobuf Python Version: 5.27.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    27,
    1,
    '',
    'authzed/api/v0/developer.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from authzed.api.v0 import core_pb2 as authzed_dot_api_dot_v0_dot_core__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1e\x61uthzed/api/v0/developer.proto\x12\x0e\x61uthzed.api.v0\x1a\x19\x61uthzed/api/v0/core.proto\"-\n\x13\x46ormatSchemaRequest\x12\x16\n\x06schema\x18\x01 \x01(\tR\x06schema\"w\n\x14\x46ormatSchemaResponse\x12\x34\n\x05\x65rror\x18\x01 \x01(\x0b\x32\x1e.authzed.api.v0.DeveloperErrorR\x05\x65rror\x12)\n\x10\x66ormatted_schema\x18\x02 \x01(\tR\x0f\x66ormattedSchema\"C\n\x14UpgradeSchemaRequest\x12+\n\x11namespace_configs\x18\x01 \x03(\tR\x10namespaceConfigs\"v\n\x15UpgradeSchemaResponse\x12\x34\n\x05\x65rror\x18\x01 \x01(\x0b\x32\x1e.authzed.api.v0.DeveloperErrorR\x05\x65rror\x12\'\n\x0fupgraded_schema\x18\x02 \x01(\tR\x0eupgradedSchema\"\xa7\x01\n\x0cShareRequest\x12\x16\n\x06schema\x18\x01 \x01(\tR\x06schema\x12-\n\x12relationships_yaml\x18\x02 \x01(\tR\x11relationshipsYaml\x12\'\n\x0fvalidation_yaml\x18\x03 \x01(\tR\x0evalidationYaml\x12\'\n\x0f\x61ssertions_yaml\x18\x04 \x01(\tR\x0e\x61ssertionsYaml\"8\n\rShareResponse\x12\'\n\x0fshare_reference\x18\x01 \x01(\tR\x0eshareReference\"=\n\x12LookupShareRequest\x12\'\n\x0fshare_reference\x18\x01 \x01(\tR\x0eshareReference\"\xe2\x02\n\x13LookupShareResponse\x12H\n\x06status\x18\x01 \x01(\x0e\x32\x30.authzed.api.v0.LookupShareResponse.LookupStatusR\x06status\x12\x16\n\x06schema\x18\x02 \x01(\tR\x06schema\x12-\n\x12relationships_yaml\x18\x03 \x01(\tR\x11relationshipsYaml\x12\'\n\x0fvalidation_yaml\x18\x04 \x01(\tR\x0evalidationYaml\x12\'\n\x0f\x61ssertions_yaml\x18\x05 \x01(\tR\x0e\x61ssertionsYaml\"h\n\x0cLookupStatus\x12\x15\n\x11UNKNOWN_REFERENCE\x10\x00\x12\x14\n\x10\x46\x41ILED_TO_LOOKUP\x10\x01\x12\x13\n\x0fVALID_REFERENCE\x10\x02\x12\x16\n\x12UPGRADED_REFERENCE\x10\x03\"s\n\x0eRequestContext\x12\x16\n\x06schema\x18\x01 \x01(\tR\x06schema\x12\x43\n\rrelationships\x18\x02 \x03(\x0b\x32\x1d.authzed.api.v0.RelationTupleR\rrelationshipsJ\x04\x08\x03\x10\x04\"\x9c\x01\n\x10\x45\x64itCheckRequest\x12\x38\n\x07\x63ontext\x18\x01 \x01(\x0b\x32\x1e.authzed.api.v0.RequestContextR\x07\x63ontext\x12N\n\x13\x63heck_relationships\x18\x02 \x03(\x0b\x32\x1d.authzed.api.v0.RelationTupleR\x12\x63heckRelationships\"\xa7\x01\n\x0f\x45\x64itCheckResult\x12\x41\n\x0crelationship\x18\x01 \x01(\x0b\x32\x1d.authzed.api.v0.RelationTupleR\x0crelationship\x12\x1b\n\tis_member\x18\x02 \x01(\x08R\x08isMember\x12\x34\n\x05\x65rror\x18\x03 \x01(\x0b\x32\x1e.authzed.api.v0.DeveloperErrorR\x05\x65rror\"\xa0\x01\n\x11\x45\x64itCheckResponse\x12\x45\n\x0erequest_errors\x18\x01 \x03(\x0b\x32\x1e.authzed.api.v0.DeveloperErrorR\rrequestErrors\x12\x44\n\rcheck_results\x18\x02 \x03(\x0b\x32\x1f.authzed.api.v0.EditCheckResultR\x0c\x63heckResults\"\xd3\x01\n\x0fValidateRequest\x12\x38\n\x07\x63ontext\x18\x01 \x01(\x0b\x32\x1e.authzed.api.v0.RequestContextR\x07\x63ontext\x12\'\n\x0fvalidation_yaml\x18\x03 \x01(\tR\x0evalidationYaml\x12\x34\n\x16update_validation_yaml\x18\x04 \x01(\x08R\x14updateValidationYaml\x12\'\n\x0f\x61ssertions_yaml\x18\x05 \x01(\tR\x0e\x61ssertionsYaml\"\xde\x01\n\x10ValidateResponse\x12\x45\n\x0erequest_errors\x18\x01 \x03(\x0b\x32\x1e.authzed.api.v0.DeveloperErrorR\rrequestErrors\x12K\n\x11validation_errors\x18\x02 \x03(\x0b\x32\x1e.authzed.api.v0.DeveloperErrorR\x10validationErrors\x12\x36\n\x17updated_validation_yaml\x18\x03 \x01(\tR\x15updatedValidationYaml\"\xee\x04\n\x0e\x44\x65veloperError\x12\x18\n\x07message\x18\x01 \x01(\tR\x07message\x12\x12\n\x04line\x18\x02 \x01(\rR\x04line\x12\x16\n\x06\x63olumn\x18\x03 \x01(\rR\x06\x63olumn\x12=\n\x06source\x18\x04 \x01(\x0e\x32%.authzed.api.v0.DeveloperError.SourceR\x06source\x12<\n\x04kind\x18\x05 \x01(\x0e\x32(.authzed.api.v0.DeveloperError.ErrorKindR\x04kind\x12\x12\n\x04path\x18\x06 \x03(\tR\x04path\x12\x18\n\x07\x63ontext\x18\x07 \x01(\tR\x07\x63ontext\"o\n\x06Source\x12\x12\n\x0eUNKNOWN_SOURCE\x10\x00\x12\n\n\x06SCHEMA\x10\x01\x12\x10\n\x0cRELATIONSHIP\x10\x02\x12\x13\n\x0fVALIDATION_YAML\x10\x03\x12\x0f\n\x0b\x43HECK_WATCH\x10\x04\x12\r\n\tASSERTION\x10\x05\"\xf9\x01\n\tErrorKind\x12\x10\n\x0cUNKNOWN_KIND\x10\x00\x12\x0f\n\x0bPARSE_ERROR\x10\x01\x12\x10\n\x0cSCHEMA_ISSUE\x10\x02\x12\x1a\n\x16\x44UPLICATE_RELATIONSHIP\x10\x03\x12!\n\x1dMISSING_EXPECTED_RELATIONSHIP\x10\x04\x12\x1c\n\x18\x45XTRA_RELATIONSHIP_FOUND\x10\x05\x12\x17\n\x13UNKNOWN_OBJECT_TYPE\x10\x06\x12\x14\n\x10UNKNOWN_RELATION\x10\x07\x12\x15\n\x11MAXIMUM_RECURSION\x10\x08\x12\x14\n\x10\x41SSERTION_FAILED\x10\t2\x97\x04\n\x10\x44\x65veloperService\x12R\n\tEditCheck\x12 .authzed.api.v0.EditCheckRequest\x1a!.authzed.api.v0.EditCheckResponse\"\x00\x12O\n\x08Validate\x12\x1f.authzed.api.v0.ValidateRequest\x1a .authzed.api.v0.ValidateResponse\"\x00\x12\x46\n\x05Share\x12\x1c.authzed.api.v0.ShareRequest\x1a\x1d.authzed.api.v0.ShareResponse\"\x00\x12Y\n\x0cLookupShared\x12\".authzed.api.v0.LookupShareRequest\x1a#.authzed.api.v0.LookupShareResponse\"\x00\x12^\n\rUpgradeSchema\x12$.authzed.api.v0.UpgradeSchemaRequest\x1a%.authzed.api.v0.UpgradeSchemaResponse\"\x00\x12[\n\x0c\x46ormatSchema\x12#.authzed.api.v0.FormatSchemaRequest\x1a$.authzed.api.v0.FormatSchemaResponse\"\x00\x42H\n\x12\x63om.authzed.api.v0Z2github.com/authzed/authzed-go/proto/authzed/api/v0b\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'authzed.api.v0.developer_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\022com.authzed.api.v0Z2github.com/authzed/authzed-go/proto/authzed/api/v0'
  _globals['_FORMATSCHEMAREQUEST']._serialized_start=77
  _globals['_FORMATSCHEMAREQUEST']._serialized_end=122
  _globals['_FORMATSCHEMARESPONSE']._serialized_start=124
  _globals['_FORMATSCHEMARESPONSE']._serialized_end=243
  _globals['_UPGRADESCHEMAREQUEST']._serialized_start=245
  _globals['_UPGRADESCHEMAREQUEST']._serialized_end=312
  _globals['_UPGRADESCHEMARESPONSE']._serialized_start=314
  _globals['_UPGRADESCHEMARESPONSE']._serialized_end=432
  _globals['_SHAREREQUEST']._serialized_start=435
  _globals['_SHAREREQUEST']._serialized_end=602
  _globals['_SHARERESPONSE']._serialized_start=604
  _globals['_SHARERESPONSE']._serialized_end=660
  _globals['_LOOKUPSHAREREQUEST']._serialized_start=662
  _globals['_LOOKUPSHAREREQUEST']._serialized_end=723
  _globals['_LOOKUPSHARERESPONSE']._serialized_start=726
  _globals['_LOOKUPSHARERESPONSE']._serialized_end=1080
  _globals['_LOOKUPSHARERESPONSE_LOOKUPSTATUS']._serialized_start=976
  _globals['_LOOKUPSHARERESPONSE_LOOKUPSTATUS']._serialized_end=1080
  _globals['_REQUESTCONTEXT']._serialized_start=1082
  _globals['_REQUESTCONTEXT']._serialized_end=1197
  _globals['_EDITCHECKREQUEST']._serialized_start=1200
  _globals['_EDITCHECKREQUEST']._serialized_end=1356
  _globals['_EDITCHECKRESULT']._serialized_start=1359
  _globals['_EDITCHECKRESULT']._serialized_end=1526
  _globals['_EDITCHECKRESPONSE']._serialized_start=1529
  _globals['_EDITCHECKRESPONSE']._serialized_end=1689
  _globals['_VALIDATEREQUEST']._serialized_start=1692
  _globals['_VALIDATEREQUEST']._serialized_end=1903
  _globals['_VALIDATERESPONSE']._serialized_start=1906
  _globals['_VALIDATERESPONSE']._serialized_end=2128
  _globals['_DEVELOPERERROR']._serialized_start=2131
  _globals['_DEVELOPERERROR']._serialized_end=2753
  _globals['_DEVELOPERERROR_SOURCE']._serialized_start=2390
  _globals['_DEVELOPERERROR_SOURCE']._serialized_end=2501
  _globals['_DEVELOPERERROR_ERRORKIND']._serialized_start=2504
  _globals['_DEVELOPERERROR_ERRORKIND']._serialized_end=2753
  _globals['_DEVELOPERSERVICE']._serialized_start=2756
  _globals['_DEVELOPERSERVICE']._serialized_end=3291
# @@protoc_insertion_point(module_scope)