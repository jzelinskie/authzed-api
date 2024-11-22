Gem::Specification.new do |s|
  s.name        = "spicedb-protobuf"
  # NOTE: version is managed programmatically at release time
  # and reflects the associated git tag.
  s.version     = "0.1.0"
  s.licenses    = ["Apache-2.0"]
  s.summary     = "Generated Protobuf and gRPC types for SpiceDB"
  s.description = "A barebones library for interfacing with types used for SpiceDB. Created and maintained by Authzed"
  s.authors     = ["Jimmy Zelinskie"]
  s.email       = "support@authzed.com"
  s.files       = Dir.glob(%w[LICENSE README.md lib/**/*]).reject { |f| File.directory?(f) }
  s.homepage    = "https://authzed.com"
  s.metadata    = { 
    "bug_tracker_uri" => "https://github.com/authzed/api/issues",
    "github_repo"     => "ssh://github.com/authzed/api",
    "homepage_uri"    => "https://authzed.com",
    "source_code_uri" => "https://github.com/authzed/api",
  }

  s.add_runtime_dependency "grpc", "~> 1.41"
  s.add_runtime_dependency "grpc-tools", "~> 1.41"

  s.add_development_dependency "rake", "~> 13"
  s.add_development_dependency "bump", "~> 0.10"
end
