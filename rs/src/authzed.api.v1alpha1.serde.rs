// @generated
impl serde::Serialize for PermissionUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subject.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if self.updated_permission != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1alpha1.PermissionUpdate", len)?;
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if self.updated_permission != 0 {
            let v = permission_update::Permissionship::from_i32(self.updated_permission)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.updated_permission)))?;
            struct_ser.serialize_field("updatedPermission", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject",
            "resource",
            "relation",
            "updated_permission",
            "updatedPermission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Resource,
            Relation,
            UpdatedPermission,
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
                            "subject" => Ok(GeneratedField::Subject),
                            "resource" => Ok(GeneratedField::Resource),
                            "relation" => Ok(GeneratedField::Relation),
                            "updatedPermission" | "updated_permission" => Ok(GeneratedField::UpdatedPermission),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1alpha1.PermissionUpdate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PermissionUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject__ = None;
                let mut resource__ = None;
                let mut relation__ = None;
                let mut updated_permission__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map.next_value()?;
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map.next_value()?;
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdatedPermission => {
                            if updated_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedPermission"));
                            }
                            updated_permission__ = Some(map.next_value::<permission_update::Permissionship>()? as i32);
                        }
                    }
                }
                Ok(PermissionUpdate {
                    subject: subject__,
                    resource: resource__,
                    relation: relation__.unwrap_or_default(),
                    updated_permission: updated_permission__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1alpha1.PermissionUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for permission_update::Permissionship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PERMISSIONSHIP_UNSPECIFIED",
            Self::NoPermission => "PERMISSIONSHIP_NO_PERMISSION",
            Self::HasPermission => "PERMISSIONSHIP_HAS_PERMISSION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for permission_update::Permissionship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PERMISSIONSHIP_UNSPECIFIED",
            "PERMISSIONSHIP_NO_PERMISSION",
            "PERMISSIONSHIP_HAS_PERMISSION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = permission_update::Permissionship;

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
                    .and_then(permission_update::Permissionship::from_i32)
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
                    .and_then(permission_update::Permissionship::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PERMISSIONSHIP_UNSPECIFIED" => Ok(permission_update::Permissionship::Unspecified),
                    "PERMISSIONSHIP_NO_PERMISSION" => Ok(permission_update::Permissionship::NoPermission),
                    "PERMISSIONSHIP_HAS_PERMISSION" => Ok(permission_update::Permissionship::HasPermission),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReadSchemaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.object_definitions_names.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1alpha1.ReadSchemaRequest", len)?;
        if !self.object_definitions_names.is_empty() {
            struct_ser.serialize_field("objectDefinitionsNames", &self.object_definitions_names)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_definitions_names",
            "objectDefinitionsNames",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectDefinitionsNames,
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
                            "objectDefinitionsNames" | "object_definitions_names" => Ok(GeneratedField::ObjectDefinitionsNames),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1alpha1.ReadSchemaRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ReadSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_definitions_names__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ObjectDefinitionsNames => {
                            if object_definitions_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectDefinitionsNames"));
                            }
                            object_definitions_names__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ReadSchemaRequest {
                    object_definitions_names: object_definitions_names__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1alpha1.ReadSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadSchemaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.object_definitions.is_empty() {
            len += 1;
        }
        if !self.computed_definitions_revision.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1alpha1.ReadSchemaResponse", len)?;
        if !self.object_definitions.is_empty() {
            struct_ser.serialize_field("objectDefinitions", &self.object_definitions)?;
        }
        if !self.computed_definitions_revision.is_empty() {
            struct_ser.serialize_field("computedDefinitionsRevision", &self.computed_definitions_revision)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadSchemaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_definitions",
            "objectDefinitions",
            "computed_definitions_revision",
            "computedDefinitionsRevision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectDefinitions,
            ComputedDefinitionsRevision,
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
                            "objectDefinitions" | "object_definitions" => Ok(GeneratedField::ObjectDefinitions),
                            "computedDefinitionsRevision" | "computed_definitions_revision" => Ok(GeneratedField::ComputedDefinitionsRevision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadSchemaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1alpha1.ReadSchemaResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ReadSchemaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_definitions__ = None;
                let mut computed_definitions_revision__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ObjectDefinitions => {
                            if object_definitions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectDefinitions"));
                            }
                            object_definitions__ = Some(map.next_value()?);
                        }
                        GeneratedField::ComputedDefinitionsRevision => {
                            if computed_definitions_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("computedDefinitionsRevision"));
                            }
                            computed_definitions_revision__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ReadSchemaResponse {
                    object_definitions: object_definitions__.unwrap_or_default(),
                    computed_definitions_revision: computed_definitions_revision__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1alpha1.ReadSchemaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchResourcesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_object_type.is_empty() {
            len += 1;
        }
        if !self.permission.is_empty() {
            len += 1;
        }
        if !self.subject_object_type.is_empty() {
            len += 1;
        }
        if !self.optional_subject_relation.is_empty() {
            len += 1;
        }
        if self.optional_start_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1alpha1.WatchResourcesRequest", len)?;
        if !self.resource_object_type.is_empty() {
            struct_ser.serialize_field("resourceObjectType", &self.resource_object_type)?;
        }
        if !self.permission.is_empty() {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        if !self.subject_object_type.is_empty() {
            struct_ser.serialize_field("subjectObjectType", &self.subject_object_type)?;
        }
        if !self.optional_subject_relation.is_empty() {
            struct_ser.serialize_field("optionalSubjectRelation", &self.optional_subject_relation)?;
        }
        if let Some(v) = self.optional_start_cursor.as_ref() {
            struct_ser.serialize_field("optionalStartCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchResourcesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_object_type",
            "resourceObjectType",
            "permission",
            "subject_object_type",
            "subjectObjectType",
            "optional_subject_relation",
            "optionalSubjectRelation",
            "optional_start_cursor",
            "optionalStartCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceObjectType,
            Permission,
            SubjectObjectType,
            OptionalSubjectRelation,
            OptionalStartCursor,
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
                            "resourceObjectType" | "resource_object_type" => Ok(GeneratedField::ResourceObjectType),
                            "permission" => Ok(GeneratedField::Permission),
                            "subjectObjectType" | "subject_object_type" => Ok(GeneratedField::SubjectObjectType),
                            "optionalSubjectRelation" | "optional_subject_relation" => Ok(GeneratedField::OptionalSubjectRelation),
                            "optionalStartCursor" | "optional_start_cursor" => Ok(GeneratedField::OptionalStartCursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchResourcesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1alpha1.WatchResourcesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchResourcesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_object_type__ = None;
                let mut permission__ = None;
                let mut subject_object_type__ = None;
                let mut optional_subject_relation__ = None;
                let mut optional_start_cursor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceObjectType => {
                            if resource_object_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceObjectType"));
                            }
                            resource_object_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubjectObjectType => {
                            if subject_object_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectObjectType"));
                            }
                            subject_object_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalSubjectRelation => {
                            if optional_subject_relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalSubjectRelation"));
                            }
                            optional_subject_relation__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalStartCursor => {
                            if optional_start_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalStartCursor"));
                            }
                            optional_start_cursor__ = map.next_value()?;
                        }
                    }
                }
                Ok(WatchResourcesRequest {
                    resource_object_type: resource_object_type__.unwrap_or_default(),
                    permission: permission__.unwrap_or_default(),
                    subject_object_type: subject_object_type__.unwrap_or_default(),
                    optional_subject_relation: optional_subject_relation__.unwrap_or_default(),
                    optional_start_cursor: optional_start_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1alpha1.WatchResourcesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchResourcesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.updates.is_empty() {
            len += 1;
        }
        if self.changes_through.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1alpha1.WatchResourcesResponse", len)?;
        if !self.updates.is_empty() {
            struct_ser.serialize_field("updates", &self.updates)?;
        }
        if let Some(v) = self.changes_through.as_ref() {
            struct_ser.serialize_field("changesThrough", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchResourcesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "updates",
            "changes_through",
            "changesThrough",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Updates,
            ChangesThrough,
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
                            "updates" => Ok(GeneratedField::Updates),
                            "changesThrough" | "changes_through" => Ok(GeneratedField::ChangesThrough),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchResourcesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1alpha1.WatchResourcesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchResourcesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut updates__ = None;
                let mut changes_through__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Updates => {
                            if updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updates"));
                            }
                            updates__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChangesThrough => {
                            if changes_through__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changesThrough"));
                            }
                            changes_through__ = map.next_value()?;
                        }
                    }
                }
                Ok(WatchResourcesResponse {
                    updates: updates__.unwrap_or_default(),
                    changes_through: changes_through__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1alpha1.WatchResourcesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteSchemaRequest {
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
        if !self.optional_definitions_revision_precondition.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1alpha1.WriteSchemaRequest", len)?;
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        if !self.optional_definitions_revision_precondition.is_empty() {
            struct_ser.serialize_field("optionalDefinitionsRevisionPrecondition", &self.optional_definitions_revision_precondition)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "schema",
            "optional_definitions_revision_precondition",
            "optionalDefinitionsRevisionPrecondition",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schema,
            OptionalDefinitionsRevisionPrecondition,
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
                            "optionalDefinitionsRevisionPrecondition" | "optional_definitions_revision_precondition" => Ok(GeneratedField::OptionalDefinitionsRevisionPrecondition),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1alpha1.WriteSchemaRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WriteSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schema__ = None;
                let mut optional_definitions_revision_precondition__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalDefinitionsRevisionPrecondition => {
                            if optional_definitions_revision_precondition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalDefinitionsRevisionPrecondition"));
                            }
                            optional_definitions_revision_precondition__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WriteSchemaRequest {
                    schema: schema__.unwrap_or_default(),
                    optional_definitions_revision_precondition: optional_definitions_revision_precondition__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1alpha1.WriteSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteSchemaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.object_definitions_names.is_empty() {
            len += 1;
        }
        if !self.computed_definitions_revision.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1alpha1.WriteSchemaResponse", len)?;
        if !self.object_definitions_names.is_empty() {
            struct_ser.serialize_field("objectDefinitionsNames", &self.object_definitions_names)?;
        }
        if !self.computed_definitions_revision.is_empty() {
            struct_ser.serialize_field("computedDefinitionsRevision", &self.computed_definitions_revision)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteSchemaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_definitions_names",
            "objectDefinitionsNames",
            "computed_definitions_revision",
            "computedDefinitionsRevision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectDefinitionsNames,
            ComputedDefinitionsRevision,
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
                            "objectDefinitionsNames" | "object_definitions_names" => Ok(GeneratedField::ObjectDefinitionsNames),
                            "computedDefinitionsRevision" | "computed_definitions_revision" => Ok(GeneratedField::ComputedDefinitionsRevision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteSchemaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1alpha1.WriteSchemaResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WriteSchemaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_definitions_names__ = None;
                let mut computed_definitions_revision__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ObjectDefinitionsNames => {
                            if object_definitions_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectDefinitionsNames"));
                            }
                            object_definitions_names__ = Some(map.next_value()?);
                        }
                        GeneratedField::ComputedDefinitionsRevision => {
                            if computed_definitions_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("computedDefinitionsRevision"));
                            }
                            computed_definitions_revision__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WriteSchemaResponse {
                    object_definitions_names: object_definitions_names__.unwrap_or_default(),
                    computed_definitions_revision: computed_definitions_revision__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1alpha1.WriteSchemaResponse", FIELDS, GeneratedVisitor)
    }
}
