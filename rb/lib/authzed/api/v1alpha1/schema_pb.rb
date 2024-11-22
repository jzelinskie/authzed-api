# frozen_string_literal: true
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: authzed/api/v1alpha1/schema.proto

require 'google/protobuf'

require 'validate/validate_pb'


descriptor_data = "\n!authzed/api/v1alpha1/schema.proto\x12\x14\x61uthzed.api.v1alpha1\x1a\x17validate/validate.proto\"\x9d\x01\n\x11ReadSchemaRequest\x12\x87\x01\n\x18object_definitions_names\x18\x01 \x03(\tBM\xfa\x42J\x92\x01G\"ErC(\x80\x01\x32>^([a-z][a-z0-9_]{1,62}[a-z0-9]/)*[a-z][a-z0-9_]{1,62}[a-z0-9]$R\x16objectDefinitionsNames\"\x87\x01\n\x12ReadSchemaResponse\x12-\n\x12object_definitions\x18\x01 \x03(\tR\x11objectDefinitions\x12\x42\n\x1d\x63omputed_definitions_revision\x18\x02 \x01(\tR\x1b\x63omputedDefinitionsRevision\"\x94\x01\n\x12WriteSchemaRequest\x12!\n\x06schema\x18\x01 \x01(\tB\t\xfa\x42\x06r\x04(\x80\x80\x10R\x06schema\x12[\n*optional_definitions_revision_precondition\x18\x02 \x01(\tR\'optionalDefinitionsRevisionPrecondition\"\x93\x01\n\x13WriteSchemaResponse\x12\x38\n\x18object_definitions_names\x18\x01 \x03(\tR\x16objectDefinitionsNames\x12\x42\n\x1d\x63omputed_definitions_revision\x18\x02 \x01(\tR\x1b\x63omputedDefinitionsRevision2\xd8\x01\n\rSchemaService\x12\x61\n\nReadSchema\x12\'.authzed.api.v1alpha1.ReadSchemaRequest\x1a(.authzed.api.v1alpha1.ReadSchemaResponse\"\x00\x12\x64\n\x0bWriteSchema\x12(.authzed.api.v1alpha1.WriteSchemaRequest\x1a).authzed.api.v1alpha1.WriteSchemaResponse\"\x00\x42T\n\x18\x63om.authzed.api.v1alpha1Z8github.com/authzed/authzed-go/proto/authzed/api/v1alpha1b\x06proto3"

pool = Google::Protobuf::DescriptorPool.generated_pool
pool.add_serialized_file(descriptor_data)

module Authzed
  module Api
    module V1alpha1
      ReadSchemaRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("authzed.api.v1alpha1.ReadSchemaRequest").msgclass
      ReadSchemaResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("authzed.api.v1alpha1.ReadSchemaResponse").msgclass
      WriteSchemaRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("authzed.api.v1alpha1.WriteSchemaRequest").msgclass
      WriteSchemaResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("authzed.api.v1alpha1.WriteSchemaResponse").msgclass
    end
  end
end