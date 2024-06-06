// @generated
impl serde::Serialize for Cursor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.limit != 0 {
            len += 1;
        }
        if self.token.is_some() {
            len += 1;
        }
        if self.starting_index != 0 {
            len += 1;
        }
        if self.completed_members {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.Cursor", len)?;
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if let Some(v) = self.token.as_ref() {
            struct_ser.serialize_field("token", v)?;
        }
        if self.starting_index != 0 {
            struct_ser.serialize_field("startingIndex", &self.starting_index)?;
        }
        if self.completed_members {
            struct_ser.serialize_field("completedMembers", &self.completed_members)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Cursor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limit",
            "token",
            "starting_index",
            "startingIndex",
            "completed_members",
            "completedMembers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limit,
            Token,
            StartingIndex,
            CompletedMembers,
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
                            "limit" => Ok(GeneratedField::Limit),
                            "token" => Ok(GeneratedField::Token),
                            "startingIndex" | "starting_index" => Ok(GeneratedField::StartingIndex),
                            "completedMembers" | "completed_members" => Ok(GeneratedField::CompletedMembers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Cursor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.Cursor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Cursor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limit__ = None;
                let mut token__ = None;
                let mut starting_index__ = None;
                let mut completed_members__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = map.next_value()?;
                        }
                        GeneratedField::StartingIndex => {
                            if starting_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startingIndex"));
                            }
                            starting_index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CompletedMembers => {
                            if completed_members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completedMembers"));
                            }
                            completed_members__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Cursor {
                    limit: limit__.unwrap_or_default(),
                    token: token__,
                    starting_index: starting_index__.unwrap_or_default(),
                    completed_members: completed_members__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.Cursor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupPermissionSetsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.limit != 0 {
            len += 1;
        }
        if self.optional_starting_after_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.LookupPermissionSetsRequest", len)?;
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if let Some(v) = self.optional_starting_after_cursor.as_ref() {
            struct_ser.serialize_field("optionalStartingAfterCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupPermissionSetsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limit",
            "optional_starting_after_cursor",
            "optionalStartingAfterCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limit,
            OptionalStartingAfterCursor,
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
                            "limit" => Ok(GeneratedField::Limit),
                            "optionalStartingAfterCursor" | "optional_starting_after_cursor" => Ok(GeneratedField::OptionalStartingAfterCursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupPermissionSetsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.LookupPermissionSetsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LookupPermissionSetsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limit__ = None;
                let mut optional_starting_after_cursor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OptionalStartingAfterCursor => {
                            if optional_starting_after_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalStartingAfterCursor"));
                            }
                            optional_starting_after_cursor__ = map.next_value()?;
                        }
                    }
                }
                Ok(LookupPermissionSetsRequest {
                    limit: limit__.unwrap_or_default(),
                    optional_starting_after_cursor: optional_starting_after_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.LookupPermissionSetsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupPermissionSetsRequired {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.required_lookup_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.LookupPermissionSetsRequired", len)?;
        if let Some(v) = self.required_lookup_at.as_ref() {
            struct_ser.serialize_field("requiredLookupAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupPermissionSetsRequired {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "required_lookup_at",
            "requiredLookupAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequiredLookupAt,
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
                            "requiredLookupAt" | "required_lookup_at" => Ok(GeneratedField::RequiredLookupAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupPermissionSetsRequired;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.LookupPermissionSetsRequired")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LookupPermissionSetsRequired, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut required_lookup_at__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequiredLookupAt => {
                            if required_lookup_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiredLookupAt"));
                            }
                            required_lookup_at__ = map.next_value()?;
                        }
                    }
                }
                Ok(LookupPermissionSetsRequired {
                    required_lookup_at: required_lookup_at__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.LookupPermissionSetsRequired", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupPermissionSetsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.change.is_some() {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.LookupPermissionSetsResponse", len)?;
        if let Some(v) = self.change.as_ref() {
            struct_ser.serialize_field("change", v)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupPermissionSetsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "change",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Change,
            Cursor,
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
                            "change" => Ok(GeneratedField::Change),
                            "cursor" => Ok(GeneratedField::Cursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupPermissionSetsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.LookupPermissionSetsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LookupPermissionSetsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut change__ = None;
                let mut cursor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Change => {
                            if change__.is_some() {
                                return Err(serde::de::Error::duplicate_field("change"));
                            }
                            change__ = map.next_value()?;
                        }
                        GeneratedField::Cursor => {
                            if cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            cursor__ = map.next_value()?;
                        }
                    }
                }
                Ok(LookupPermissionSetsResponse {
                    change: change__,
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.LookupPermissionSetsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MemberReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.object_type.is_empty() {
            len += 1;
        }
        if !self.object_id.is_empty() {
            len += 1;
        }
        if !self.optional_permission_or_relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.MemberReference", len)?;
        if !self.object_type.is_empty() {
            struct_ser.serialize_field("objectType", &self.object_type)?;
        }
        if !self.object_id.is_empty() {
            struct_ser.serialize_field("objectId", &self.object_id)?;
        }
        if !self.optional_permission_or_relation.is_empty() {
            struct_ser.serialize_field("optionalPermissionOrRelation", &self.optional_permission_or_relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MemberReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_type",
            "objectType",
            "object_id",
            "objectId",
            "optional_permission_or_relation",
            "optionalPermissionOrRelation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectType,
            ObjectId,
            OptionalPermissionOrRelation,
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
                            "objectType" | "object_type" => Ok(GeneratedField::ObjectType),
                            "objectId" | "object_id" => Ok(GeneratedField::ObjectId),
                            "optionalPermissionOrRelation" | "optional_permission_or_relation" => Ok(GeneratedField::OptionalPermissionOrRelation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MemberReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.MemberReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MemberReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_type__ = None;
                let mut object_id__ = None;
                let mut optional_permission_or_relation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ObjectType => {
                            if object_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectType"));
                            }
                            object_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::ObjectId => {
                            if object_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectId"));
                            }
                            object_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalPermissionOrRelation => {
                            if optional_permission_or_relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalPermissionOrRelation"));
                            }
                            optional_permission_or_relation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MemberReference {
                    object_type: object_type__.unwrap_or_default(),
                    object_id: object_id__.unwrap_or_default(),
                    optional_permission_or_relation: optional_permission_or_relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.MemberReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionChange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.revision.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if !self.permission.is_empty() {
            len += 1;
        }
        if self.subject.is_some() {
            len += 1;
        }
        if self.permissionship != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.PermissionChange", len)?;
        if let Some(v) = self.revision.as_ref() {
            struct_ser.serialize_field("revision", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.permission.is_empty() {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if self.permissionship != 0 {
            let v = permission_change::Permissionship::from_i32(self.permissionship)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.permissionship)))?;
            struct_ser.serialize_field("permissionship", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionChange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "revision",
            "resource",
            "permission",
            "subject",
            "permissionship",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Revision,
            Resource,
            Permission,
            Subject,
            Permissionship,
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
                            "revision" => Ok(GeneratedField::Revision),
                            "resource" => Ok(GeneratedField::Resource),
                            "permission" => Ok(GeneratedField::Permission),
                            "subject" => Ok(GeneratedField::Subject),
                            "permissionship" => Ok(GeneratedField::Permissionship),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionChange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.PermissionChange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PermissionChange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut revision__ = None;
                let mut resource__ = None;
                let mut permission__ = None;
                let mut subject__ = None;
                let mut permissionship__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Revision => {
                            if revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("revision"));
                            }
                            revision__ = map.next_value()?;
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map.next_value()?;
                        }
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = Some(map.next_value()?);
                        }
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map.next_value()?;
                        }
                        GeneratedField::Permissionship => {
                            if permissionship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionship"));
                            }
                            permissionship__ = Some(map.next_value::<permission_change::Permissionship>()? as i32);
                        }
                    }
                }
                Ok(PermissionChange {
                    revision: revision__,
                    resource: resource__,
                    permission: permission__.unwrap_or_default(),
                    subject: subject__,
                    permissionship: permissionship__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.PermissionChange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for permission_change::Permissionship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PERMISSIONSHIP_UNSPECIFIED",
            Self::NoPermission => "PERMISSIONSHIP_NO_PERMISSION",
            Self::HasPermission => "PERMISSIONSHIP_HAS_PERMISSION",
            Self::ConditionalPermission => "PERMISSIONSHIP_CONDITIONAL_PERMISSION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for permission_change::Permissionship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PERMISSIONSHIP_UNSPECIFIED",
            "PERMISSIONSHIP_NO_PERMISSION",
            "PERMISSIONSHIP_HAS_PERMISSION",
            "PERMISSIONSHIP_CONDITIONAL_PERMISSION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = permission_change::Permissionship;

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
                    .and_then(permission_change::Permissionship::from_i32)
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
                    .and_then(permission_change::Permissionship::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PERMISSIONSHIP_UNSPECIFIED" => Ok(permission_change::Permissionship::Unspecified),
                    "PERMISSIONSHIP_NO_PERMISSION" => Ok(permission_change::Permissionship::NoPermission),
                    "PERMISSIONSHIP_HAS_PERMISSION" => Ok(permission_change::Permissionship::HasPermission),
                    "PERMISSIONSHIP_CONDITIONAL_PERMISSION" => Ok(permission_change::Permissionship::ConditionalPermission),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionSetChange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.at_revision.is_some() {
            len += 1;
        }
        if self.operation != 0 {
            len += 1;
        }
        if self.parent_set.is_some() {
            len += 1;
        }
        if self.child.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.PermissionSetChange", len)?;
        if let Some(v) = self.at_revision.as_ref() {
            struct_ser.serialize_field("atRevision", v)?;
        }
        if self.operation != 0 {
            let v = permission_set_change::SetOperation::from_i32(self.operation)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.operation)))?;
            struct_ser.serialize_field("operation", &v)?;
        }
        if let Some(v) = self.parent_set.as_ref() {
            struct_ser.serialize_field("parentSet", v)?;
        }
        if let Some(v) = self.child.as_ref() {
            match v {
                permission_set_change::Child::ChildSet(v) => {
                    struct_ser.serialize_field("childSet", v)?;
                }
                permission_set_change::Child::ChildMember(v) => {
                    struct_ser.serialize_field("childMember", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionSetChange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "at_revision",
            "atRevision",
            "operation",
            "parent_set",
            "parentSet",
            "child_set",
            "childSet",
            "child_member",
            "childMember",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AtRevision,
            Operation,
            ParentSet,
            ChildSet,
            ChildMember,
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
                            "atRevision" | "at_revision" => Ok(GeneratedField::AtRevision),
                            "operation" => Ok(GeneratedField::Operation),
                            "parentSet" | "parent_set" => Ok(GeneratedField::ParentSet),
                            "childSet" | "child_set" => Ok(GeneratedField::ChildSet),
                            "childMember" | "child_member" => Ok(GeneratedField::ChildMember),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionSetChange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.PermissionSetChange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PermissionSetChange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut at_revision__ = None;
                let mut operation__ = None;
                let mut parent_set__ = None;
                let mut child__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AtRevision => {
                            if at_revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("atRevision"));
                            }
                            at_revision__ = map.next_value()?;
                        }
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = Some(map.next_value::<permission_set_change::SetOperation>()? as i32);
                        }
                        GeneratedField::ParentSet => {
                            if parent_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentSet"));
                            }
                            parent_set__ = map.next_value()?;
                        }
                        GeneratedField::ChildSet => {
                            if child__.is_some() {
                                return Err(serde::de::Error::duplicate_field("childSet"));
                            }
                            child__ = map.next_value::<::std::option::Option<_>>()?.map(permission_set_change::Child::ChildSet)
;
                        }
                        GeneratedField::ChildMember => {
                            if child__.is_some() {
                                return Err(serde::de::Error::duplicate_field("childMember"));
                            }
                            child__ = map.next_value::<::std::option::Option<_>>()?.map(permission_set_change::Child::ChildMember)
;
                        }
                    }
                }
                Ok(PermissionSetChange {
                    at_revision: at_revision__,
                    operation: operation__.unwrap_or_default(),
                    parent_set: parent_set__,
                    child: child__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.PermissionSetChange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for permission_set_change::SetOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SET_OPERATION_UNSPECIFIED",
            Self::Added => "SET_OPERATION_ADDED",
            Self::Removed => "SET_OPERATION_REMOVED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for permission_set_change::SetOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SET_OPERATION_UNSPECIFIED",
            "SET_OPERATION_ADDED",
            "SET_OPERATION_REMOVED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = permission_set_change::SetOperation;

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
                    .and_then(permission_set_change::SetOperation::from_i32)
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
                    .and_then(permission_set_change::SetOperation::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SET_OPERATION_UNSPECIFIED" => Ok(permission_set_change::SetOperation::Unspecified),
                    "SET_OPERATION_ADDED" => Ok(permission_set_change::SetOperation::Added),
                    "SET_OPERATION_REMOVED" => Ok(permission_set_change::SetOperation::Removed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SetReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.object_type.is_empty() {
            len += 1;
        }
        if !self.object_id.is_empty() {
            len += 1;
        }
        if !self.permission_or_relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.SetReference", len)?;
        if !self.object_type.is_empty() {
            struct_ser.serialize_field("objectType", &self.object_type)?;
        }
        if !self.object_id.is_empty() {
            struct_ser.serialize_field("objectId", &self.object_id)?;
        }
        if !self.permission_or_relation.is_empty() {
            struct_ser.serialize_field("permissionOrRelation", &self.permission_or_relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_type",
            "objectType",
            "object_id",
            "objectId",
            "permission_or_relation",
            "permissionOrRelation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectType,
            ObjectId,
            PermissionOrRelation,
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
                            "objectType" | "object_type" => Ok(GeneratedField::ObjectType),
                            "objectId" | "object_id" => Ok(GeneratedField::ObjectId),
                            "permissionOrRelation" | "permission_or_relation" => Ok(GeneratedField::PermissionOrRelation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.SetReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_type__ = None;
                let mut object_id__ = None;
                let mut permission_or_relation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ObjectType => {
                            if object_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectType"));
                            }
                            object_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::ObjectId => {
                            if object_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectId"));
                            }
                            object_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::PermissionOrRelation => {
                            if permission_or_relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionOrRelation"));
                            }
                            permission_or_relation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SetReference {
                    object_type: object_type__.unwrap_or_default(),
                    object_id: object_id__.unwrap_or_default(),
                    permission_or_relation: permission_or_relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.SetReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchPermissionSetsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.optional_starting_after.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.WatchPermissionSetsRequest", len)?;
        if let Some(v) = self.optional_starting_after.as_ref() {
            struct_ser.serialize_field("optionalStartingAfter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchPermissionSetsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "optional_starting_after",
            "optionalStartingAfter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OptionalStartingAfter,
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
                            "optionalStartingAfter" | "optional_starting_after" => Ok(GeneratedField::OptionalStartingAfter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchPermissionSetsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.WatchPermissionSetsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchPermissionSetsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut optional_starting_after__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OptionalStartingAfter => {
                            if optional_starting_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalStartingAfter"));
                            }
                            optional_starting_after__ = map.next_value()?;
                        }
                    }
                }
                Ok(WatchPermissionSetsRequest {
                    optional_starting_after: optional_starting_after__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.WatchPermissionSetsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchPermissionSetsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.WatchPermissionSetsResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                watch_permission_sets_response::Response::Change(v) => {
                    struct_ser.serialize_field("change", v)?;
                }
                watch_permission_sets_response::Response::CompletedRevision(v) => {
                    struct_ser.serialize_field("completedRevision", v)?;
                }
                watch_permission_sets_response::Response::LookupPermissionSetsRequired(v) => {
                    struct_ser.serialize_field("lookupPermissionSetsRequired", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchPermissionSetsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "change",
            "completed_revision",
            "completedRevision",
            "lookup_permission_sets_required",
            "lookupPermissionSetsRequired",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Change,
            CompletedRevision,
            LookupPermissionSetsRequired,
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
                            "change" => Ok(GeneratedField::Change),
                            "completedRevision" | "completed_revision" => Ok(GeneratedField::CompletedRevision),
                            "lookupPermissionSetsRequired" | "lookup_permission_sets_required" => Ok(GeneratedField::LookupPermissionSetsRequired),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchPermissionSetsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.WatchPermissionSetsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchPermissionSetsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Change => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("change"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(watch_permission_sets_response::Response::Change)
;
                        }
                        GeneratedField::CompletedRevision => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completedRevision"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(watch_permission_sets_response::Response::CompletedRevision)
;
                        }
                        GeneratedField::LookupPermissionSetsRequired => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lookupPermissionSetsRequired"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(watch_permission_sets_response::Response::LookupPermissionSetsRequired)
;
                        }
                    }
                }
                Ok(WatchPermissionSetsResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.WatchPermissionSetsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchPermissionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.permissions.is_empty() {
            len += 1;
        }
        if self.optional_starting_after.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.WatchPermissionsRequest", len)?;
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        if let Some(v) = self.optional_starting_after.as_ref() {
            struct_ser.serialize_field("optionalStartingAfter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchPermissionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permissions",
            "optional_starting_after",
            "optionalStartingAfter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permissions,
            OptionalStartingAfter,
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
                            "permissions" => Ok(GeneratedField::Permissions),
                            "optionalStartingAfter" | "optional_starting_after" => Ok(GeneratedField::OptionalStartingAfter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchPermissionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.WatchPermissionsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchPermissionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permissions__ = None;
                let mut optional_starting_after__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalStartingAfter => {
                            if optional_starting_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalStartingAfter"));
                            }
                            optional_starting_after__ = map.next_value()?;
                        }
                    }
                }
                Ok(WatchPermissionsRequest {
                    permissions: permissions__.unwrap_or_default(),
                    optional_starting_after: optional_starting_after__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.WatchPermissionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchPermissionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.WatchPermissionsResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                watch_permissions_response::Response::Change(v) => {
                    struct_ser.serialize_field("change", v)?;
                }
                watch_permissions_response::Response::CompletedRevision(v) => {
                    struct_ser.serialize_field("completedRevision", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchPermissionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "change",
            "completed_revision",
            "completedRevision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Change,
            CompletedRevision,
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
                            "change" => Ok(GeneratedField::Change),
                            "completedRevision" | "completed_revision" => Ok(GeneratedField::CompletedRevision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchPermissionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.WatchPermissionsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchPermissionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Change => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("change"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(watch_permissions_response::Response::Change)
;
                        }
                        GeneratedField::CompletedRevision => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completedRevision"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(watch_permissions_response::Response::CompletedRevision)
;
                        }
                    }
                }
                Ok(WatchPermissionsResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.WatchPermissionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchedPermission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_type.is_empty() {
            len += 1;
        }
        if !self.permission.is_empty() {
            len += 1;
        }
        if !self.subject_type.is_empty() {
            len += 1;
        }
        if !self.optional_subject_relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.materialize.v0.WatchedPermission", len)?;
        if !self.resource_type.is_empty() {
            struct_ser.serialize_field("resourceType", &self.resource_type)?;
        }
        if !self.permission.is_empty() {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        if !self.subject_type.is_empty() {
            struct_ser.serialize_field("subjectType", &self.subject_type)?;
        }
        if !self.optional_subject_relation.is_empty() {
            struct_ser.serialize_field("optionalSubjectRelation", &self.optional_subject_relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchedPermission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_type",
            "resourceType",
            "permission",
            "subject_type",
            "subjectType",
            "optional_subject_relation",
            "optionalSubjectRelation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceType,
            Permission,
            SubjectType,
            OptionalSubjectRelation,
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
                            "resourceType" | "resource_type" => Ok(GeneratedField::ResourceType),
                            "permission" => Ok(GeneratedField::Permission),
                            "subjectType" | "subject_type" => Ok(GeneratedField::SubjectType),
                            "optionalSubjectRelation" | "optional_subject_relation" => Ok(GeneratedField::OptionalSubjectRelation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchedPermission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.materialize.v0.WatchedPermission")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchedPermission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_type__ = None;
                let mut permission__ = None;
                let mut subject_type__ = None;
                let mut optional_subject_relation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceType"));
                            }
                            resource_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubjectType => {
                            if subject_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectType"));
                            }
                            subject_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalSubjectRelation => {
                            if optional_subject_relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalSubjectRelation"));
                            }
                            optional_subject_relation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WatchedPermission {
                    resource_type: resource_type__.unwrap_or_default(),
                    permission: permission__.unwrap_or_default(),
                    subject_type: subject_type__.unwrap_or_default(),
                    optional_subject_relation: optional_subject_relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.materialize.v0.WatchedPermission", FIELDS, GeneratedVisitor)
    }
}
