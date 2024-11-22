# frozen_string_literal: true
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: authzed/api/v1alpha1/watchresources_service.proto

require 'google/protobuf'

require 'google/api/annotations_pb'
require 'validate/validate_pb'
require 'authzed/api/v1/core_pb'


descriptor_data = "\n1authzed/api/v1alpha1/watchresources_service.proto\x12\x14\x61uthzed.api.v1alpha1\x1a\x1cgoogle/api/annotations.proto\x1a\x17validate/validate.proto\x1a\x19\x61uthzed/api/v1/core.proto\"\x96\x03\n\x15WatchResourcesRequest\x12z\n\x14resource_object_type\x18\x01 \x01(\tBH\xfa\x42\x45rC(\x80\x01\x32>^([a-z][a-z0-9_]{1,61}[a-z0-9]/)*[a-z][a-z0-9_]{1,62}[a-z0-9]$R\x12resourceObjectType\x12G\n\npermission\x18\x02 \x01(\tB\'\xfa\x42$r\"(@2\x1e^[a-z][a-z0-9_]{1,62}[a-z0-9]$R\npermission\x12.\n\x13subject_object_type\x18\x03 \x01(\tR\x11subjectObjectType\x12:\n\x19optional_subject_relation\x18\x04 \x01(\tR\x17optionalSubjectRelation\x12L\n\x15optional_start_cursor\x18\x05 \x01(\x0b\x32\x18.authzed.api.v1.ZedTokenR\x13optionalStartCursor\"\x84\x03\n\x10PermissionUpdate\x12:\n\x07subject\x18\x01 \x01(\x0b\x32 .authzed.api.v1.SubjectReferenceR\x07subject\x12;\n\x08resource\x18\x02 \x01(\x0b\x32\x1f.authzed.api.v1.ObjectReferenceR\x08resource\x12\x1a\n\x08relation\x18\x03 \x01(\tR\x08relation\x12\x64\n\x12updated_permission\x18\x04 \x01(\x0e\x32\x35.authzed.api.v1alpha1.PermissionUpdate.PermissionshipR\x11updatedPermission\"u\n\x0ePermissionship\x12\x1e\n\x1aPERMISSIONSHIP_UNSPECIFIED\x10\x00\x12 \n\x1cPERMISSIONSHIP_NO_PERMISSION\x10\x01\x12!\n\x1dPERMISSIONSHIP_HAS_PERMISSION\x10\x02\"\x9d\x01\n\x16WatchResourcesResponse\x12@\n\x07updates\x18\x01 \x03(\x0b\x32&.authzed.api.v1alpha1.PermissionUpdateR\x07updates\x12\x41\n\x0f\x63hanges_through\x18\x02 \x01(\x0b\x32\x18.authzed.api.v1.ZedTokenR\x0e\x63hangesThrough2\xa9\x01\n\x15WatchResourcesService\x12\x8f\x01\n\x0eWatchResources\x12+.authzed.api.v1alpha1.WatchResourcesRequest\x1a,.authzed.api.v1alpha1.WatchResourcesResponse\" \x82\xd3\xe4\x93\x02\x1a\"\x15/v1alpha1/lookupwatch:\x01*0\x01\x42T\n\x18\x63om.authzed.api.v1alpha1Z8github.com/authzed/authzed-go/proto/authzed/api/v1alpha1b\x06proto3"

pool = Google::Protobuf::DescriptorPool.generated_pool
pool.add_serialized_file(descriptor_data)

module Authzed
  module Api
    module V1alpha1
      WatchResourcesRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("authzed.api.v1alpha1.WatchResourcesRequest").msgclass
      PermissionUpdate = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("authzed.api.v1alpha1.PermissionUpdate").msgclass
      PermissionUpdate::Permissionship = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("authzed.api.v1alpha1.PermissionUpdate.Permissionship").enummodule
      WatchResourcesResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("authzed.api.v1alpha1.WatchResourcesResponse").msgclass
    end
  end
end