// @generated
impl serde::Serialize for DeveloperError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        if self.line != 0 {
            len += 1;
        }
        if self.column != 0 {
            len += 1;
        }
        if self.source != 0 {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.context.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.DeveloperError", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if self.line != 0 {
            struct_ser.serialize_field("line", &self.line)?;
        }
        if self.column != 0 {
            struct_ser.serialize_field("column", &self.column)?;
        }
        if self.source != 0 {
            let v = developer_error::Source::from_i32(self.source)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.source)))?;
            struct_ser.serialize_field("source", &v)?;
        }
        if self.kind != 0 {
            let v = developer_error::ErrorKind::from_i32(self.kind)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.context.is_empty() {
            struct_ser.serialize_field("context", &self.context)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeveloperError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
            "line",
            "column",
            "source",
            "kind",
            "path",
            "context",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
            Line,
            Column,
            Source,
            Kind,
            Path,
            Context,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "message" => Ok(GeneratedField::Message),
                            "line" => Ok(GeneratedField::Line),
                            "column" => Ok(GeneratedField::Column),
                            "source" => Ok(GeneratedField::Source),
                            "kind" => Ok(GeneratedField::Kind),
                            "path" => Ok(GeneratedField::Path),
                            "context" => Ok(GeneratedField::Context),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeveloperError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.DeveloperError")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeveloperError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                let mut line__ = None;
                let mut column__ = None;
                let mut source__ = None;
                let mut kind__ = None;
                let mut path__ = None;
                let mut context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map.next_value()?);
                        }
                        GeneratedField::Line => {
                            if line__.is_some() {
                                return Err(serde::de::Error::duplicate_field("line"));
                            }
                            line__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map.next_value::<developer_error::Source>()? as i32);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map.next_value::<developer_error::ErrorKind>()? as i32);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeveloperError {
                    message: message__.unwrap_or_default(),
                    line: line__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    context: context__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.DeveloperError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for developer_error::ErrorKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UnknownKind => "UNKNOWN_KIND",
            Self::ParseError => "PARSE_ERROR",
            Self::SchemaIssue => "SCHEMA_ISSUE",
            Self::DuplicateRelationship => "DUPLICATE_RELATIONSHIP",
            Self::MissingExpectedRelationship => "MISSING_EXPECTED_RELATIONSHIP",
            Self::ExtraRelationshipFound => "EXTRA_RELATIONSHIP_FOUND",
            Self::UnknownObjectType => "UNKNOWN_OBJECT_TYPE",
            Self::UnknownRelation => "UNKNOWN_RELATION",
            Self::MaximumRecursion => "MAXIMUM_RECURSION",
            Self::AssertionFailed => "ASSERTION_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for developer_error::ErrorKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN_KIND",
            "PARSE_ERROR",
            "SCHEMA_ISSUE",
            "DUPLICATE_RELATIONSHIP",
            "MISSING_EXPECTED_RELATIONSHIP",
            "EXTRA_RELATIONSHIP_FOUND",
            "UNKNOWN_OBJECT_TYPE",
            "UNKNOWN_RELATION",
            "MAXIMUM_RECURSION",
            "ASSERTION_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = developer_error::ErrorKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(developer_error::ErrorKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(developer_error::ErrorKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN_KIND" => Ok(developer_error::ErrorKind::UnknownKind),
                    "PARSE_ERROR" => Ok(developer_error::ErrorKind::ParseError),
                    "SCHEMA_ISSUE" => Ok(developer_error::ErrorKind::SchemaIssue),
                    "DUPLICATE_RELATIONSHIP" => Ok(developer_error::ErrorKind::DuplicateRelationship),
                    "MISSING_EXPECTED_RELATIONSHIP" => Ok(developer_error::ErrorKind::MissingExpectedRelationship),
                    "EXTRA_RELATIONSHIP_FOUND" => Ok(developer_error::ErrorKind::ExtraRelationshipFound),
                    "UNKNOWN_OBJECT_TYPE" => Ok(developer_error::ErrorKind::UnknownObjectType),
                    "UNKNOWN_RELATION" => Ok(developer_error::ErrorKind::UnknownRelation),
                    "MAXIMUM_RECURSION" => Ok(developer_error::ErrorKind::MaximumRecursion),
                    "ASSERTION_FAILED" => Ok(developer_error::ErrorKind::AssertionFailed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for developer_error::Source {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UnknownSource => "UNKNOWN_SOURCE",
            Self::Schema => "SCHEMA",
            Self::Relationship => "RELATIONSHIP",
            Self::ValidationYaml => "VALIDATION_YAML",
            Self::CheckWatch => "CHECK_WATCH",
            Self::Assertion => "ASSERTION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for developer_error::Source {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN_SOURCE",
            "SCHEMA",
            "RELATIONSHIP",
            "VALIDATION_YAML",
            "CHECK_WATCH",
            "ASSERTION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = developer_error::Source;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(developer_error::Source::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(developer_error::Source::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN_SOURCE" => Ok(developer_error::Source::UnknownSource),
                    "SCHEMA" => Ok(developer_error::Source::Schema),
                    "RELATIONSHIP" => Ok(developer_error::Source::Relationship),
                    "VALIDATION_YAML" => Ok(developer_error::Source::ValidationYaml),
                    "CHECK_WATCH" => Ok(developer_error::Source::CheckWatch),
                    "ASSERTION" => Ok(developer_error::Source::Assertion),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for EditCheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.context.is_some() {
            len += 1;
        }
        if !self.check_relationships.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.EditCheckRequest", len)?;
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if !self.check_relationships.is_empty() {
            struct_ser.serialize_field("checkRelationships", &self.check_relationships)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EditCheckRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "context",
            "check_relationships",
            "checkRelationships",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Context,
            CheckRelationships,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "context" => Ok(GeneratedField::Context),
                            "checkRelationships" | "check_relationships" => Ok(GeneratedField::CheckRelationships),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EditCheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.EditCheckRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EditCheckRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut context__ = None;
                let mut check_relationships__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                        GeneratedField::CheckRelationships => {
                            if check_relationships__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkRelationships"));
                            }
                            check_relationships__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EditCheckRequest {
                    context: context__,
                    check_relationships: check_relationships__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.EditCheckRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EditCheckResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_errors.is_empty() {
            len += 1;
        }
        if !self.check_results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.EditCheckResponse", len)?;
        if !self.request_errors.is_empty() {
            struct_ser.serialize_field("requestErrors", &self.request_errors)?;
        }
        if !self.check_results.is_empty() {
            struct_ser.serialize_field("checkResults", &self.check_results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EditCheckResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_errors",
            "requestErrors",
            "check_results",
            "checkResults",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestErrors,
            CheckResults,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestErrors" | "request_errors" => Ok(GeneratedField::RequestErrors),
                            "checkResults" | "check_results" => Ok(GeneratedField::CheckResults),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EditCheckResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.EditCheckResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EditCheckResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_errors__ = None;
                let mut check_results__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestErrors => {
                            if request_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestErrors"));
                            }
                            request_errors__ = Some(map.next_value()?);
                        }
                        GeneratedField::CheckResults => {
                            if check_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkResults"));
                            }
                            check_results__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EditCheckResponse {
                    request_errors: request_errors__.unwrap_or_default(),
                    check_results: check_results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.EditCheckResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EditCheckResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.relationship.is_some() {
            len += 1;
        }
        if self.is_member {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.EditCheckResult", len)?;
        if let Some(v) = self.relationship.as_ref() {
            struct_ser.serialize_field("relationship", v)?;
        }
        if self.is_member {
            struct_ser.serialize_field("isMember", &self.is_member)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EditCheckResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relationship",
            "is_member",
            "isMember",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relationship,
            IsMember,
            Error,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "relationship" => Ok(GeneratedField::Relationship),
                            "isMember" | "is_member" => Ok(GeneratedField::IsMember),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EditCheckResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.EditCheckResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EditCheckResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relationship__ = None;
                let mut is_member__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Relationship => {
                            if relationship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            relationship__ = map.next_value()?;
                        }
                        GeneratedField::IsMember => {
                            if is_member__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isMember"));
                            }
                            is_member__ = Some(map.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                    }
                }
                Ok(EditCheckResult {
                    relationship: relationship__,
                    is_member: is_member__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.EditCheckResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FormatSchemaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.schema.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.FormatSchemaRequest", len)?;
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FormatSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "schema",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schema,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "schema" => Ok(GeneratedField::Schema),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FormatSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.FormatSchemaRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FormatSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schema__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FormatSchemaRequest {
                    schema: schema__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.FormatSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FormatSchemaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.error.is_some() {
            len += 1;
        }
        if !self.formatted_schema.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.FormatSchemaResponse", len)?;
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.formatted_schema.is_empty() {
            struct_ser.serialize_field("formattedSchema", &self.formatted_schema)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FormatSchemaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error",
            "formatted_schema",
            "formattedSchema",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
            FormattedSchema,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "error" => Ok(GeneratedField::Error),
                            "formattedSchema" | "formatted_schema" => Ok(GeneratedField::FormattedSchema),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FormatSchemaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.FormatSchemaResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FormatSchemaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error__ = None;
                let mut formatted_schema__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::FormattedSchema => {
                            if formatted_schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("formattedSchema"));
                            }
                            formatted_schema__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FormatSchemaResponse {
                    error: error__,
                    formatted_schema: formatted_schema__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.FormatSchemaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupShareRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.share_reference.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.LookupShareRequest", len)?;
        if !self.share_reference.is_empty() {
            struct_ser.serialize_field("shareReference", &self.share_reference)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupShareRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "share_reference",
            "shareReference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShareReference,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "shareReference" | "share_reference" => Ok(GeneratedField::ShareReference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupShareRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.LookupShareRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LookupShareRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut share_reference__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ShareReference => {
                            if share_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shareReference"));
                            }
                            share_reference__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LookupShareRequest {
                    share_reference: share_reference__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.LookupShareRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupShareResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if !self.schema.is_empty() {
            len += 1;
        }
        if !self.relationships_yaml.is_empty() {
            len += 1;
        }
        if !self.validation_yaml.is_empty() {
            len += 1;
        }
        if !self.assertions_yaml.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.LookupShareResponse", len)?;
        if self.status != 0 {
            let v = lookup_share_response::LookupStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        if !self.relationships_yaml.is_empty() {
            struct_ser.serialize_field("relationshipsYaml", &self.relationships_yaml)?;
        }
        if !self.validation_yaml.is_empty() {
            struct_ser.serialize_field("validationYaml", &self.validation_yaml)?;
        }
        if !self.assertions_yaml.is_empty() {
            struct_ser.serialize_field("assertionsYaml", &self.assertions_yaml)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupShareResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "schema",
            "relationships_yaml",
            "relationshipsYaml",
            "validation_yaml",
            "validationYaml",
            "assertions_yaml",
            "assertionsYaml",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Schema,
            RelationshipsYaml,
            ValidationYaml,
            AssertionsYaml,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "schema" => Ok(GeneratedField::Schema),
                            "relationshipsYaml" | "relationships_yaml" => Ok(GeneratedField::RelationshipsYaml),
                            "validationYaml" | "validation_yaml" => Ok(GeneratedField::ValidationYaml),
                            "assertionsYaml" | "assertions_yaml" => Ok(GeneratedField::AssertionsYaml),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupShareResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.LookupShareResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LookupShareResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut schema__ = None;
                let mut relationships_yaml__ = None;
                let mut validation_yaml__ = None;
                let mut assertions_yaml__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<lookup_share_response::LookupStatus>()? as i32);
                        }
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map.next_value()?);
                        }
                        GeneratedField::RelationshipsYaml => {
                            if relationships_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationshipsYaml"));
                            }
                            relationships_yaml__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidationYaml => {
                            if validation_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationYaml"));
                            }
                            validation_yaml__ = Some(map.next_value()?);
                        }
                        GeneratedField::AssertionsYaml => {
                            if assertions_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assertionsYaml"));
                            }
                            assertions_yaml__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LookupShareResponse {
                    status: status__.unwrap_or_default(),
                    schema: schema__.unwrap_or_default(),
                    relationships_yaml: relationships_yaml__.unwrap_or_default(),
                    validation_yaml: validation_yaml__.unwrap_or_default(),
                    assertions_yaml: assertions_yaml__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.LookupShareResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for lookup_share_response::LookupStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UnknownReference => "UNKNOWN_REFERENCE",
            Self::FailedToLookup => "FAILED_TO_LOOKUP",
            Self::ValidReference => "VALID_REFERENCE",
            Self::UpgradedReference => "UPGRADED_REFERENCE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for lookup_share_response::LookupStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN_REFERENCE",
            "FAILED_TO_LOOKUP",
            "VALID_REFERENCE",
            "UPGRADED_REFERENCE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = lookup_share_response::LookupStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(lookup_share_response::LookupStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(lookup_share_response::LookupStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN_REFERENCE" => Ok(lookup_share_response::LookupStatus::UnknownReference),
                    "FAILED_TO_LOOKUP" => Ok(lookup_share_response::LookupStatus::FailedToLookup),
                    "VALID_REFERENCE" => Ok(lookup_share_response::LookupStatus::ValidReference),
                    "UPGRADED_REFERENCE" => Ok(lookup_share_response::LookupStatus::UpgradedReference),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ObjectAndRelation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.namespace.is_empty() {
            len += 1;
        }
        if !self.object_id.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.ObjectAndRelation", len)?;
        if !self.namespace.is_empty() {
            struct_ser.serialize_field("namespace", &self.namespace)?;
        }
        if !self.object_id.is_empty() {
            struct_ser.serialize_field("objectId", &self.object_id)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObjectAndRelation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace",
            "object_id",
            "objectId",
            "relation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Namespace,
            ObjectId,
            Relation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "namespace" => Ok(GeneratedField::Namespace),
                            "objectId" | "object_id" => Ok(GeneratedField::ObjectId),
                            "relation" => Ok(GeneratedField::Relation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ObjectAndRelation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.ObjectAndRelation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ObjectAndRelation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace__ = None;
                let mut object_id__ = None;
                let mut relation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = Some(map.next_value()?);
                        }
                        GeneratedField::ObjectId => {
                            if object_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectId"));
                            }
                            object_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ObjectAndRelation {
                    namespace: namespace__.unwrap_or_default(),
                    object_id: object_id__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.ObjectAndRelation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.namespace.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.RelationReference", len)?;
        if !self.namespace.is_empty() {
            struct_ser.serialize_field("namespace", &self.namespace)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace",
            "relation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Namespace,
            Relation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "namespace" => Ok(GeneratedField::Namespace),
                            "relation" => Ok(GeneratedField::Relation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.RelationReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RelationReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace__ = None;
                let mut relation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = Some(map.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RelationReference {
                    namespace: namespace__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.RelationReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationTuple {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.object_and_relation.is_some() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.RelationTuple", len)?;
        if let Some(v) = self.object_and_relation.as_ref() {
            struct_ser.serialize_field("objectAndRelation", v)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationTuple {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_and_relation",
            "objectAndRelation",
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectAndRelation,
            User,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objectAndRelation" | "object_and_relation" => Ok(GeneratedField::ObjectAndRelation),
                            "user" => Ok(GeneratedField::User),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationTuple;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.RelationTuple")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RelationTuple, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_and_relation__ = None;
                let mut user__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ObjectAndRelation => {
                            if object_and_relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectAndRelation"));
                            }
                            object_and_relation__ = map.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map.next_value()?;
                        }
                    }
                }
                Ok(RelationTuple {
                    object_and_relation: object_and_relation__,
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.RelationTuple", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.schema.is_empty() {
            len += 1;
        }
        if !self.relationships.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.RequestContext", len)?;
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        if !self.relationships.is_empty() {
            struct_ser.serialize_field("relationships", &self.relationships)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "schema",
            "relationships",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schema,
            Relationships,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "schema" => Ok(GeneratedField::Schema),
                            "relationships" => Ok(GeneratedField::Relationships),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.RequestContext")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RequestContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schema__ = None;
                let mut relationships__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map.next_value()?);
                        }
                        GeneratedField::Relationships => {
                            if relationships__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationships"));
                            }
                            relationships__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RequestContext {
                    schema: schema__.unwrap_or_default(),
                    relationships: relationships__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.RequestContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShareRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.schema.is_empty() {
            len += 1;
        }
        if !self.relationships_yaml.is_empty() {
            len += 1;
        }
        if !self.validation_yaml.is_empty() {
            len += 1;
        }
        if !self.assertions_yaml.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.ShareRequest", len)?;
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        if !self.relationships_yaml.is_empty() {
            struct_ser.serialize_field("relationshipsYaml", &self.relationships_yaml)?;
        }
        if !self.validation_yaml.is_empty() {
            struct_ser.serialize_field("validationYaml", &self.validation_yaml)?;
        }
        if !self.assertions_yaml.is_empty() {
            struct_ser.serialize_field("assertionsYaml", &self.assertions_yaml)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShareRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "schema",
            "relationships_yaml",
            "relationshipsYaml",
            "validation_yaml",
            "validationYaml",
            "assertions_yaml",
            "assertionsYaml",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schema,
            RelationshipsYaml,
            ValidationYaml,
            AssertionsYaml,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "schema" => Ok(GeneratedField::Schema),
                            "relationshipsYaml" | "relationships_yaml" => Ok(GeneratedField::RelationshipsYaml),
                            "validationYaml" | "validation_yaml" => Ok(GeneratedField::ValidationYaml),
                            "assertionsYaml" | "assertions_yaml" => Ok(GeneratedField::AssertionsYaml),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShareRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.ShareRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ShareRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schema__ = None;
                let mut relationships_yaml__ = None;
                let mut validation_yaml__ = None;
                let mut assertions_yaml__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map.next_value()?);
                        }
                        GeneratedField::RelationshipsYaml => {
                            if relationships_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationshipsYaml"));
                            }
                            relationships_yaml__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidationYaml => {
                            if validation_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationYaml"));
                            }
                            validation_yaml__ = Some(map.next_value()?);
                        }
                        GeneratedField::AssertionsYaml => {
                            if assertions_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assertionsYaml"));
                            }
                            assertions_yaml__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ShareRequest {
                    schema: schema__.unwrap_or_default(),
                    relationships_yaml: relationships_yaml__.unwrap_or_default(),
                    validation_yaml: validation_yaml__.unwrap_or_default(),
                    assertions_yaml: assertions_yaml__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.ShareRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShareResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.share_reference.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.ShareResponse", len)?;
        if !self.share_reference.is_empty() {
            struct_ser.serialize_field("shareReference", &self.share_reference)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShareResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "share_reference",
            "shareReference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShareReference,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "shareReference" | "share_reference" => Ok(GeneratedField::ShareReference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShareResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.ShareResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ShareResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut share_reference__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ShareReference => {
                            if share_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shareReference"));
                            }
                            share_reference__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ShareResponse {
                    share_reference: share_reference__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.ShareResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpgradeSchemaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.namespace_configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.UpgradeSchemaRequest", len)?;
        if !self.namespace_configs.is_empty() {
            struct_ser.serialize_field("namespaceConfigs", &self.namespace_configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpgradeSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_configs",
            "namespaceConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceConfigs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "namespaceConfigs" | "namespace_configs" => Ok(GeneratedField::NamespaceConfigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpgradeSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.UpgradeSchemaRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpgradeSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_configs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NamespaceConfigs => {
                            if namespace_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceConfigs"));
                            }
                            namespace_configs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpgradeSchemaRequest {
                    namespace_configs: namespace_configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.UpgradeSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpgradeSchemaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.error.is_some() {
            len += 1;
        }
        if !self.upgraded_schema.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.UpgradeSchemaResponse", len)?;
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.upgraded_schema.is_empty() {
            struct_ser.serialize_field("upgradedSchema", &self.upgraded_schema)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpgradeSchemaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error",
            "upgraded_schema",
            "upgradedSchema",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
            UpgradedSchema,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "error" => Ok(GeneratedField::Error),
                            "upgradedSchema" | "upgraded_schema" => Ok(GeneratedField::UpgradedSchema),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpgradeSchemaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.UpgradeSchemaResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpgradeSchemaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error__ = None;
                let mut upgraded_schema__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::UpgradedSchema => {
                            if upgraded_schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgradedSchema"));
                            }
                            upgraded_schema__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpgradeSchemaResponse {
                    error: error__,
                    upgraded_schema: upgraded_schema__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.UpgradeSchemaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_oneof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.User", len)?;
        if let Some(v) = self.user_oneof.as_ref() {
            match v {
                user::UserOneof::Userset(v) => {
                    struct_ser.serialize_field("userset", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "userset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Userset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "userset" => Ok(GeneratedField::Userset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.User")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_oneof__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Userset => {
                            if user_oneof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userset"));
                            }
                            user_oneof__ = map.next_value::<::std::option::Option<_>>()?.map(user::UserOneof::Userset)
;
                        }
                    }
                }
                Ok(User {
                    user_oneof: user_oneof__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.User", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.context.is_some() {
            len += 1;
        }
        if !self.validation_yaml.is_empty() {
            len += 1;
        }
        if self.update_validation_yaml {
            len += 1;
        }
        if !self.assertions_yaml.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.ValidateRequest", len)?;
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if !self.validation_yaml.is_empty() {
            struct_ser.serialize_field("validationYaml", &self.validation_yaml)?;
        }
        if self.update_validation_yaml {
            struct_ser.serialize_field("updateValidationYaml", &self.update_validation_yaml)?;
        }
        if !self.assertions_yaml.is_empty() {
            struct_ser.serialize_field("assertionsYaml", &self.assertions_yaml)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "context",
            "validation_yaml",
            "validationYaml",
            "update_validation_yaml",
            "updateValidationYaml",
            "assertions_yaml",
            "assertionsYaml",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Context,
            ValidationYaml,
            UpdateValidationYaml,
            AssertionsYaml,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "context" => Ok(GeneratedField::Context),
                            "validationYaml" | "validation_yaml" => Ok(GeneratedField::ValidationYaml),
                            "updateValidationYaml" | "update_validation_yaml" => Ok(GeneratedField::UpdateValidationYaml),
                            "assertionsYaml" | "assertions_yaml" => Ok(GeneratedField::AssertionsYaml),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.ValidateRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ValidateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut context__ = None;
                let mut validation_yaml__ = None;
                let mut update_validation_yaml__ = None;
                let mut assertions_yaml__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                        GeneratedField::ValidationYaml => {
                            if validation_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationYaml"));
                            }
                            validation_yaml__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdateValidationYaml => {
                            if update_validation_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateValidationYaml"));
                            }
                            update_validation_yaml__ = Some(map.next_value()?);
                        }
                        GeneratedField::AssertionsYaml => {
                            if assertions_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assertionsYaml"));
                            }
                            assertions_yaml__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ValidateRequest {
                    context: context__,
                    validation_yaml: validation_yaml__.unwrap_or_default(),
                    update_validation_yaml: update_validation_yaml__.unwrap_or_default(),
                    assertions_yaml: assertions_yaml__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.ValidateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_errors.is_empty() {
            len += 1;
        }
        if !self.validation_errors.is_empty() {
            len += 1;
        }
        if !self.updated_validation_yaml.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v0.ValidateResponse", len)?;
        if !self.request_errors.is_empty() {
            struct_ser.serialize_field("requestErrors", &self.request_errors)?;
        }
        if !self.validation_errors.is_empty() {
            struct_ser.serialize_field("validationErrors", &self.validation_errors)?;
        }
        if !self.updated_validation_yaml.is_empty() {
            struct_ser.serialize_field("updatedValidationYaml", &self.updated_validation_yaml)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_errors",
            "requestErrors",
            "validation_errors",
            "validationErrors",
            "updated_validation_yaml",
            "updatedValidationYaml",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestErrors,
            ValidationErrors,
            UpdatedValidationYaml,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestErrors" | "request_errors" => Ok(GeneratedField::RequestErrors),
                            "validationErrors" | "validation_errors" => Ok(GeneratedField::ValidationErrors),
                            "updatedValidationYaml" | "updated_validation_yaml" => Ok(GeneratedField::UpdatedValidationYaml),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v0.ValidateResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ValidateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_errors__ = None;
                let mut validation_errors__ = None;
                let mut updated_validation_yaml__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestErrors => {
                            if request_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestErrors"));
                            }
                            request_errors__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidationErrors => {
                            if validation_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationErrors"));
                            }
                            validation_errors__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdatedValidationYaml => {
                            if updated_validation_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedValidationYaml"));
                            }
                            updated_validation_yaml__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ValidateResponse {
                    request_errors: request_errors__.unwrap_or_default(),
                    validation_errors: validation_errors__.unwrap_or_default(),
                    updated_validation_yaml: updated_validation_yaml__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v0.ValidateResponse", FIELDS, GeneratedVisitor)
    }
}
