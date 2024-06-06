// @generated
impl serde::Serialize for AlgebraicSubjectSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation != 0 {
            len += 1;
        }
        if !self.children.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.AlgebraicSubjectSet", len)?;
        if self.operation != 0 {
            let v = algebraic_subject_set::Operation::from_i32(self.operation)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.operation)))?;
            struct_ser.serialize_field("operation", &v)?;
        }
        if !self.children.is_empty() {
            struct_ser.serialize_field("children", &self.children)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AlgebraicSubjectSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation",
            "children",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operation,
            Children,
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
                            "operation" => Ok(GeneratedField::Operation),
                            "children" => Ok(GeneratedField::Children),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AlgebraicSubjectSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.AlgebraicSubjectSet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AlgebraicSubjectSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation__ = None;
                let mut children__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = Some(map.next_value::<algebraic_subject_set::Operation>()? as i32);
                        }
                        GeneratedField::Children => {
                            if children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("children"));
                            }
                            children__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AlgebraicSubjectSet {
                    operation: operation__.unwrap_or_default(),
                    children: children__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.AlgebraicSubjectSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for algebraic_subject_set::Operation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OPERATION_UNSPECIFIED",
            Self::Union => "OPERATION_UNION",
            Self::Intersection => "OPERATION_INTERSECTION",
            Self::Exclusion => "OPERATION_EXCLUSION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for algebraic_subject_set::Operation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OPERATION_UNSPECIFIED",
            "OPERATION_UNION",
            "OPERATION_INTERSECTION",
            "OPERATION_EXCLUSION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = algebraic_subject_set::Operation;

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
                    .and_then(algebraic_subject_set::Operation::from_i32)
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
                    .and_then(algebraic_subject_set::Operation::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OPERATION_UNSPECIFIED" => Ok(algebraic_subject_set::Operation::Unspecified),
                    "OPERATION_UNION" => Ok(algebraic_subject_set::Operation::Union),
                    "OPERATION_INTERSECTION" => Ok(algebraic_subject_set::Operation::Intersection),
                    "OPERATION_EXCLUSION" => Ok(algebraic_subject_set::Operation::Exclusion),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for BulkCheckPermissionPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.BulkCheckPermissionPair", len)?;
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if let Some(v) = self.response.as_ref() {
            match v {
                bulk_check_permission_pair::Response::Item(v) => {
                    struct_ser.serialize_field("item", v)?;
                }
                bulk_check_permission_pair::Response::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkCheckPermissionPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
            "item",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
            Item,
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
                            "request" => Ok(GeneratedField::Request),
                            "item" => Ok(GeneratedField::Item),
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
            type Value = BulkCheckPermissionPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.BulkCheckPermissionPair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BulkCheckPermissionPair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map.next_value()?;
                        }
                        GeneratedField::Item => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(bulk_check_permission_pair::Response::Item)
;
                        }
                        GeneratedField::Error => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(bulk_check_permission_pair::Response::Error)
;
                        }
                    }
                }
                Ok(BulkCheckPermissionPair {
                    request: request__,
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.BulkCheckPermissionPair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BulkCheckPermissionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if !self.items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.BulkCheckPermissionRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkCheckPermissionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            Items,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "items" => Ok(GeneratedField::Items),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BulkCheckPermissionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.BulkCheckPermissionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BulkCheckPermissionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut items__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BulkCheckPermissionRequest {
                    consistency: consistency__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.BulkCheckPermissionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BulkCheckPermissionRequestItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        if !self.permission.is_empty() {
            len += 1;
        }
        if self.subject.is_some() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.BulkCheckPermissionRequestItem", len)?;
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.permission.is_empty() {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkCheckPermissionRequestItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource",
            "permission",
            "subject",
            "context",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resource,
            Permission,
            Subject,
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
                            "resource" => Ok(GeneratedField::Resource),
                            "permission" => Ok(GeneratedField::Permission),
                            "subject" => Ok(GeneratedField::Subject),
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
            type Value = BulkCheckPermissionRequestItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.BulkCheckPermissionRequestItem")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BulkCheckPermissionRequestItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                let mut permission__ = None;
                let mut subject__ = None;
                let mut context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                    }
                }
                Ok(BulkCheckPermissionRequestItem {
                    resource: resource__,
                    permission: permission__.unwrap_or_default(),
                    subject: subject__,
                    context: context__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.BulkCheckPermissionRequestItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BulkCheckPermissionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.checked_at.is_some() {
            len += 1;
        }
        if !self.pairs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.BulkCheckPermissionResponse", len)?;
        if let Some(v) = self.checked_at.as_ref() {
            struct_ser.serialize_field("checkedAt", v)?;
        }
        if !self.pairs.is_empty() {
            struct_ser.serialize_field("pairs", &self.pairs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkCheckPermissionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "checked_at",
            "checkedAt",
            "pairs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CheckedAt,
            Pairs,
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
                            "checkedAt" | "checked_at" => Ok(GeneratedField::CheckedAt),
                            "pairs" => Ok(GeneratedField::Pairs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BulkCheckPermissionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.BulkCheckPermissionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BulkCheckPermissionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut checked_at__ = None;
                let mut pairs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CheckedAt => {
                            if checked_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkedAt"));
                            }
                            checked_at__ = map.next_value()?;
                        }
                        GeneratedField::Pairs => {
                            if pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pairs"));
                            }
                            pairs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BulkCheckPermissionResponse {
                    checked_at: checked_at__,
                    pairs: pairs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.BulkCheckPermissionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BulkCheckPermissionResponseItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.permissionship != 0 {
            len += 1;
        }
        if self.partial_caveat_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.BulkCheckPermissionResponseItem", len)?;
        if self.permissionship != 0 {
            let v = check_permission_response::Permissionship::from_i32(self.permissionship)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.permissionship)))?;
            struct_ser.serialize_field("permissionship", &v)?;
        }
        if let Some(v) = self.partial_caveat_info.as_ref() {
            struct_ser.serialize_field("partialCaveatInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkCheckPermissionResponseItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permissionship",
            "partial_caveat_info",
            "partialCaveatInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permissionship,
            PartialCaveatInfo,
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
                            "permissionship" => Ok(GeneratedField::Permissionship),
                            "partialCaveatInfo" | "partial_caveat_info" => Ok(GeneratedField::PartialCaveatInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BulkCheckPermissionResponseItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.BulkCheckPermissionResponseItem")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BulkCheckPermissionResponseItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permissionship__ = None;
                let mut partial_caveat_info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Permissionship => {
                            if permissionship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionship"));
                            }
                            permissionship__ = Some(map.next_value::<check_permission_response::Permissionship>()? as i32);
                        }
                        GeneratedField::PartialCaveatInfo => {
                            if partial_caveat_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partialCaveatInfo"));
                            }
                            partial_caveat_info__ = map.next_value()?;
                        }
                    }
                }
                Ok(BulkCheckPermissionResponseItem {
                    permissionship: permissionship__.unwrap_or_default(),
                    partial_caveat_info: partial_caveat_info__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.BulkCheckPermissionResponseItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BulkExportRelationshipsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if self.optional_limit != 0 {
            len += 1;
        }
        if self.optional_cursor.is_some() {
            len += 1;
        }
        if self.optional_relationship_filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.BulkExportRelationshipsRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if self.optional_limit != 0 {
            struct_ser.serialize_field("optionalLimit", &self.optional_limit)?;
        }
        if let Some(v) = self.optional_cursor.as_ref() {
            struct_ser.serialize_field("optionalCursor", v)?;
        }
        if let Some(v) = self.optional_relationship_filter.as_ref() {
            struct_ser.serialize_field("optionalRelationshipFilter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkExportRelationshipsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "optional_limit",
            "optionalLimit",
            "optional_cursor",
            "optionalCursor",
            "optional_relationship_filter",
            "optionalRelationshipFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            OptionalLimit,
            OptionalCursor,
            OptionalRelationshipFilter,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "optionalLimit" | "optional_limit" => Ok(GeneratedField::OptionalLimit),
                            "optionalCursor" | "optional_cursor" => Ok(GeneratedField::OptionalCursor),
                            "optionalRelationshipFilter" | "optional_relationship_filter" => Ok(GeneratedField::OptionalRelationshipFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BulkExportRelationshipsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.BulkExportRelationshipsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BulkExportRelationshipsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut optional_limit__ = None;
                let mut optional_cursor__ = None;
                let mut optional_relationship_filter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
                        }
                        GeneratedField::OptionalLimit => {
                            if optional_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalLimit"));
                            }
                            optional_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OptionalCursor => {
                            if optional_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalCursor"));
                            }
                            optional_cursor__ = map.next_value()?;
                        }
                        GeneratedField::OptionalRelationshipFilter => {
                            if optional_relationship_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalRelationshipFilter"));
                            }
                            optional_relationship_filter__ = map.next_value()?;
                        }
                    }
                }
                Ok(BulkExportRelationshipsRequest {
                    consistency: consistency__,
                    optional_limit: optional_limit__.unwrap_or_default(),
                    optional_cursor: optional_cursor__,
                    optional_relationship_filter: optional_relationship_filter__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.BulkExportRelationshipsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BulkExportRelationshipsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.after_result_cursor.is_some() {
            len += 1;
        }
        if !self.relationships.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.BulkExportRelationshipsResponse", len)?;
        if let Some(v) = self.after_result_cursor.as_ref() {
            struct_ser.serialize_field("afterResultCursor", v)?;
        }
        if !self.relationships.is_empty() {
            struct_ser.serialize_field("relationships", &self.relationships)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkExportRelationshipsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "after_result_cursor",
            "afterResultCursor",
            "relationships",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AfterResultCursor,
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
                            "afterResultCursor" | "after_result_cursor" => Ok(GeneratedField::AfterResultCursor),
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
            type Value = BulkExportRelationshipsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.BulkExportRelationshipsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BulkExportRelationshipsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut after_result_cursor__ = None;
                let mut relationships__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AfterResultCursor => {
                            if after_result_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("afterResultCursor"));
                            }
                            after_result_cursor__ = map.next_value()?;
                        }
                        GeneratedField::Relationships => {
                            if relationships__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationships"));
                            }
                            relationships__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BulkExportRelationshipsResponse {
                    after_result_cursor: after_result_cursor__,
                    relationships: relationships__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.BulkExportRelationshipsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BulkImportRelationshipsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relationships.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.BulkImportRelationshipsRequest", len)?;
        if !self.relationships.is_empty() {
            struct_ser.serialize_field("relationships", &self.relationships)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkImportRelationshipsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relationships",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = BulkImportRelationshipsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.BulkImportRelationshipsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BulkImportRelationshipsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relationships__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Relationships => {
                            if relationships__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationships"));
                            }
                            relationships__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BulkImportRelationshipsRequest {
                    relationships: relationships__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.BulkImportRelationshipsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BulkImportRelationshipsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.num_loaded != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.BulkImportRelationshipsResponse", len)?;
        if self.num_loaded != 0 {
            struct_ser.serialize_field("numLoaded", ToString::to_string(&self.num_loaded).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkImportRelationshipsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "num_loaded",
            "numLoaded",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NumLoaded,
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
                            "numLoaded" | "num_loaded" => Ok(GeneratedField::NumLoaded),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BulkImportRelationshipsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.BulkImportRelationshipsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BulkImportRelationshipsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut num_loaded__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NumLoaded => {
                            if num_loaded__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numLoaded"));
                            }
                            num_loaded__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BulkImportRelationshipsResponse {
                    num_loaded: num_loaded__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.BulkImportRelationshipsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CaveatEvalInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.expression.is_empty() {
            len += 1;
        }
        if self.result != 0 {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        if self.partial_caveat_info.is_some() {
            len += 1;
        }
        if !self.caveat_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.CaveatEvalInfo", len)?;
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        if self.result != 0 {
            let v = caveat_eval_info::Result::from_i32(self.result)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.result)))?;
            struct_ser.serialize_field("result", &v)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if let Some(v) = self.partial_caveat_info.as_ref() {
            struct_ser.serialize_field("partialCaveatInfo", v)?;
        }
        if !self.caveat_name.is_empty() {
            struct_ser.serialize_field("caveatName", &self.caveat_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CaveatEvalInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expression",
            "result",
            "context",
            "partial_caveat_info",
            "partialCaveatInfo",
            "caveat_name",
            "caveatName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expression,
            Result,
            Context,
            PartialCaveatInfo,
            CaveatName,
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
                            "expression" => Ok(GeneratedField::Expression),
                            "result" => Ok(GeneratedField::Result),
                            "context" => Ok(GeneratedField::Context),
                            "partialCaveatInfo" | "partial_caveat_info" => Ok(GeneratedField::PartialCaveatInfo),
                            "caveatName" | "caveat_name" => Ok(GeneratedField::CaveatName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CaveatEvalInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.CaveatEvalInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CaveatEvalInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expression__ = None;
                let mut result__ = None;
                let mut context__ = None;
                let mut partial_caveat_info__ = None;
                let mut caveat_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map.next_value()?);
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map.next_value::<caveat_eval_info::Result>()? as i32);
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                        GeneratedField::PartialCaveatInfo => {
                            if partial_caveat_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partialCaveatInfo"));
                            }
                            partial_caveat_info__ = map.next_value()?;
                        }
                        GeneratedField::CaveatName => {
                            if caveat_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveatName"));
                            }
                            caveat_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CaveatEvalInfo {
                    expression: expression__.unwrap_or_default(),
                    result: result__.unwrap_or_default(),
                    context: context__,
                    partial_caveat_info: partial_caveat_info__,
                    caveat_name: caveat_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.CaveatEvalInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for caveat_eval_info::Result {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RESULT_UNSPECIFIED",
            Self::Unevaluated => "RESULT_UNEVALUATED",
            Self::False => "RESULT_FALSE",
            Self::True => "RESULT_TRUE",
            Self::MissingSomeContext => "RESULT_MISSING_SOME_CONTEXT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for caveat_eval_info::Result {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RESULT_UNSPECIFIED",
            "RESULT_UNEVALUATED",
            "RESULT_FALSE",
            "RESULT_TRUE",
            "RESULT_MISSING_SOME_CONTEXT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = caveat_eval_info::Result;

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
                    .and_then(caveat_eval_info::Result::from_i32)
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
                    .and_then(caveat_eval_info::Result::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "RESULT_UNSPECIFIED" => Ok(caveat_eval_info::Result::Unspecified),
                    "RESULT_UNEVALUATED" => Ok(caveat_eval_info::Result::Unevaluated),
                    "RESULT_FALSE" => Ok(caveat_eval_info::Result::False),
                    "RESULT_TRUE" => Ok(caveat_eval_info::Result::True),
                    "RESULT_MISSING_SOME_CONTEXT" => Ok(caveat_eval_info::Result::MissingSomeContext),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CheckBulkPermissionsPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.CheckBulkPermissionsPair", len)?;
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if let Some(v) = self.response.as_ref() {
            match v {
                check_bulk_permissions_pair::Response::Item(v) => {
                    struct_ser.serialize_field("item", v)?;
                }
                check_bulk_permissions_pair::Response::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckBulkPermissionsPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
            "item",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
            Item,
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
                            "request" => Ok(GeneratedField::Request),
                            "item" => Ok(GeneratedField::Item),
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
            type Value = CheckBulkPermissionsPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.CheckBulkPermissionsPair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckBulkPermissionsPair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map.next_value()?;
                        }
                        GeneratedField::Item => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(check_bulk_permissions_pair::Response::Item)
;
                        }
                        GeneratedField::Error => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(check_bulk_permissions_pair::Response::Error)
;
                        }
                    }
                }
                Ok(CheckBulkPermissionsPair {
                    request: request__,
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.CheckBulkPermissionsPair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckBulkPermissionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if !self.items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.CheckBulkPermissionsRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckBulkPermissionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            Items,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "items" => Ok(GeneratedField::Items),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckBulkPermissionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.CheckBulkPermissionsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckBulkPermissionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut items__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CheckBulkPermissionsRequest {
                    consistency: consistency__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.CheckBulkPermissionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckBulkPermissionsRequestItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        if !self.permission.is_empty() {
            len += 1;
        }
        if self.subject.is_some() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.CheckBulkPermissionsRequestItem", len)?;
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.permission.is_empty() {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckBulkPermissionsRequestItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource",
            "permission",
            "subject",
            "context",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resource,
            Permission,
            Subject,
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
                            "resource" => Ok(GeneratedField::Resource),
                            "permission" => Ok(GeneratedField::Permission),
                            "subject" => Ok(GeneratedField::Subject),
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
            type Value = CheckBulkPermissionsRequestItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.CheckBulkPermissionsRequestItem")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckBulkPermissionsRequestItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                let mut permission__ = None;
                let mut subject__ = None;
                let mut context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                    }
                }
                Ok(CheckBulkPermissionsRequestItem {
                    resource: resource__,
                    permission: permission__.unwrap_or_default(),
                    subject: subject__,
                    context: context__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.CheckBulkPermissionsRequestItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckBulkPermissionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.checked_at.is_some() {
            len += 1;
        }
        if !self.pairs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.CheckBulkPermissionsResponse", len)?;
        if let Some(v) = self.checked_at.as_ref() {
            struct_ser.serialize_field("checkedAt", v)?;
        }
        if !self.pairs.is_empty() {
            struct_ser.serialize_field("pairs", &self.pairs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckBulkPermissionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "checked_at",
            "checkedAt",
            "pairs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CheckedAt,
            Pairs,
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
                            "checkedAt" | "checked_at" => Ok(GeneratedField::CheckedAt),
                            "pairs" => Ok(GeneratedField::Pairs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckBulkPermissionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.CheckBulkPermissionsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckBulkPermissionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut checked_at__ = None;
                let mut pairs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CheckedAt => {
                            if checked_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkedAt"));
                            }
                            checked_at__ = map.next_value()?;
                        }
                        GeneratedField::Pairs => {
                            if pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pairs"));
                            }
                            pairs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CheckBulkPermissionsResponse {
                    checked_at: checked_at__,
                    pairs: pairs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.CheckBulkPermissionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckBulkPermissionsResponseItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.permissionship != 0 {
            len += 1;
        }
        if self.partial_caveat_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.CheckBulkPermissionsResponseItem", len)?;
        if self.permissionship != 0 {
            let v = check_permission_response::Permissionship::from_i32(self.permissionship)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.permissionship)))?;
            struct_ser.serialize_field("permissionship", &v)?;
        }
        if let Some(v) = self.partial_caveat_info.as_ref() {
            struct_ser.serialize_field("partialCaveatInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckBulkPermissionsResponseItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permissionship",
            "partial_caveat_info",
            "partialCaveatInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permissionship,
            PartialCaveatInfo,
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
                            "permissionship" => Ok(GeneratedField::Permissionship),
                            "partialCaveatInfo" | "partial_caveat_info" => Ok(GeneratedField::PartialCaveatInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckBulkPermissionsResponseItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.CheckBulkPermissionsResponseItem")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckBulkPermissionsResponseItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permissionship__ = None;
                let mut partial_caveat_info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Permissionship => {
                            if permissionship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionship"));
                            }
                            permissionship__ = Some(map.next_value::<check_permission_response::Permissionship>()? as i32);
                        }
                        GeneratedField::PartialCaveatInfo => {
                            if partial_caveat_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partialCaveatInfo"));
                            }
                            partial_caveat_info__ = map.next_value()?;
                        }
                    }
                }
                Ok(CheckBulkPermissionsResponseItem {
                    permissionship: permissionship__.unwrap_or_default(),
                    partial_caveat_info: partial_caveat_info__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.CheckBulkPermissionsResponseItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckDebugTrace {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        if !self.permission.is_empty() {
            len += 1;
        }
        if self.permission_type != 0 {
            len += 1;
        }
        if self.subject.is_some() {
            len += 1;
        }
        if self.result != 0 {
            len += 1;
        }
        if self.caveat_evaluation_info.is_some() {
            len += 1;
        }
        if self.duration.is_some() {
            len += 1;
        }
        if self.resolution.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.CheckDebugTrace", len)?;
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.permission.is_empty() {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        if self.permission_type != 0 {
            let v = check_debug_trace::PermissionType::from_i32(self.permission_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.permission_type)))?;
            struct_ser.serialize_field("permissionType", &v)?;
        }
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if self.result != 0 {
            let v = check_debug_trace::Permissionship::from_i32(self.result)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.result)))?;
            struct_ser.serialize_field("result", &v)?;
        }
        if let Some(v) = self.caveat_evaluation_info.as_ref() {
            struct_ser.serialize_field("caveatEvaluationInfo", v)?;
        }
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", v)?;
        }
        if let Some(v) = self.resolution.as_ref() {
            match v {
                check_debug_trace::Resolution::WasCachedResult(v) => {
                    struct_ser.serialize_field("wasCachedResult", v)?;
                }
                check_debug_trace::Resolution::SubProblems(v) => {
                    struct_ser.serialize_field("subProblems", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckDebugTrace {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource",
            "permission",
            "permission_type",
            "permissionType",
            "subject",
            "result",
            "caveat_evaluation_info",
            "caveatEvaluationInfo",
            "duration",
            "was_cached_result",
            "wasCachedResult",
            "sub_problems",
            "subProblems",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resource,
            Permission,
            PermissionType,
            Subject,
            Result,
            CaveatEvaluationInfo,
            Duration,
            WasCachedResult,
            SubProblems,
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
                            "resource" => Ok(GeneratedField::Resource),
                            "permission" => Ok(GeneratedField::Permission),
                            "permissionType" | "permission_type" => Ok(GeneratedField::PermissionType),
                            "subject" => Ok(GeneratedField::Subject),
                            "result" => Ok(GeneratedField::Result),
                            "caveatEvaluationInfo" | "caveat_evaluation_info" => Ok(GeneratedField::CaveatEvaluationInfo),
                            "duration" => Ok(GeneratedField::Duration),
                            "wasCachedResult" | "was_cached_result" => Ok(GeneratedField::WasCachedResult),
                            "subProblems" | "sub_problems" => Ok(GeneratedField::SubProblems),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckDebugTrace;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.CheckDebugTrace")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckDebugTrace, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                let mut permission__ = None;
                let mut permission_type__ = None;
                let mut subject__ = None;
                let mut result__ = None;
                let mut caveat_evaluation_info__ = None;
                let mut duration__ = None;
                let mut resolution__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::PermissionType => {
                            if permission_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionType"));
                            }
                            permission_type__ = Some(map.next_value::<check_debug_trace::PermissionType>()? as i32);
                        }
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map.next_value::<check_debug_trace::Permissionship>()? as i32);
                        }
                        GeneratedField::CaveatEvaluationInfo => {
                            if caveat_evaluation_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveatEvaluationInfo"));
                            }
                            caveat_evaluation_info__ = map.next_value()?;
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = map.next_value()?;
                        }
                        GeneratedField::WasCachedResult => {
                            if resolution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wasCachedResult"));
                            }
                            resolution__ = map.next_value::<::std::option::Option<_>>()?.map(check_debug_trace::Resolution::WasCachedResult);
                        }
                        GeneratedField::SubProblems => {
                            if resolution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subProblems"));
                            }
                            resolution__ = map.next_value::<::std::option::Option<_>>()?.map(check_debug_trace::Resolution::SubProblems)
;
                        }
                    }
                }
                Ok(CheckDebugTrace {
                    resource: resource__,
                    permission: permission__.unwrap_or_default(),
                    permission_type: permission_type__.unwrap_or_default(),
                    subject: subject__,
                    result: result__.unwrap_or_default(),
                    caveat_evaluation_info: caveat_evaluation_info__,
                    duration: duration__,
                    resolution: resolution__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.CheckDebugTrace", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for check_debug_trace::PermissionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PERMISSION_TYPE_UNSPECIFIED",
            Self::Relation => "PERMISSION_TYPE_RELATION",
            Self::Permission => "PERMISSION_TYPE_PERMISSION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for check_debug_trace::PermissionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PERMISSION_TYPE_UNSPECIFIED",
            "PERMISSION_TYPE_RELATION",
            "PERMISSION_TYPE_PERMISSION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = check_debug_trace::PermissionType;

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
                    .and_then(check_debug_trace::PermissionType::from_i32)
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
                    .and_then(check_debug_trace::PermissionType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PERMISSION_TYPE_UNSPECIFIED" => Ok(check_debug_trace::PermissionType::Unspecified),
                    "PERMISSION_TYPE_RELATION" => Ok(check_debug_trace::PermissionType::Relation),
                    "PERMISSION_TYPE_PERMISSION" => Ok(check_debug_trace::PermissionType::Permission),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for check_debug_trace::Permissionship {
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
impl<'de> serde::Deserialize<'de> for check_debug_trace::Permissionship {
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
            type Value = check_debug_trace::Permissionship;

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
                    .and_then(check_debug_trace::Permissionship::from_i32)
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
                    .and_then(check_debug_trace::Permissionship::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PERMISSIONSHIP_UNSPECIFIED" => Ok(check_debug_trace::Permissionship::Unspecified),
                    "PERMISSIONSHIP_NO_PERMISSION" => Ok(check_debug_trace::Permissionship::NoPermission),
                    "PERMISSIONSHIP_HAS_PERMISSION" => Ok(check_debug_trace::Permissionship::HasPermission),
                    "PERMISSIONSHIP_CONDITIONAL_PERMISSION" => Ok(check_debug_trace::Permissionship::ConditionalPermission),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for check_debug_trace::SubProblems {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.traces.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.CheckDebugTrace.SubProblems", len)?;
        if !self.traces.is_empty() {
            struct_ser.serialize_field("traces", &self.traces)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for check_debug_trace::SubProblems {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "traces",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Traces,
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
                            "traces" => Ok(GeneratedField::Traces),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = check_debug_trace::SubProblems;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.CheckDebugTrace.SubProblems")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<check_debug_trace::SubProblems, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut traces__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Traces => {
                            if traces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traces"));
                            }
                            traces__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(check_debug_trace::SubProblems {
                    traces: traces__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.CheckDebugTrace.SubProblems", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckPermissionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
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
        if self.context.is_some() {
            len += 1;
        }
        if self.with_tracing {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.CheckPermissionRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
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
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if self.with_tracing {
            struct_ser.serialize_field("withTracing", &self.with_tracing)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckPermissionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "resource",
            "permission",
            "subject",
            "context",
            "with_tracing",
            "withTracing",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            Resource,
            Permission,
            Subject,
            Context,
            WithTracing,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "resource" => Ok(GeneratedField::Resource),
                            "permission" => Ok(GeneratedField::Permission),
                            "subject" => Ok(GeneratedField::Subject),
                            "context" => Ok(GeneratedField::Context),
                            "withTracing" | "with_tracing" => Ok(GeneratedField::WithTracing),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckPermissionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.CheckPermissionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckPermissionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut resource__ = None;
                let mut permission__ = None;
                let mut subject__ = None;
                let mut context__ = None;
                let mut with_tracing__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
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
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                        GeneratedField::WithTracing => {
                            if with_tracing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withTracing"));
                            }
                            with_tracing__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CheckPermissionRequest {
                    consistency: consistency__,
                    resource: resource__,
                    permission: permission__.unwrap_or_default(),
                    subject: subject__,
                    context: context__,
                    with_tracing: with_tracing__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.CheckPermissionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckPermissionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.checked_at.is_some() {
            len += 1;
        }
        if self.permissionship != 0 {
            len += 1;
        }
        if self.partial_caveat_info.is_some() {
            len += 1;
        }
        if self.debug_trace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.CheckPermissionResponse", len)?;
        if let Some(v) = self.checked_at.as_ref() {
            struct_ser.serialize_field("checkedAt", v)?;
        }
        if self.permissionship != 0 {
            let v = check_permission_response::Permissionship::from_i32(self.permissionship)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.permissionship)))?;
            struct_ser.serialize_field("permissionship", &v)?;
        }
        if let Some(v) = self.partial_caveat_info.as_ref() {
            struct_ser.serialize_field("partialCaveatInfo", v)?;
        }
        if let Some(v) = self.debug_trace.as_ref() {
            struct_ser.serialize_field("debugTrace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckPermissionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "checked_at",
            "checkedAt",
            "permissionship",
            "partial_caveat_info",
            "partialCaveatInfo",
            "debug_trace",
            "debugTrace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CheckedAt,
            Permissionship,
            PartialCaveatInfo,
            DebugTrace,
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
                            "checkedAt" | "checked_at" => Ok(GeneratedField::CheckedAt),
                            "permissionship" => Ok(GeneratedField::Permissionship),
                            "partialCaveatInfo" | "partial_caveat_info" => Ok(GeneratedField::PartialCaveatInfo),
                            "debugTrace" | "debug_trace" => Ok(GeneratedField::DebugTrace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckPermissionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.CheckPermissionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckPermissionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut checked_at__ = None;
                let mut permissionship__ = None;
                let mut partial_caveat_info__ = None;
                let mut debug_trace__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CheckedAt => {
                            if checked_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkedAt"));
                            }
                            checked_at__ = map.next_value()?;
                        }
                        GeneratedField::Permissionship => {
                            if permissionship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionship"));
                            }
                            permissionship__ = Some(map.next_value::<check_permission_response::Permissionship>()? as i32);
                        }
                        GeneratedField::PartialCaveatInfo => {
                            if partial_caveat_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partialCaveatInfo"));
                            }
                            partial_caveat_info__ = map.next_value()?;
                        }
                        GeneratedField::DebugTrace => {
                            if debug_trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debugTrace"));
                            }
                            debug_trace__ = map.next_value()?;
                        }
                    }
                }
                Ok(CheckPermissionResponse {
                    checked_at: checked_at__,
                    permissionship: permissionship__.unwrap_or_default(),
                    partial_caveat_info: partial_caveat_info__,
                    debug_trace: debug_trace__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.CheckPermissionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for check_permission_response::Permissionship {
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
impl<'de> serde::Deserialize<'de> for check_permission_response::Permissionship {
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
            type Value = check_permission_response::Permissionship;

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
                    .and_then(check_permission_response::Permissionship::from_i32)
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
                    .and_then(check_permission_response::Permissionship::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PERMISSIONSHIP_UNSPECIFIED" => Ok(check_permission_response::Permissionship::Unspecified),
                    "PERMISSIONSHIP_NO_PERMISSION" => Ok(check_permission_response::Permissionship::NoPermission),
                    "PERMISSIONSHIP_HAS_PERMISSION" => Ok(check_permission_response::Permissionship::HasPermission),
                    "PERMISSIONSHIP_CONDITIONAL_PERMISSION" => Ok(check_permission_response::Permissionship::ConditionalPermission),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Consistency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.requirement.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.Consistency", len)?;
        if let Some(v) = self.requirement.as_ref() {
            match v {
                consistency::Requirement::MinimizeLatency(v) => {
                    struct_ser.serialize_field("minimizeLatency", v)?;
                }
                consistency::Requirement::AtLeastAsFresh(v) => {
                    struct_ser.serialize_field("atLeastAsFresh", v)?;
                }
                consistency::Requirement::AtExactSnapshot(v) => {
                    struct_ser.serialize_field("atExactSnapshot", v)?;
                }
                consistency::Requirement::FullyConsistent(v) => {
                    struct_ser.serialize_field("fullyConsistent", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Consistency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "minimize_latency",
            "minimizeLatency",
            "at_least_as_fresh",
            "atLeastAsFresh",
            "at_exact_snapshot",
            "atExactSnapshot",
            "fully_consistent",
            "fullyConsistent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinimizeLatency,
            AtLeastAsFresh,
            AtExactSnapshot,
            FullyConsistent,
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
                            "minimizeLatency" | "minimize_latency" => Ok(GeneratedField::MinimizeLatency),
                            "atLeastAsFresh" | "at_least_as_fresh" => Ok(GeneratedField::AtLeastAsFresh),
                            "atExactSnapshot" | "at_exact_snapshot" => Ok(GeneratedField::AtExactSnapshot),
                            "fullyConsistent" | "fully_consistent" => Ok(GeneratedField::FullyConsistent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Consistency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.Consistency")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Consistency, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requirement__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MinimizeLatency => {
                            if requirement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimizeLatency"));
                            }
                            requirement__ = map.next_value::<::std::option::Option<_>>()?.map(consistency::Requirement::MinimizeLatency);
                        }
                        GeneratedField::AtLeastAsFresh => {
                            if requirement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("atLeastAsFresh"));
                            }
                            requirement__ = map.next_value::<::std::option::Option<_>>()?.map(consistency::Requirement::AtLeastAsFresh)
;
                        }
                        GeneratedField::AtExactSnapshot => {
                            if requirement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("atExactSnapshot"));
                            }
                            requirement__ = map.next_value::<::std::option::Option<_>>()?.map(consistency::Requirement::AtExactSnapshot)
;
                        }
                        GeneratedField::FullyConsistent => {
                            if requirement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullyConsistent"));
                            }
                            requirement__ = map.next_value::<::std::option::Option<_>>()?.map(consistency::Requirement::FullyConsistent);
                        }
                    }
                }
                Ok(Consistency {
                    requirement: requirement__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.Consistency", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContextualizedCaveat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.caveat_name.is_empty() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ContextualizedCaveat", len)?;
        if !self.caveat_name.is_empty() {
            struct_ser.serialize_field("caveatName", &self.caveat_name)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContextualizedCaveat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "caveat_name",
            "caveatName",
            "context",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CaveatName,
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
                            "caveatName" | "caveat_name" => Ok(GeneratedField::CaveatName),
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
            type Value = ContextualizedCaveat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ContextualizedCaveat")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContextualizedCaveat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut caveat_name__ = None;
                let mut context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CaveatName => {
                            if caveat_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveatName"));
                            }
                            caveat_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                    }
                }
                Ok(ContextualizedCaveat {
                    caveat_name: caveat_name__.unwrap_or_default(),
                    context: context__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ContextualizedCaveat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Cursor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.Cursor", len)?;
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
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
            "token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
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
                            "token" => Ok(GeneratedField::Token),
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
                formatter.write_str("struct authzed.api.v1.Cursor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Cursor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Cursor {
                    token: token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.Cursor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DebugInformation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.check.is_some() {
            len += 1;
        }
        if !self.schema_used.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.DebugInformation", len)?;
        if let Some(v) = self.check.as_ref() {
            struct_ser.serialize_field("check", v)?;
        }
        if !self.schema_used.is_empty() {
            struct_ser.serialize_field("schemaUsed", &self.schema_used)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DebugInformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "check",
            "schema_used",
            "schemaUsed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Check,
            SchemaUsed,
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
                            "check" => Ok(GeneratedField::Check),
                            "schemaUsed" | "schema_used" => Ok(GeneratedField::SchemaUsed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DebugInformation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.DebugInformation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DebugInformation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut check__ = None;
                let mut schema_used__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Check => {
                            if check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("check"));
                            }
                            check__ = map.next_value()?;
                        }
                        GeneratedField::SchemaUsed => {
                            if schema_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaUsed"));
                            }
                            schema_used__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DebugInformation {
                    check: check__,
                    schema_used: schema_used__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.DebugInformation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRelationshipsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.relationship_filter.is_some() {
            len += 1;
        }
        if !self.optional_preconditions.is_empty() {
            len += 1;
        }
        if self.optional_limit != 0 {
            len += 1;
        }
        if self.optional_allow_partial_deletions {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.DeleteRelationshipsRequest", len)?;
        if let Some(v) = self.relationship_filter.as_ref() {
            struct_ser.serialize_field("relationshipFilter", v)?;
        }
        if !self.optional_preconditions.is_empty() {
            struct_ser.serialize_field("optionalPreconditions", &self.optional_preconditions)?;
        }
        if self.optional_limit != 0 {
            struct_ser.serialize_field("optionalLimit", &self.optional_limit)?;
        }
        if self.optional_allow_partial_deletions {
            struct_ser.serialize_field("optionalAllowPartialDeletions", &self.optional_allow_partial_deletions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRelationshipsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relationship_filter",
            "relationshipFilter",
            "optional_preconditions",
            "optionalPreconditions",
            "optional_limit",
            "optionalLimit",
            "optional_allow_partial_deletions",
            "optionalAllowPartialDeletions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RelationshipFilter,
            OptionalPreconditions,
            OptionalLimit,
            OptionalAllowPartialDeletions,
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
                            "relationshipFilter" | "relationship_filter" => Ok(GeneratedField::RelationshipFilter),
                            "optionalPreconditions" | "optional_preconditions" => Ok(GeneratedField::OptionalPreconditions),
                            "optionalLimit" | "optional_limit" => Ok(GeneratedField::OptionalLimit),
                            "optionalAllowPartialDeletions" | "optional_allow_partial_deletions" => Ok(GeneratedField::OptionalAllowPartialDeletions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRelationshipsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.DeleteRelationshipsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteRelationshipsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relationship_filter__ = None;
                let mut optional_preconditions__ = None;
                let mut optional_limit__ = None;
                let mut optional_allow_partial_deletions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RelationshipFilter => {
                            if relationship_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationshipFilter"));
                            }
                            relationship_filter__ = map.next_value()?;
                        }
                        GeneratedField::OptionalPreconditions => {
                            if optional_preconditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalPreconditions"));
                            }
                            optional_preconditions__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalLimit => {
                            if optional_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalLimit"));
                            }
                            optional_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OptionalAllowPartialDeletions => {
                            if optional_allow_partial_deletions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalAllowPartialDeletions"));
                            }
                            optional_allow_partial_deletions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeleteRelationshipsRequest {
                    relationship_filter: relationship_filter__,
                    optional_preconditions: optional_preconditions__.unwrap_or_default(),
                    optional_limit: optional_limit__.unwrap_or_default(),
                    optional_allow_partial_deletions: optional_allow_partial_deletions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.DeleteRelationshipsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRelationshipsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deleted_at.is_some() {
            len += 1;
        }
        if self.deletion_progress != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.DeleteRelationshipsResponse", len)?;
        if let Some(v) = self.deleted_at.as_ref() {
            struct_ser.serialize_field("deletedAt", v)?;
        }
        if self.deletion_progress != 0 {
            let v = delete_relationships_response::DeletionProgress::from_i32(self.deletion_progress)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.deletion_progress)))?;
            struct_ser.serialize_field("deletionProgress", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRelationshipsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deleted_at",
            "deletedAt",
            "deletion_progress",
            "deletionProgress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeletedAt,
            DeletionProgress,
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
                            "deletedAt" | "deleted_at" => Ok(GeneratedField::DeletedAt),
                            "deletionProgress" | "deletion_progress" => Ok(GeneratedField::DeletionProgress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRelationshipsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.DeleteRelationshipsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteRelationshipsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deleted_at__ = None;
                let mut deletion_progress__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DeletedAt => {
                            if deleted_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedAt"));
                            }
                            deleted_at__ = map.next_value()?;
                        }
                        GeneratedField::DeletionProgress => {
                            if deletion_progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletionProgress"));
                            }
                            deletion_progress__ = Some(map.next_value::<delete_relationships_response::DeletionProgress>()? as i32);
                        }
                    }
                }
                Ok(DeleteRelationshipsResponse {
                    deleted_at: deleted_at__,
                    deletion_progress: deletion_progress__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.DeleteRelationshipsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for delete_relationships_response::DeletionProgress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DELETION_PROGRESS_UNSPECIFIED",
            Self::Complete => "DELETION_PROGRESS_COMPLETE",
            Self::Partial => "DELETION_PROGRESS_PARTIAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for delete_relationships_response::DeletionProgress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DELETION_PROGRESS_UNSPECIFIED",
            "DELETION_PROGRESS_COMPLETE",
            "DELETION_PROGRESS_PARTIAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = delete_relationships_response::DeletionProgress;

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
                    .and_then(delete_relationships_response::DeletionProgress::from_i32)
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
                    .and_then(delete_relationships_response::DeletionProgress::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DELETION_PROGRESS_UNSPECIFIED" => Ok(delete_relationships_response::DeletionProgress::Unspecified),
                    "DELETION_PROGRESS_COMPLETE" => Ok(delete_relationships_response::DeletionProgress::Complete),
                    "DELETION_PROGRESS_PARTIAL" => Ok(delete_relationships_response::DeletionProgress::Partial),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DirectSubjectSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subjects.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.DirectSubjectSet", len)?;
        if !self.subjects.is_empty() {
            struct_ser.serialize_field("subjects", &self.subjects)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectSubjectSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subjects",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subjects,
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
                            "subjects" => Ok(GeneratedField::Subjects),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DirectSubjectSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.DirectSubjectSet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DirectSubjectSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subjects__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Subjects => {
                            if subjects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjects"));
                            }
                            subjects__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DirectSubjectSet {
                    subjects: subjects__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.DirectSubjectSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ERROR_REASON_UNSPECIFIED",
            Self::SchemaParseError => "ERROR_REASON_SCHEMA_PARSE_ERROR",
            Self::SchemaTypeError => "ERROR_REASON_SCHEMA_TYPE_ERROR",
            Self::UnknownDefinition => "ERROR_REASON_UNKNOWN_DEFINITION",
            Self::UnknownRelationOrPermission => "ERROR_REASON_UNKNOWN_RELATION_OR_PERMISSION",
            Self::TooManyUpdatesInRequest => "ERROR_REASON_TOO_MANY_UPDATES_IN_REQUEST",
            Self::TooManyPreconditionsInRequest => "ERROR_REASON_TOO_MANY_PRECONDITIONS_IN_REQUEST",
            Self::WriteOrDeletePreconditionFailure => "ERROR_REASON_WRITE_OR_DELETE_PRECONDITION_FAILURE",
            Self::ServiceReadOnly => "ERROR_REASON_SERVICE_READ_ONLY",
            Self::UnknownCaveat => "ERROR_REASON_UNKNOWN_CAVEAT",
            Self::InvalidSubjectType => "ERROR_REASON_INVALID_SUBJECT_TYPE",
            Self::CaveatParameterTypeError => "ERROR_REASON_CAVEAT_PARAMETER_TYPE_ERROR",
            Self::UpdatesOnSameRelationship => "ERROR_REASON_UPDATES_ON_SAME_RELATIONSHIP",
            Self::CannotUpdatePermission => "ERROR_REASON_CANNOT_UPDATE_PERMISSION",
            Self::CaveatEvaluationError => "ERROR_REASON_CAVEAT_EVALUATION_ERROR",
            Self::InvalidCursor => "ERROR_REASON_INVALID_CURSOR",
            Self::TooManyRelationshipsForTransactionalDelete => "ERROR_REASON_TOO_MANY_RELATIONSHIPS_FOR_TRANSACTIONAL_DELETE",
            Self::MaxRelationshipContextSize => "ERROR_REASON_MAX_RELATIONSHIP_CONTEXT_SIZE",
            Self::AttemptToRecreateRelationship => "ERROR_REASON_ATTEMPT_TO_RECREATE_RELATIONSHIP",
            Self::MaximumDepthExceeded => "ERROR_REASON_MAXIMUM_DEPTH_EXCEEDED",
            Self::SerializationFailure => "ERROR_REASON_SERIALIZATION_FAILURE",
            Self::TooManyChecksInRequest => "ERROR_REASON_TOO_MANY_CHECKS_IN_REQUEST",
            Self::ExceedsMaximumAllowableLimit => "ERROR_REASON_EXCEEDS_MAXIMUM_ALLOWABLE_LIMIT",
            Self::InvalidFilter => "ERROR_REASON_INVALID_FILTER",
            Self::InmemoryTooManyConcurrentUpdates => "ERROR_REASON_INMEMORY_TOO_MANY_CONCURRENT_UPDATES",
            Self::EmptyPrecondition => "ERROR_REASON_EMPTY_PRECONDITION",
            Self::CounterAlreadyRegistered => "ERROR_REASON_COUNTER_ALREADY_REGISTERED",
            Self::CounterNotRegistered => "ERROR_REASON_COUNTER_NOT_REGISTERED",
            Self::WildcardNotAllowed => "ERROR_REASON_WILDCARD_NOT_ALLOWED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ERROR_REASON_UNSPECIFIED",
            "ERROR_REASON_SCHEMA_PARSE_ERROR",
            "ERROR_REASON_SCHEMA_TYPE_ERROR",
            "ERROR_REASON_UNKNOWN_DEFINITION",
            "ERROR_REASON_UNKNOWN_RELATION_OR_PERMISSION",
            "ERROR_REASON_TOO_MANY_UPDATES_IN_REQUEST",
            "ERROR_REASON_TOO_MANY_PRECONDITIONS_IN_REQUEST",
            "ERROR_REASON_WRITE_OR_DELETE_PRECONDITION_FAILURE",
            "ERROR_REASON_SERVICE_READ_ONLY",
            "ERROR_REASON_UNKNOWN_CAVEAT",
            "ERROR_REASON_INVALID_SUBJECT_TYPE",
            "ERROR_REASON_CAVEAT_PARAMETER_TYPE_ERROR",
            "ERROR_REASON_UPDATES_ON_SAME_RELATIONSHIP",
            "ERROR_REASON_CANNOT_UPDATE_PERMISSION",
            "ERROR_REASON_CAVEAT_EVALUATION_ERROR",
            "ERROR_REASON_INVALID_CURSOR",
            "ERROR_REASON_TOO_MANY_RELATIONSHIPS_FOR_TRANSACTIONAL_DELETE",
            "ERROR_REASON_MAX_RELATIONSHIP_CONTEXT_SIZE",
            "ERROR_REASON_ATTEMPT_TO_RECREATE_RELATIONSHIP",
            "ERROR_REASON_MAXIMUM_DEPTH_EXCEEDED",
            "ERROR_REASON_SERIALIZATION_FAILURE",
            "ERROR_REASON_TOO_MANY_CHECKS_IN_REQUEST",
            "ERROR_REASON_EXCEEDS_MAXIMUM_ALLOWABLE_LIMIT",
            "ERROR_REASON_INVALID_FILTER",
            "ERROR_REASON_INMEMORY_TOO_MANY_CONCURRENT_UPDATES",
            "ERROR_REASON_EMPTY_PRECONDITION",
            "ERROR_REASON_COUNTER_ALREADY_REGISTERED",
            "ERROR_REASON_COUNTER_NOT_REGISTERED",
            "ERROR_REASON_WILDCARD_NOT_ALLOWED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorReason;

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
                    .and_then(ErrorReason::from_i32)
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
                    .and_then(ErrorReason::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ERROR_REASON_UNSPECIFIED" => Ok(ErrorReason::Unspecified),
                    "ERROR_REASON_SCHEMA_PARSE_ERROR" => Ok(ErrorReason::SchemaParseError),
                    "ERROR_REASON_SCHEMA_TYPE_ERROR" => Ok(ErrorReason::SchemaTypeError),
                    "ERROR_REASON_UNKNOWN_DEFINITION" => Ok(ErrorReason::UnknownDefinition),
                    "ERROR_REASON_UNKNOWN_RELATION_OR_PERMISSION" => Ok(ErrorReason::UnknownRelationOrPermission),
                    "ERROR_REASON_TOO_MANY_UPDATES_IN_REQUEST" => Ok(ErrorReason::TooManyUpdatesInRequest),
                    "ERROR_REASON_TOO_MANY_PRECONDITIONS_IN_REQUEST" => Ok(ErrorReason::TooManyPreconditionsInRequest),
                    "ERROR_REASON_WRITE_OR_DELETE_PRECONDITION_FAILURE" => Ok(ErrorReason::WriteOrDeletePreconditionFailure),
                    "ERROR_REASON_SERVICE_READ_ONLY" => Ok(ErrorReason::ServiceReadOnly),
                    "ERROR_REASON_UNKNOWN_CAVEAT" => Ok(ErrorReason::UnknownCaveat),
                    "ERROR_REASON_INVALID_SUBJECT_TYPE" => Ok(ErrorReason::InvalidSubjectType),
                    "ERROR_REASON_CAVEAT_PARAMETER_TYPE_ERROR" => Ok(ErrorReason::CaveatParameterTypeError),
                    "ERROR_REASON_UPDATES_ON_SAME_RELATIONSHIP" => Ok(ErrorReason::UpdatesOnSameRelationship),
                    "ERROR_REASON_CANNOT_UPDATE_PERMISSION" => Ok(ErrorReason::CannotUpdatePermission),
                    "ERROR_REASON_CAVEAT_EVALUATION_ERROR" => Ok(ErrorReason::CaveatEvaluationError),
                    "ERROR_REASON_INVALID_CURSOR" => Ok(ErrorReason::InvalidCursor),
                    "ERROR_REASON_TOO_MANY_RELATIONSHIPS_FOR_TRANSACTIONAL_DELETE" => Ok(ErrorReason::TooManyRelationshipsForTransactionalDelete),
                    "ERROR_REASON_MAX_RELATIONSHIP_CONTEXT_SIZE" => Ok(ErrorReason::MaxRelationshipContextSize),
                    "ERROR_REASON_ATTEMPT_TO_RECREATE_RELATIONSHIP" => Ok(ErrorReason::AttemptToRecreateRelationship),
                    "ERROR_REASON_MAXIMUM_DEPTH_EXCEEDED" => Ok(ErrorReason::MaximumDepthExceeded),
                    "ERROR_REASON_SERIALIZATION_FAILURE" => Ok(ErrorReason::SerializationFailure),
                    "ERROR_REASON_TOO_MANY_CHECKS_IN_REQUEST" => Ok(ErrorReason::TooManyChecksInRequest),
                    "ERROR_REASON_EXCEEDS_MAXIMUM_ALLOWABLE_LIMIT" => Ok(ErrorReason::ExceedsMaximumAllowableLimit),
                    "ERROR_REASON_INVALID_FILTER" => Ok(ErrorReason::InvalidFilter),
                    "ERROR_REASON_INMEMORY_TOO_MANY_CONCURRENT_UPDATES" => Ok(ErrorReason::InmemoryTooManyConcurrentUpdates),
                    "ERROR_REASON_EMPTY_PRECONDITION" => Ok(ErrorReason::EmptyPrecondition),
                    "ERROR_REASON_COUNTER_ALREADY_REGISTERED" => Ok(ErrorReason::CounterAlreadyRegistered),
                    "ERROR_REASON_COUNTER_NOT_REGISTERED" => Ok(ErrorReason::CounterNotRegistered),
                    "ERROR_REASON_WILDCARD_NOT_ALLOWED" => Ok(ErrorReason::WildcardNotAllowed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExpCaveat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.comment.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        if !self.expression.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpCaveat", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.comment.is_empty() {
            struct_ser.serialize_field("comment", &self.comment)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpCaveat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "comment",
            "parameters",
            "expression",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Comment,
            Parameters,
            Expression,
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
                            "name" => Ok(GeneratedField::Name),
                            "comment" => Ok(GeneratedField::Comment),
                            "parameters" => Ok(GeneratedField::Parameters),
                            "expression" => Ok(GeneratedField::Expression),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpCaveat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpCaveat")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpCaveat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut comment__ = None;
                let mut parameters__ = None;
                let mut expression__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = Some(map.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(map.next_value()?);
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExpCaveat {
                    name: name__.unwrap_or_default(),
                    comment: comment__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                    expression: expression__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpCaveat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpCaveatParameter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.parent_caveat_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpCaveatParameter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.parent_caveat_name.is_empty() {
            struct_ser.serialize_field("parentCaveatName", &self.parent_caveat_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpCaveatParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type",
            "parent_caveat_name",
            "parentCaveatName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Type,
            ParentCaveatName,
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
                            "name" => Ok(GeneratedField::Name),
                            "type" => Ok(GeneratedField::Type),
                            "parentCaveatName" | "parent_caveat_name" => Ok(GeneratedField::ParentCaveatName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpCaveatParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpCaveatParameter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpCaveatParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                let mut parent_caveat_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::ParentCaveatName => {
                            if parent_caveat_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentCaveatName"));
                            }
                            parent_caveat_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExpCaveatParameter {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    parent_caveat_name: parent_caveat_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpCaveatParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpCaveatParameterTypeChange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.parameter.is_some() {
            len += 1;
        }
        if !self.previous_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpCaveatParameterTypeChange", len)?;
        if let Some(v) = self.parameter.as_ref() {
            struct_ser.serialize_field("parameter", v)?;
        }
        if !self.previous_type.is_empty() {
            struct_ser.serialize_field("previousType", &self.previous_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpCaveatParameterTypeChange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parameter",
            "previous_type",
            "previousType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parameter,
            PreviousType,
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
                            "parameter" => Ok(GeneratedField::Parameter),
                            "previousType" | "previous_type" => Ok(GeneratedField::PreviousType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpCaveatParameterTypeChange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpCaveatParameterTypeChange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpCaveatParameterTypeChange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parameter__ = None;
                let mut previous_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Parameter => {
                            if parameter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameter"));
                            }
                            parameter__ = map.next_value()?;
                        }
                        GeneratedField::PreviousType => {
                            if previous_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousType"));
                            }
                            previous_type__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExpCaveatParameterTypeChange {
                    parameter: parameter__,
                    previous_type: previous_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpCaveatParameterTypeChange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpDefinition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.comment.is_empty() {
            len += 1;
        }
        if !self.relations.is_empty() {
            len += 1;
        }
        if !self.permissions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpDefinition", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.comment.is_empty() {
            struct_ser.serialize_field("comment", &self.comment)?;
        }
        if !self.relations.is_empty() {
            struct_ser.serialize_field("relations", &self.relations)?;
        }
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpDefinition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "comment",
            "relations",
            "permissions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Comment,
            Relations,
            Permissions,
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
                            "name" => Ok(GeneratedField::Name),
                            "comment" => Ok(GeneratedField::Comment),
                            "relations" => Ok(GeneratedField::Relations),
                            "permissions" => Ok(GeneratedField::Permissions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpDefinition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpDefinition")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpDefinition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut comment__ = None;
                let mut relations__ = None;
                let mut permissions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = Some(map.next_value()?);
                        }
                        GeneratedField::Relations => {
                            if relations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relations"));
                            }
                            relations__ = Some(map.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExpDefinition {
                    name: name__.unwrap_or_default(),
                    comment: comment__.unwrap_or_default(),
                    relations: relations__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpDefinition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpPermission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.comment.is_empty() {
            len += 1;
        }
        if !self.parent_definition_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpPermission", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.comment.is_empty() {
            struct_ser.serialize_field("comment", &self.comment)?;
        }
        if !self.parent_definition_name.is_empty() {
            struct_ser.serialize_field("parentDefinitionName", &self.parent_definition_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpPermission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "comment",
            "parent_definition_name",
            "parentDefinitionName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Comment,
            ParentDefinitionName,
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
                            "name" => Ok(GeneratedField::Name),
                            "comment" => Ok(GeneratedField::Comment),
                            "parentDefinitionName" | "parent_definition_name" => Ok(GeneratedField::ParentDefinitionName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpPermission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpPermission")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpPermission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut comment__ = None;
                let mut parent_definition_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = Some(map.next_value()?);
                        }
                        GeneratedField::ParentDefinitionName => {
                            if parent_definition_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentDefinitionName"));
                            }
                            parent_definition_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExpPermission {
                    name: name__.unwrap_or_default(),
                    comment: comment__.unwrap_or_default(),
                    parent_definition_name: parent_definition_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpPermission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpRelation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.comment.is_empty() {
            len += 1;
        }
        if !self.parent_definition_name.is_empty() {
            len += 1;
        }
        if !self.subject_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpRelation", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.comment.is_empty() {
            struct_ser.serialize_field("comment", &self.comment)?;
        }
        if !self.parent_definition_name.is_empty() {
            struct_ser.serialize_field("parentDefinitionName", &self.parent_definition_name)?;
        }
        if !self.subject_types.is_empty() {
            struct_ser.serialize_field("subjectTypes", &self.subject_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpRelation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "comment",
            "parent_definition_name",
            "parentDefinitionName",
            "subject_types",
            "subjectTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Comment,
            ParentDefinitionName,
            SubjectTypes,
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
                            "name" => Ok(GeneratedField::Name),
                            "comment" => Ok(GeneratedField::Comment),
                            "parentDefinitionName" | "parent_definition_name" => Ok(GeneratedField::ParentDefinitionName),
                            "subjectTypes" | "subject_types" => Ok(GeneratedField::SubjectTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpRelation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpRelation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpRelation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut comment__ = None;
                let mut parent_definition_name__ = None;
                let mut subject_types__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = Some(map.next_value()?);
                        }
                        GeneratedField::ParentDefinitionName => {
                            if parent_definition_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentDefinitionName"));
                            }
                            parent_definition_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubjectTypes => {
                            if subject_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectTypes"));
                            }
                            subject_types__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExpRelation {
                    name: name__.unwrap_or_default(),
                    comment: comment__.unwrap_or_default(),
                    parent_definition_name: parent_definition_name__.unwrap_or_default(),
                    subject_types: subject_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpRelation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpRelationReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.definition_name.is_empty() {
            len += 1;
        }
        if !self.relation_name.is_empty() {
            len += 1;
        }
        if self.is_permission {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpRelationReference", len)?;
        if !self.definition_name.is_empty() {
            struct_ser.serialize_field("definitionName", &self.definition_name)?;
        }
        if !self.relation_name.is_empty() {
            struct_ser.serialize_field("relationName", &self.relation_name)?;
        }
        if self.is_permission {
            struct_ser.serialize_field("isPermission", &self.is_permission)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpRelationReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "definition_name",
            "definitionName",
            "relation_name",
            "relationName",
            "is_permission",
            "isPermission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefinitionName,
            RelationName,
            IsPermission,
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
                            "definitionName" | "definition_name" => Ok(GeneratedField::DefinitionName),
                            "relationName" | "relation_name" => Ok(GeneratedField::RelationName),
                            "isPermission" | "is_permission" => Ok(GeneratedField::IsPermission),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpRelationReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpRelationReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpRelationReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut definition_name__ = None;
                let mut relation_name__ = None;
                let mut is_permission__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DefinitionName => {
                            if definition_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitionName"));
                            }
                            definition_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::RelationName => {
                            if relation_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationName"));
                            }
                            relation_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::IsPermission => {
                            if is_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPermission"));
                            }
                            is_permission__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExpRelationReference {
                    definition_name: definition_name__.unwrap_or_default(),
                    relation_name: relation_name__.unwrap_or_default(),
                    is_permission: is_permission__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpRelationReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpRelationSubjectTypeChange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.relation.is_some() {
            len += 1;
        }
        if self.changed_subject_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpRelationSubjectTypeChange", len)?;
        if let Some(v) = self.relation.as_ref() {
            struct_ser.serialize_field("relation", v)?;
        }
        if let Some(v) = self.changed_subject_type.as_ref() {
            struct_ser.serialize_field("changedSubjectType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpRelationSubjectTypeChange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relation",
            "changed_subject_type",
            "changedSubjectType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relation,
            ChangedSubjectType,
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
                            "relation" => Ok(GeneratedField::Relation),
                            "changedSubjectType" | "changed_subject_type" => Ok(GeneratedField::ChangedSubjectType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpRelationSubjectTypeChange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpRelationSubjectTypeChange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpRelationSubjectTypeChange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relation__ = None;
                let mut changed_subject_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = map.next_value()?;
                        }
                        GeneratedField::ChangedSubjectType => {
                            if changed_subject_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changedSubjectType"));
                            }
                            changed_subject_type__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExpRelationSubjectTypeChange {
                    relation: relation__,
                    changed_subject_type: changed_subject_type__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpRelationSubjectTypeChange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpSchemaDiff {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.diff.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpSchemaDiff", len)?;
        if let Some(v) = self.diff.as_ref() {
            match v {
                exp_schema_diff::Diff::DefinitionAdded(v) => {
                    struct_ser.serialize_field("definitionAdded", v)?;
                }
                exp_schema_diff::Diff::DefinitionRemoved(v) => {
                    struct_ser.serialize_field("definitionRemoved", v)?;
                }
                exp_schema_diff::Diff::DefinitionDocCommentChanged(v) => {
                    struct_ser.serialize_field("definitionDocCommentChanged", v)?;
                }
                exp_schema_diff::Diff::RelationAdded(v) => {
                    struct_ser.serialize_field("relationAdded", v)?;
                }
                exp_schema_diff::Diff::RelationRemoved(v) => {
                    struct_ser.serialize_field("relationRemoved", v)?;
                }
                exp_schema_diff::Diff::RelationDocCommentChanged(v) => {
                    struct_ser.serialize_field("relationDocCommentChanged", v)?;
                }
                exp_schema_diff::Diff::RelationSubjectTypeAdded(v) => {
                    struct_ser.serialize_field("relationSubjectTypeAdded", v)?;
                }
                exp_schema_diff::Diff::RelationSubjectTypeRemoved(v) => {
                    struct_ser.serialize_field("relationSubjectTypeRemoved", v)?;
                }
                exp_schema_diff::Diff::PermissionAdded(v) => {
                    struct_ser.serialize_field("permissionAdded", v)?;
                }
                exp_schema_diff::Diff::PermissionRemoved(v) => {
                    struct_ser.serialize_field("permissionRemoved", v)?;
                }
                exp_schema_diff::Diff::PermissionDocCommentChanged(v) => {
                    struct_ser.serialize_field("permissionDocCommentChanged", v)?;
                }
                exp_schema_diff::Diff::PermissionExprChanged(v) => {
                    struct_ser.serialize_field("permissionExprChanged", v)?;
                }
                exp_schema_diff::Diff::CaveatAdded(v) => {
                    struct_ser.serialize_field("caveatAdded", v)?;
                }
                exp_schema_diff::Diff::CaveatRemoved(v) => {
                    struct_ser.serialize_field("caveatRemoved", v)?;
                }
                exp_schema_diff::Diff::CaveatDocCommentChanged(v) => {
                    struct_ser.serialize_field("caveatDocCommentChanged", v)?;
                }
                exp_schema_diff::Diff::CaveatExprChanged(v) => {
                    struct_ser.serialize_field("caveatExprChanged", v)?;
                }
                exp_schema_diff::Diff::CaveatParameterAdded(v) => {
                    struct_ser.serialize_field("caveatParameterAdded", v)?;
                }
                exp_schema_diff::Diff::CaveatParameterRemoved(v) => {
                    struct_ser.serialize_field("caveatParameterRemoved", v)?;
                }
                exp_schema_diff::Diff::CaveatParameterTypeChanged(v) => {
                    struct_ser.serialize_field("caveatParameterTypeChanged", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpSchemaDiff {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "definition_added",
            "definitionAdded",
            "definition_removed",
            "definitionRemoved",
            "definition_doc_comment_changed",
            "definitionDocCommentChanged",
            "relation_added",
            "relationAdded",
            "relation_removed",
            "relationRemoved",
            "relation_doc_comment_changed",
            "relationDocCommentChanged",
            "relation_subject_type_added",
            "relationSubjectTypeAdded",
            "relation_subject_type_removed",
            "relationSubjectTypeRemoved",
            "permission_added",
            "permissionAdded",
            "permission_removed",
            "permissionRemoved",
            "permission_doc_comment_changed",
            "permissionDocCommentChanged",
            "permission_expr_changed",
            "permissionExprChanged",
            "caveat_added",
            "caveatAdded",
            "caveat_removed",
            "caveatRemoved",
            "caveat_doc_comment_changed",
            "caveatDocCommentChanged",
            "caveat_expr_changed",
            "caveatExprChanged",
            "caveat_parameter_added",
            "caveatParameterAdded",
            "caveat_parameter_removed",
            "caveatParameterRemoved",
            "caveat_parameter_type_changed",
            "caveatParameterTypeChanged",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefinitionAdded,
            DefinitionRemoved,
            DefinitionDocCommentChanged,
            RelationAdded,
            RelationRemoved,
            RelationDocCommentChanged,
            RelationSubjectTypeAdded,
            RelationSubjectTypeRemoved,
            PermissionAdded,
            PermissionRemoved,
            PermissionDocCommentChanged,
            PermissionExprChanged,
            CaveatAdded,
            CaveatRemoved,
            CaveatDocCommentChanged,
            CaveatExprChanged,
            CaveatParameterAdded,
            CaveatParameterRemoved,
            CaveatParameterTypeChanged,
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
                            "definitionAdded" | "definition_added" => Ok(GeneratedField::DefinitionAdded),
                            "definitionRemoved" | "definition_removed" => Ok(GeneratedField::DefinitionRemoved),
                            "definitionDocCommentChanged" | "definition_doc_comment_changed" => Ok(GeneratedField::DefinitionDocCommentChanged),
                            "relationAdded" | "relation_added" => Ok(GeneratedField::RelationAdded),
                            "relationRemoved" | "relation_removed" => Ok(GeneratedField::RelationRemoved),
                            "relationDocCommentChanged" | "relation_doc_comment_changed" => Ok(GeneratedField::RelationDocCommentChanged),
                            "relationSubjectTypeAdded" | "relation_subject_type_added" => Ok(GeneratedField::RelationSubjectTypeAdded),
                            "relationSubjectTypeRemoved" | "relation_subject_type_removed" => Ok(GeneratedField::RelationSubjectTypeRemoved),
                            "permissionAdded" | "permission_added" => Ok(GeneratedField::PermissionAdded),
                            "permissionRemoved" | "permission_removed" => Ok(GeneratedField::PermissionRemoved),
                            "permissionDocCommentChanged" | "permission_doc_comment_changed" => Ok(GeneratedField::PermissionDocCommentChanged),
                            "permissionExprChanged" | "permission_expr_changed" => Ok(GeneratedField::PermissionExprChanged),
                            "caveatAdded" | "caveat_added" => Ok(GeneratedField::CaveatAdded),
                            "caveatRemoved" | "caveat_removed" => Ok(GeneratedField::CaveatRemoved),
                            "caveatDocCommentChanged" | "caveat_doc_comment_changed" => Ok(GeneratedField::CaveatDocCommentChanged),
                            "caveatExprChanged" | "caveat_expr_changed" => Ok(GeneratedField::CaveatExprChanged),
                            "caveatParameterAdded" | "caveat_parameter_added" => Ok(GeneratedField::CaveatParameterAdded),
                            "caveatParameterRemoved" | "caveat_parameter_removed" => Ok(GeneratedField::CaveatParameterRemoved),
                            "caveatParameterTypeChanged" | "caveat_parameter_type_changed" => Ok(GeneratedField::CaveatParameterTypeChanged),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpSchemaDiff;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpSchemaDiff")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpSchemaDiff, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut diff__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DefinitionAdded => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitionAdded"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::DefinitionAdded)
;
                        }
                        GeneratedField::DefinitionRemoved => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitionRemoved"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::DefinitionRemoved)
;
                        }
                        GeneratedField::DefinitionDocCommentChanged => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitionDocCommentChanged"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::DefinitionDocCommentChanged)
;
                        }
                        GeneratedField::RelationAdded => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationAdded"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::RelationAdded)
;
                        }
                        GeneratedField::RelationRemoved => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationRemoved"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::RelationRemoved)
;
                        }
                        GeneratedField::RelationDocCommentChanged => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationDocCommentChanged"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::RelationDocCommentChanged)
;
                        }
                        GeneratedField::RelationSubjectTypeAdded => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationSubjectTypeAdded"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::RelationSubjectTypeAdded)
;
                        }
                        GeneratedField::RelationSubjectTypeRemoved => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationSubjectTypeRemoved"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::RelationSubjectTypeRemoved)
;
                        }
                        GeneratedField::PermissionAdded => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionAdded"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::PermissionAdded)
;
                        }
                        GeneratedField::PermissionRemoved => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionRemoved"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::PermissionRemoved)
;
                        }
                        GeneratedField::PermissionDocCommentChanged => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionDocCommentChanged"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::PermissionDocCommentChanged)
;
                        }
                        GeneratedField::PermissionExprChanged => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionExprChanged"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::PermissionExprChanged)
;
                        }
                        GeneratedField::CaveatAdded => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveatAdded"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::CaveatAdded)
;
                        }
                        GeneratedField::CaveatRemoved => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveatRemoved"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::CaveatRemoved)
;
                        }
                        GeneratedField::CaveatDocCommentChanged => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveatDocCommentChanged"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::CaveatDocCommentChanged)
;
                        }
                        GeneratedField::CaveatExprChanged => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveatExprChanged"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::CaveatExprChanged)
;
                        }
                        GeneratedField::CaveatParameterAdded => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveatParameterAdded"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::CaveatParameterAdded)
;
                        }
                        GeneratedField::CaveatParameterRemoved => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveatParameterRemoved"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::CaveatParameterRemoved)
;
                        }
                        GeneratedField::CaveatParameterTypeChanged => {
                            if diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveatParameterTypeChanged"));
                            }
                            diff__ = map.next_value::<::std::option::Option<_>>()?.map(exp_schema_diff::Diff::CaveatParameterTypeChanged)
;
                        }
                    }
                }
                Ok(ExpSchemaDiff {
                    diff: diff__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpSchemaDiff", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpSchemaFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.optional_definition_name_filter.is_empty() {
            len += 1;
        }
        if !self.optional_caveat_name_filter.is_empty() {
            len += 1;
        }
        if !self.optional_relation_name_filter.is_empty() {
            len += 1;
        }
        if !self.optional_permission_name_filter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpSchemaFilter", len)?;
        if !self.optional_definition_name_filter.is_empty() {
            struct_ser.serialize_field("optionalDefinitionNameFilter", &self.optional_definition_name_filter)?;
        }
        if !self.optional_caveat_name_filter.is_empty() {
            struct_ser.serialize_field("optionalCaveatNameFilter", &self.optional_caveat_name_filter)?;
        }
        if !self.optional_relation_name_filter.is_empty() {
            struct_ser.serialize_field("optionalRelationNameFilter", &self.optional_relation_name_filter)?;
        }
        if !self.optional_permission_name_filter.is_empty() {
            struct_ser.serialize_field("optionalPermissionNameFilter", &self.optional_permission_name_filter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpSchemaFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "optional_definition_name_filter",
            "optionalDefinitionNameFilter",
            "optional_caveat_name_filter",
            "optionalCaveatNameFilter",
            "optional_relation_name_filter",
            "optionalRelationNameFilter",
            "optional_permission_name_filter",
            "optionalPermissionNameFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OptionalDefinitionNameFilter,
            OptionalCaveatNameFilter,
            OptionalRelationNameFilter,
            OptionalPermissionNameFilter,
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
                            "optionalDefinitionNameFilter" | "optional_definition_name_filter" => Ok(GeneratedField::OptionalDefinitionNameFilter),
                            "optionalCaveatNameFilter" | "optional_caveat_name_filter" => Ok(GeneratedField::OptionalCaveatNameFilter),
                            "optionalRelationNameFilter" | "optional_relation_name_filter" => Ok(GeneratedField::OptionalRelationNameFilter),
                            "optionalPermissionNameFilter" | "optional_permission_name_filter" => Ok(GeneratedField::OptionalPermissionNameFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpSchemaFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpSchemaFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpSchemaFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut optional_definition_name_filter__ = None;
                let mut optional_caveat_name_filter__ = None;
                let mut optional_relation_name_filter__ = None;
                let mut optional_permission_name_filter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OptionalDefinitionNameFilter => {
                            if optional_definition_name_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalDefinitionNameFilter"));
                            }
                            optional_definition_name_filter__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalCaveatNameFilter => {
                            if optional_caveat_name_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalCaveatNameFilter"));
                            }
                            optional_caveat_name_filter__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalRelationNameFilter => {
                            if optional_relation_name_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalRelationNameFilter"));
                            }
                            optional_relation_name_filter__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalPermissionNameFilter => {
                            if optional_permission_name_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalPermissionNameFilter"));
                            }
                            optional_permission_name_filter__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExpSchemaFilter {
                    optional_definition_name_filter: optional_definition_name_filter__.unwrap_or_default(),
                    optional_caveat_name_filter: optional_caveat_name_filter__.unwrap_or_default(),
                    optional_relation_name_filter: optional_relation_name_filter__.unwrap_or_default(),
                    optional_permission_name_filter: optional_permission_name_filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpSchemaFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpTypeReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subject_definition_name.is_empty() {
            len += 1;
        }
        if !self.optional_caveat_name.is_empty() {
            len += 1;
        }
        if self.typeref.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpTypeReference", len)?;
        if !self.subject_definition_name.is_empty() {
            struct_ser.serialize_field("subjectDefinitionName", &self.subject_definition_name)?;
        }
        if !self.optional_caveat_name.is_empty() {
            struct_ser.serialize_field("optionalCaveatName", &self.optional_caveat_name)?;
        }
        if let Some(v) = self.typeref.as_ref() {
            match v {
                exp_type_reference::Typeref::IsTerminalSubject(v) => {
                    struct_ser.serialize_field("isTerminalSubject", v)?;
                }
                exp_type_reference::Typeref::OptionalRelationName(v) => {
                    struct_ser.serialize_field("optionalRelationName", v)?;
                }
                exp_type_reference::Typeref::IsPublicWildcard(v) => {
                    struct_ser.serialize_field("isPublicWildcard", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpTypeReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject_definition_name",
            "subjectDefinitionName",
            "optional_caveat_name",
            "optionalCaveatName",
            "is_terminal_subject",
            "isTerminalSubject",
            "optional_relation_name",
            "optionalRelationName",
            "is_public_wildcard",
            "isPublicWildcard",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubjectDefinitionName,
            OptionalCaveatName,
            IsTerminalSubject,
            OptionalRelationName,
            IsPublicWildcard,
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
                            "subjectDefinitionName" | "subject_definition_name" => Ok(GeneratedField::SubjectDefinitionName),
                            "optionalCaveatName" | "optional_caveat_name" => Ok(GeneratedField::OptionalCaveatName),
                            "isTerminalSubject" | "is_terminal_subject" => Ok(GeneratedField::IsTerminalSubject),
                            "optionalRelationName" | "optional_relation_name" => Ok(GeneratedField::OptionalRelationName),
                            "isPublicWildcard" | "is_public_wildcard" => Ok(GeneratedField::IsPublicWildcard),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpTypeReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpTypeReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpTypeReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject_definition_name__ = None;
                let mut optional_caveat_name__ = None;
                let mut typeref__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubjectDefinitionName => {
                            if subject_definition_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectDefinitionName"));
                            }
                            subject_definition_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalCaveatName => {
                            if optional_caveat_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalCaveatName"));
                            }
                            optional_caveat_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::IsTerminalSubject => {
                            if typeref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isTerminalSubject"));
                            }
                            typeref__ = map.next_value::<::std::option::Option<_>>()?.map(exp_type_reference::Typeref::IsTerminalSubject);
                        }
                        GeneratedField::OptionalRelationName => {
                            if typeref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalRelationName"));
                            }
                            typeref__ = map.next_value::<::std::option::Option<_>>()?.map(exp_type_reference::Typeref::OptionalRelationName);
                        }
                        GeneratedField::IsPublicWildcard => {
                            if typeref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPublicWildcard"));
                            }
                            typeref__ = map.next_value::<::std::option::Option<_>>()?.map(exp_type_reference::Typeref::IsPublicWildcard);
                        }
                    }
                }
                Ok(ExpTypeReference {
                    subject_definition_name: subject_definition_name__.unwrap_or_default(),
                    optional_caveat_name: optional_caveat_name__.unwrap_or_default(),
                    typeref: typeref__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpTypeReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpandPermissionTreeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if !self.permission.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpandPermissionTreeRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.permission.is_empty() {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpandPermissionTreeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "resource",
            "permission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            Resource,
            Permission,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "resource" => Ok(GeneratedField::Resource),
                            "permission" => Ok(GeneratedField::Permission),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpandPermissionTreeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpandPermissionTreeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpandPermissionTreeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut resource__ = None;
                let mut permission__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
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
                    }
                }
                Ok(ExpandPermissionTreeRequest {
                    consistency: consistency__,
                    resource: resource__,
                    permission: permission__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpandPermissionTreeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpandPermissionTreeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expanded_at.is_some() {
            len += 1;
        }
        if self.tree_root.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExpandPermissionTreeResponse", len)?;
        if let Some(v) = self.expanded_at.as_ref() {
            struct_ser.serialize_field("expandedAt", v)?;
        }
        if let Some(v) = self.tree_root.as_ref() {
            struct_ser.serialize_field("treeRoot", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpandPermissionTreeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expanded_at",
            "expandedAt",
            "tree_root",
            "treeRoot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExpandedAt,
            TreeRoot,
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
                            "expandedAt" | "expanded_at" => Ok(GeneratedField::ExpandedAt),
                            "treeRoot" | "tree_root" => Ok(GeneratedField::TreeRoot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpandPermissionTreeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExpandPermissionTreeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpandPermissionTreeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expanded_at__ = None;
                let mut tree_root__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExpandedAt => {
                            if expanded_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expandedAt"));
                            }
                            expanded_at__ = map.next_value()?;
                        }
                        GeneratedField::TreeRoot => {
                            if tree_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treeRoot"));
                            }
                            tree_root__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExpandPermissionTreeResponse {
                    expanded_at: expanded_at__,
                    tree_root: tree_root__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExpandPermissionTreeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalComputablePermissionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if !self.definition_name.is_empty() {
            len += 1;
        }
        if !self.relation_name.is_empty() {
            len += 1;
        }
        if !self.optional_definition_name_filter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalComputablePermissionsRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if !self.definition_name.is_empty() {
            struct_ser.serialize_field("definitionName", &self.definition_name)?;
        }
        if !self.relation_name.is_empty() {
            struct_ser.serialize_field("relationName", &self.relation_name)?;
        }
        if !self.optional_definition_name_filter.is_empty() {
            struct_ser.serialize_field("optionalDefinitionNameFilter", &self.optional_definition_name_filter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalComputablePermissionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "definition_name",
            "definitionName",
            "relation_name",
            "relationName",
            "optional_definition_name_filter",
            "optionalDefinitionNameFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            DefinitionName,
            RelationName,
            OptionalDefinitionNameFilter,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "definitionName" | "definition_name" => Ok(GeneratedField::DefinitionName),
                            "relationName" | "relation_name" => Ok(GeneratedField::RelationName),
                            "optionalDefinitionNameFilter" | "optional_definition_name_filter" => Ok(GeneratedField::OptionalDefinitionNameFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalComputablePermissionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalComputablePermissionsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalComputablePermissionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut definition_name__ = None;
                let mut relation_name__ = None;
                let mut optional_definition_name_filter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
                        }
                        GeneratedField::DefinitionName => {
                            if definition_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitionName"));
                            }
                            definition_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::RelationName => {
                            if relation_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationName"));
                            }
                            relation_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalDefinitionNameFilter => {
                            if optional_definition_name_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalDefinitionNameFilter"));
                            }
                            optional_definition_name_filter__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExperimentalComputablePermissionsRequest {
                    consistency: consistency__,
                    definition_name: definition_name__.unwrap_or_default(),
                    relation_name: relation_name__.unwrap_or_default(),
                    optional_definition_name_filter: optional_definition_name_filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalComputablePermissionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalComputablePermissionsResponse {
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
        if self.read_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalComputablePermissionsResponse", len)?;
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        if let Some(v) = self.read_at.as_ref() {
            struct_ser.serialize_field("readAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalComputablePermissionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permissions",
            "read_at",
            "readAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permissions,
            ReadAt,
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
                            "readAt" | "read_at" => Ok(GeneratedField::ReadAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalComputablePermissionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalComputablePermissionsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalComputablePermissionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permissions__ = None;
                let mut read_at__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReadAt => {
                            if read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readAt"));
                            }
                            read_at__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExperimentalComputablePermissionsResponse {
                    permissions: permissions__.unwrap_or_default(),
                    read_at: read_at__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalComputablePermissionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalCountRelationshipsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalCountRelationshipsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalCountRelationshipsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalCountRelationshipsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalCountRelationshipsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalCountRelationshipsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExperimentalCountRelationshipsRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalCountRelationshipsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalCountRelationshipsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.counter_result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalCountRelationshipsResponse", len)?;
        if let Some(v) = self.counter_result.as_ref() {
            match v {
                experimental_count_relationships_response::CounterResult::CounterStillCalculating(v) => {
                    struct_ser.serialize_field("counterStillCalculating", v)?;
                }
                experimental_count_relationships_response::CounterResult::ReadCounterValue(v) => {
                    struct_ser.serialize_field("readCounterValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalCountRelationshipsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "counter_still_calculating",
            "counterStillCalculating",
            "read_counter_value",
            "readCounterValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CounterStillCalculating,
            ReadCounterValue,
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
                            "counterStillCalculating" | "counter_still_calculating" => Ok(GeneratedField::CounterStillCalculating),
                            "readCounterValue" | "read_counter_value" => Ok(GeneratedField::ReadCounterValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalCountRelationshipsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalCountRelationshipsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalCountRelationshipsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut counter_result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CounterStillCalculating => {
                            if counter_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterStillCalculating"));
                            }
                            counter_result__ = map.next_value::<::std::option::Option<_>>()?.map(experimental_count_relationships_response::CounterResult::CounterStillCalculating);
                        }
                        GeneratedField::ReadCounterValue => {
                            if counter_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readCounterValue"));
                            }
                            counter_result__ = map.next_value::<::std::option::Option<_>>()?.map(experimental_count_relationships_response::CounterResult::ReadCounterValue)
;
                        }
                    }
                }
                Ok(ExperimentalCountRelationshipsResponse {
                    counter_result: counter_result__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalCountRelationshipsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalDependentRelationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if !self.definition_name.is_empty() {
            len += 1;
        }
        if !self.permission_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalDependentRelationsRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if !self.definition_name.is_empty() {
            struct_ser.serialize_field("definitionName", &self.definition_name)?;
        }
        if !self.permission_name.is_empty() {
            struct_ser.serialize_field("permissionName", &self.permission_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalDependentRelationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "definition_name",
            "definitionName",
            "permission_name",
            "permissionName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            DefinitionName,
            PermissionName,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "definitionName" | "definition_name" => Ok(GeneratedField::DefinitionName),
                            "permissionName" | "permission_name" => Ok(GeneratedField::PermissionName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalDependentRelationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalDependentRelationsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalDependentRelationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut definition_name__ = None;
                let mut permission_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
                        }
                        GeneratedField::DefinitionName => {
                            if definition_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitionName"));
                            }
                            definition_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::PermissionName => {
                            if permission_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionName"));
                            }
                            permission_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExperimentalDependentRelationsRequest {
                    consistency: consistency__,
                    definition_name: definition_name__.unwrap_or_default(),
                    permission_name: permission_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalDependentRelationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalDependentRelationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relations.is_empty() {
            len += 1;
        }
        if self.read_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalDependentRelationsResponse", len)?;
        if !self.relations.is_empty() {
            struct_ser.serialize_field("relations", &self.relations)?;
        }
        if let Some(v) = self.read_at.as_ref() {
            struct_ser.serialize_field("readAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalDependentRelationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relations",
            "read_at",
            "readAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relations,
            ReadAt,
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
                            "relations" => Ok(GeneratedField::Relations),
                            "readAt" | "read_at" => Ok(GeneratedField::ReadAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalDependentRelationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalDependentRelationsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalDependentRelationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relations__ = None;
                let mut read_at__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Relations => {
                            if relations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relations"));
                            }
                            relations__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReadAt => {
                            if read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readAt"));
                            }
                            read_at__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExperimentalDependentRelationsResponse {
                    relations: relations__.unwrap_or_default(),
                    read_at: read_at__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalDependentRelationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalDiffSchemaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if !self.comparison_schema.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalDiffSchemaRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if !self.comparison_schema.is_empty() {
            struct_ser.serialize_field("comparisonSchema", &self.comparison_schema)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalDiffSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "comparison_schema",
            "comparisonSchema",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            ComparisonSchema,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "comparisonSchema" | "comparison_schema" => Ok(GeneratedField::ComparisonSchema),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalDiffSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalDiffSchemaRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalDiffSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut comparison_schema__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
                        }
                        GeneratedField::ComparisonSchema => {
                            if comparison_schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comparisonSchema"));
                            }
                            comparison_schema__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExperimentalDiffSchemaRequest {
                    consistency: consistency__,
                    comparison_schema: comparison_schema__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalDiffSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalDiffSchemaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.diffs.is_empty() {
            len += 1;
        }
        if self.read_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalDiffSchemaResponse", len)?;
        if !self.diffs.is_empty() {
            struct_ser.serialize_field("diffs", &self.diffs)?;
        }
        if let Some(v) = self.read_at.as_ref() {
            struct_ser.serialize_field("readAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalDiffSchemaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "diffs",
            "read_at",
            "readAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Diffs,
            ReadAt,
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
                            "diffs" => Ok(GeneratedField::Diffs),
                            "readAt" | "read_at" => Ok(GeneratedField::ReadAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalDiffSchemaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalDiffSchemaResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalDiffSchemaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut diffs__ = None;
                let mut read_at__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Diffs => {
                            if diffs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("diffs"));
                            }
                            diffs__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReadAt => {
                            if read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readAt"));
                            }
                            read_at__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExperimentalDiffSchemaResponse {
                    diffs: diffs__.unwrap_or_default(),
                    read_at: read_at__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalDiffSchemaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalReflectSchemaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if !self.optional_filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalReflectSchemaRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if !self.optional_filters.is_empty() {
            struct_ser.serialize_field("optionalFilters", &self.optional_filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalReflectSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "optional_filters",
            "optionalFilters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            OptionalFilters,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "optionalFilters" | "optional_filters" => Ok(GeneratedField::OptionalFilters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalReflectSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalReflectSchemaRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalReflectSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut optional_filters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
                        }
                        GeneratedField::OptionalFilters => {
                            if optional_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalFilters"));
                            }
                            optional_filters__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExperimentalReflectSchemaRequest {
                    consistency: consistency__,
                    optional_filters: optional_filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalReflectSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalReflectSchemaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.definitions.is_empty() {
            len += 1;
        }
        if !self.caveats.is_empty() {
            len += 1;
        }
        if self.read_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalReflectSchemaResponse", len)?;
        if !self.definitions.is_empty() {
            struct_ser.serialize_field("definitions", &self.definitions)?;
        }
        if !self.caveats.is_empty() {
            struct_ser.serialize_field("caveats", &self.caveats)?;
        }
        if let Some(v) = self.read_at.as_ref() {
            struct_ser.serialize_field("readAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalReflectSchemaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "definitions",
            "caveats",
            "read_at",
            "readAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Definitions,
            Caveats,
            ReadAt,
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
                            "definitions" => Ok(GeneratedField::Definitions),
                            "caveats" => Ok(GeneratedField::Caveats),
                            "readAt" | "read_at" => Ok(GeneratedField::ReadAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalReflectSchemaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalReflectSchemaResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalReflectSchemaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut definitions__ = None;
                let mut caveats__ = None;
                let mut read_at__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Definitions => {
                            if definitions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitions"));
                            }
                            definitions__ = Some(map.next_value()?);
                        }
                        GeneratedField::Caveats => {
                            if caveats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveats"));
                            }
                            caveats__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReadAt => {
                            if read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readAt"));
                            }
                            read_at__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExperimentalReflectSchemaResponse {
                    definitions: definitions__.unwrap_or_default(),
                    caveats: caveats__.unwrap_or_default(),
                    read_at: read_at__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalReflectSchemaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalRegisterRelationshipCounterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.relationship_filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalRegisterRelationshipCounterRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.relationship_filter.as_ref() {
            struct_ser.serialize_field("relationshipFilter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalRegisterRelationshipCounterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "relationship_filter",
            "relationshipFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            RelationshipFilter,
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
                            "name" => Ok(GeneratedField::Name),
                            "relationshipFilter" | "relationship_filter" => Ok(GeneratedField::RelationshipFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalRegisterRelationshipCounterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalRegisterRelationshipCounterRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalRegisterRelationshipCounterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut relationship_filter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::RelationshipFilter => {
                            if relationship_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationshipFilter"));
                            }
                            relationship_filter__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExperimentalRegisterRelationshipCounterRequest {
                    name: name__.unwrap_or_default(),
                    relationship_filter: relationship_filter__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalRegisterRelationshipCounterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalRegisterRelationshipCounterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalRegisterRelationshipCounterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalRegisterRelationshipCounterResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalRegisterRelationshipCounterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalRegisterRelationshipCounterResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalRegisterRelationshipCounterResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ExperimentalRegisterRelationshipCounterResponse {
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalRegisterRelationshipCounterResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalUnregisterRelationshipCounterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalUnregisterRelationshipCounterRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalUnregisterRelationshipCounterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalUnregisterRelationshipCounterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalUnregisterRelationshipCounterRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalUnregisterRelationshipCounterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExperimentalUnregisterRelationshipCounterRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalUnregisterRelationshipCounterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentalUnregisterRelationshipCounterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("authzed.api.v1.ExperimentalUnregisterRelationshipCounterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentalUnregisterRelationshipCounterResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentalUnregisterRelationshipCounterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ExperimentalUnregisterRelationshipCounterResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentalUnregisterRelationshipCounterResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ExperimentalUnregisterRelationshipCounterResponse {
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ExperimentalUnregisterRelationshipCounterResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupPermissionship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "LOOKUP_PERMISSIONSHIP_UNSPECIFIED",
            Self::HasPermission => "LOOKUP_PERMISSIONSHIP_HAS_PERMISSION",
            Self::ConditionalPermission => "LOOKUP_PERMISSIONSHIP_CONDITIONAL_PERMISSION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for LookupPermissionship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LOOKUP_PERMISSIONSHIP_UNSPECIFIED",
            "LOOKUP_PERMISSIONSHIP_HAS_PERMISSION",
            "LOOKUP_PERMISSIONSHIP_CONDITIONAL_PERMISSION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupPermissionship;

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
                    .and_then(LookupPermissionship::from_i32)
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
                    .and_then(LookupPermissionship::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "LOOKUP_PERMISSIONSHIP_UNSPECIFIED" => Ok(LookupPermissionship::Unspecified),
                    "LOOKUP_PERMISSIONSHIP_HAS_PERMISSION" => Ok(LookupPermissionship::HasPermission),
                    "LOOKUP_PERMISSIONSHIP_CONDITIONAL_PERMISSION" => Ok(LookupPermissionship::ConditionalPermission),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LookupResourcesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if !self.resource_object_type.is_empty() {
            len += 1;
        }
        if !self.permission.is_empty() {
            len += 1;
        }
        if self.subject.is_some() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        if self.optional_limit != 0 {
            len += 1;
        }
        if self.optional_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.LookupResourcesRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if !self.resource_object_type.is_empty() {
            struct_ser.serialize_field("resourceObjectType", &self.resource_object_type)?;
        }
        if !self.permission.is_empty() {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if self.optional_limit != 0 {
            struct_ser.serialize_field("optionalLimit", &self.optional_limit)?;
        }
        if let Some(v) = self.optional_cursor.as_ref() {
            struct_ser.serialize_field("optionalCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupResourcesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "resource_object_type",
            "resourceObjectType",
            "permission",
            "subject",
            "context",
            "optional_limit",
            "optionalLimit",
            "optional_cursor",
            "optionalCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            ResourceObjectType,
            Permission,
            Subject,
            Context,
            OptionalLimit,
            OptionalCursor,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "resourceObjectType" | "resource_object_type" => Ok(GeneratedField::ResourceObjectType),
                            "permission" => Ok(GeneratedField::Permission),
                            "subject" => Ok(GeneratedField::Subject),
                            "context" => Ok(GeneratedField::Context),
                            "optionalLimit" | "optional_limit" => Ok(GeneratedField::OptionalLimit),
                            "optionalCursor" | "optional_cursor" => Ok(GeneratedField::OptionalCursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupResourcesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.LookupResourcesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LookupResourcesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut resource_object_type__ = None;
                let mut permission__ = None;
                let mut subject__ = None;
                let mut context__ = None;
                let mut optional_limit__ = None;
                let mut optional_cursor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
                        }
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
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map.next_value()?;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                        GeneratedField::OptionalLimit => {
                            if optional_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalLimit"));
                            }
                            optional_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OptionalCursor => {
                            if optional_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalCursor"));
                            }
                            optional_cursor__ = map.next_value()?;
                        }
                    }
                }
                Ok(LookupResourcesRequest {
                    consistency: consistency__,
                    resource_object_type: resource_object_type__.unwrap_or_default(),
                    permission: permission__.unwrap_or_default(),
                    subject: subject__,
                    context: context__,
                    optional_limit: optional_limit__.unwrap_or_default(),
                    optional_cursor: optional_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.LookupResourcesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupResourcesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.looked_up_at.is_some() {
            len += 1;
        }
        if !self.resource_object_id.is_empty() {
            len += 1;
        }
        if self.permissionship != 0 {
            len += 1;
        }
        if self.partial_caveat_info.is_some() {
            len += 1;
        }
        if self.after_result_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.LookupResourcesResponse", len)?;
        if let Some(v) = self.looked_up_at.as_ref() {
            struct_ser.serialize_field("lookedUpAt", v)?;
        }
        if !self.resource_object_id.is_empty() {
            struct_ser.serialize_field("resourceObjectId", &self.resource_object_id)?;
        }
        if self.permissionship != 0 {
            let v = LookupPermissionship::from_i32(self.permissionship)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.permissionship)))?;
            struct_ser.serialize_field("permissionship", &v)?;
        }
        if let Some(v) = self.partial_caveat_info.as_ref() {
            struct_ser.serialize_field("partialCaveatInfo", v)?;
        }
        if let Some(v) = self.after_result_cursor.as_ref() {
            struct_ser.serialize_field("afterResultCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupResourcesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "looked_up_at",
            "lookedUpAt",
            "resource_object_id",
            "resourceObjectId",
            "permissionship",
            "partial_caveat_info",
            "partialCaveatInfo",
            "after_result_cursor",
            "afterResultCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LookedUpAt,
            ResourceObjectId,
            Permissionship,
            PartialCaveatInfo,
            AfterResultCursor,
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
                            "lookedUpAt" | "looked_up_at" => Ok(GeneratedField::LookedUpAt),
                            "resourceObjectId" | "resource_object_id" => Ok(GeneratedField::ResourceObjectId),
                            "permissionship" => Ok(GeneratedField::Permissionship),
                            "partialCaveatInfo" | "partial_caveat_info" => Ok(GeneratedField::PartialCaveatInfo),
                            "afterResultCursor" | "after_result_cursor" => Ok(GeneratedField::AfterResultCursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupResourcesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.LookupResourcesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LookupResourcesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut looked_up_at__ = None;
                let mut resource_object_id__ = None;
                let mut permissionship__ = None;
                let mut partial_caveat_info__ = None;
                let mut after_result_cursor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LookedUpAt => {
                            if looked_up_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lookedUpAt"));
                            }
                            looked_up_at__ = map.next_value()?;
                        }
                        GeneratedField::ResourceObjectId => {
                            if resource_object_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceObjectId"));
                            }
                            resource_object_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Permissionship => {
                            if permissionship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionship"));
                            }
                            permissionship__ = Some(map.next_value::<LookupPermissionship>()? as i32);
                        }
                        GeneratedField::PartialCaveatInfo => {
                            if partial_caveat_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partialCaveatInfo"));
                            }
                            partial_caveat_info__ = map.next_value()?;
                        }
                        GeneratedField::AfterResultCursor => {
                            if after_result_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("afterResultCursor"));
                            }
                            after_result_cursor__ = map.next_value()?;
                        }
                    }
                }
                Ok(LookupResourcesResponse {
                    looked_up_at: looked_up_at__,
                    resource_object_id: resource_object_id__.unwrap_or_default(),
                    permissionship: permissionship__.unwrap_or_default(),
                    partial_caveat_info: partial_caveat_info__,
                    after_result_cursor: after_result_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.LookupResourcesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupSubjectsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
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
        if self.context.is_some() {
            len += 1;
        }
        if self.optional_concrete_limit != 0 {
            len += 1;
        }
        if self.optional_cursor.is_some() {
            len += 1;
        }
        if self.wildcard_option != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.LookupSubjectsRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
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
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if self.optional_concrete_limit != 0 {
            struct_ser.serialize_field("optionalConcreteLimit", &self.optional_concrete_limit)?;
        }
        if let Some(v) = self.optional_cursor.as_ref() {
            struct_ser.serialize_field("optionalCursor", v)?;
        }
        if self.wildcard_option != 0 {
            let v = lookup_subjects_request::WildcardOption::from_i32(self.wildcard_option)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.wildcard_option)))?;
            struct_ser.serialize_field("wildcardOption", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupSubjectsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "resource",
            "permission",
            "subject_object_type",
            "subjectObjectType",
            "optional_subject_relation",
            "optionalSubjectRelation",
            "context",
            "optional_concrete_limit",
            "optionalConcreteLimit",
            "optional_cursor",
            "optionalCursor",
            "wildcard_option",
            "wildcardOption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            Resource,
            Permission,
            SubjectObjectType,
            OptionalSubjectRelation,
            Context,
            OptionalConcreteLimit,
            OptionalCursor,
            WildcardOption,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "resource" => Ok(GeneratedField::Resource),
                            "permission" => Ok(GeneratedField::Permission),
                            "subjectObjectType" | "subject_object_type" => Ok(GeneratedField::SubjectObjectType),
                            "optionalSubjectRelation" | "optional_subject_relation" => Ok(GeneratedField::OptionalSubjectRelation),
                            "context" => Ok(GeneratedField::Context),
                            "optionalConcreteLimit" | "optional_concrete_limit" => Ok(GeneratedField::OptionalConcreteLimit),
                            "optionalCursor" | "optional_cursor" => Ok(GeneratedField::OptionalCursor),
                            "wildcardOption" | "wildcard_option" => Ok(GeneratedField::WildcardOption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupSubjectsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.LookupSubjectsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LookupSubjectsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut resource__ = None;
                let mut permission__ = None;
                let mut subject_object_type__ = None;
                let mut optional_subject_relation__ = None;
                let mut context__ = None;
                let mut optional_concrete_limit__ = None;
                let mut optional_cursor__ = None;
                let mut wildcard_option__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
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
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map.next_value()?;
                        }
                        GeneratedField::OptionalConcreteLimit => {
                            if optional_concrete_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalConcreteLimit"));
                            }
                            optional_concrete_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OptionalCursor => {
                            if optional_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalCursor"));
                            }
                            optional_cursor__ = map.next_value()?;
                        }
                        GeneratedField::WildcardOption => {
                            if wildcard_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wildcardOption"));
                            }
                            wildcard_option__ = Some(map.next_value::<lookup_subjects_request::WildcardOption>()? as i32);
                        }
                    }
                }
                Ok(LookupSubjectsRequest {
                    consistency: consistency__,
                    resource: resource__,
                    permission: permission__.unwrap_or_default(),
                    subject_object_type: subject_object_type__.unwrap_or_default(),
                    optional_subject_relation: optional_subject_relation__.unwrap_or_default(),
                    context: context__,
                    optional_concrete_limit: optional_concrete_limit__.unwrap_or_default(),
                    optional_cursor: optional_cursor__,
                    wildcard_option: wildcard_option__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.LookupSubjectsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for lookup_subjects_request::WildcardOption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "WILDCARD_OPTION_UNSPECIFIED",
            Self::IncludeWildcards => "WILDCARD_OPTION_INCLUDE_WILDCARDS",
            Self::ExcludeWildcards => "WILDCARD_OPTION_EXCLUDE_WILDCARDS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for lookup_subjects_request::WildcardOption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "WILDCARD_OPTION_UNSPECIFIED",
            "WILDCARD_OPTION_INCLUDE_WILDCARDS",
            "WILDCARD_OPTION_EXCLUDE_WILDCARDS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = lookup_subjects_request::WildcardOption;

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
                    .and_then(lookup_subjects_request::WildcardOption::from_i32)
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
                    .and_then(lookup_subjects_request::WildcardOption::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "WILDCARD_OPTION_UNSPECIFIED" => Ok(lookup_subjects_request::WildcardOption::Unspecified),
                    "WILDCARD_OPTION_INCLUDE_WILDCARDS" => Ok(lookup_subjects_request::WildcardOption::IncludeWildcards),
                    "WILDCARD_OPTION_EXCLUDE_WILDCARDS" => Ok(lookup_subjects_request::WildcardOption::ExcludeWildcards),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LookupSubjectsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.looked_up_at.is_some() {
            len += 1;
        }
        if !self.subject_object_id.is_empty() {
            len += 1;
        }
        if !self.excluded_subject_ids.is_empty() {
            len += 1;
        }
        if self.permissionship != 0 {
            len += 1;
        }
        if self.partial_caveat_info.is_some() {
            len += 1;
        }
        if self.subject.is_some() {
            len += 1;
        }
        if !self.excluded_subjects.is_empty() {
            len += 1;
        }
        if self.after_result_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.LookupSubjectsResponse", len)?;
        if let Some(v) = self.looked_up_at.as_ref() {
            struct_ser.serialize_field("lookedUpAt", v)?;
        }
        if !self.subject_object_id.is_empty() {
            struct_ser.serialize_field("subjectObjectId", &self.subject_object_id)?;
        }
        if !self.excluded_subject_ids.is_empty() {
            struct_ser.serialize_field("excludedSubjectIds", &self.excluded_subject_ids)?;
        }
        if self.permissionship != 0 {
            let v = LookupPermissionship::from_i32(self.permissionship)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.permissionship)))?;
            struct_ser.serialize_field("permissionship", &v)?;
        }
        if let Some(v) = self.partial_caveat_info.as_ref() {
            struct_ser.serialize_field("partialCaveatInfo", v)?;
        }
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if !self.excluded_subjects.is_empty() {
            struct_ser.serialize_field("excludedSubjects", &self.excluded_subjects)?;
        }
        if let Some(v) = self.after_result_cursor.as_ref() {
            struct_ser.serialize_field("afterResultCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LookupSubjectsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "looked_up_at",
            "lookedUpAt",
            "subject_object_id",
            "subjectObjectId",
            "excluded_subject_ids",
            "excludedSubjectIds",
            "permissionship",
            "partial_caveat_info",
            "partialCaveatInfo",
            "subject",
            "excluded_subjects",
            "excludedSubjects",
            "after_result_cursor",
            "afterResultCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LookedUpAt,
            SubjectObjectId,
            ExcludedSubjectIds,
            Permissionship,
            PartialCaveatInfo,
            Subject,
            ExcludedSubjects,
            AfterResultCursor,
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
                            "lookedUpAt" | "looked_up_at" => Ok(GeneratedField::LookedUpAt),
                            "subjectObjectId" | "subject_object_id" => Ok(GeneratedField::SubjectObjectId),
                            "excludedSubjectIds" | "excluded_subject_ids" => Ok(GeneratedField::ExcludedSubjectIds),
                            "permissionship" => Ok(GeneratedField::Permissionship),
                            "partialCaveatInfo" | "partial_caveat_info" => Ok(GeneratedField::PartialCaveatInfo),
                            "subject" => Ok(GeneratedField::Subject),
                            "excludedSubjects" | "excluded_subjects" => Ok(GeneratedField::ExcludedSubjects),
                            "afterResultCursor" | "after_result_cursor" => Ok(GeneratedField::AfterResultCursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupSubjectsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.LookupSubjectsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LookupSubjectsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut looked_up_at__ = None;
                let mut subject_object_id__ = None;
                let mut excluded_subject_ids__ = None;
                let mut permissionship__ = None;
                let mut partial_caveat_info__ = None;
                let mut subject__ = None;
                let mut excluded_subjects__ = None;
                let mut after_result_cursor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LookedUpAt => {
                            if looked_up_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lookedUpAt"));
                            }
                            looked_up_at__ = map.next_value()?;
                        }
                        GeneratedField::SubjectObjectId => {
                            if subject_object_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectObjectId"));
                            }
                            subject_object_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExcludedSubjectIds => {
                            if excluded_subject_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("excludedSubjectIds"));
                            }
                            excluded_subject_ids__ = Some(map.next_value()?);
                        }
                        GeneratedField::Permissionship => {
                            if permissionship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionship"));
                            }
                            permissionship__ = Some(map.next_value::<LookupPermissionship>()? as i32);
                        }
                        GeneratedField::PartialCaveatInfo => {
                            if partial_caveat_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partialCaveatInfo"));
                            }
                            partial_caveat_info__ = map.next_value()?;
                        }
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map.next_value()?;
                        }
                        GeneratedField::ExcludedSubjects => {
                            if excluded_subjects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("excludedSubjects"));
                            }
                            excluded_subjects__ = Some(map.next_value()?);
                        }
                        GeneratedField::AfterResultCursor => {
                            if after_result_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("afterResultCursor"));
                            }
                            after_result_cursor__ = map.next_value()?;
                        }
                    }
                }
                Ok(LookupSubjectsResponse {
                    looked_up_at: looked_up_at__,
                    subject_object_id: subject_object_id__.unwrap_or_default(),
                    excluded_subject_ids: excluded_subject_ids__.unwrap_or_default(),
                    permissionship: permissionship__.unwrap_or_default(),
                    partial_caveat_info: partial_caveat_info__,
                    subject: subject__,
                    excluded_subjects: excluded_subjects__.unwrap_or_default(),
                    after_result_cursor: after_result_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.LookupSubjectsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ObjectReference {
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
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ObjectReference", len)?;
        if !self.object_type.is_empty() {
            struct_ser.serialize_field("objectType", &self.object_type)?;
        }
        if !self.object_id.is_empty() {
            struct_ser.serialize_field("objectId", &self.object_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObjectReference {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectType,
            ObjectId,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ObjectReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ObjectReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ObjectReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_type__ = None;
                let mut object_id__ = None;
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
                    }
                }
                Ok(ObjectReference {
                    object_type: object_type__.unwrap_or_default(),
                    object_id: object_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ObjectReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PartialCaveatInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.missing_required_context.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.PartialCaveatInfo", len)?;
        if !self.missing_required_context.is_empty() {
            struct_ser.serialize_field("missingRequiredContext", &self.missing_required_context)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PartialCaveatInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "missing_required_context",
            "missingRequiredContext",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MissingRequiredContext,
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
                            "missingRequiredContext" | "missing_required_context" => Ok(GeneratedField::MissingRequiredContext),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PartialCaveatInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.PartialCaveatInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PartialCaveatInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut missing_required_context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MissingRequiredContext => {
                            if missing_required_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("missingRequiredContext"));
                            }
                            missing_required_context__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PartialCaveatInfo {
                    missing_required_context: missing_required_context__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.PartialCaveatInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionRelationshipTree {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expanded_object.is_some() {
            len += 1;
        }
        if !self.expanded_relation.is_empty() {
            len += 1;
        }
        if self.tree_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.PermissionRelationshipTree", len)?;
        if let Some(v) = self.expanded_object.as_ref() {
            struct_ser.serialize_field("expandedObject", v)?;
        }
        if !self.expanded_relation.is_empty() {
            struct_ser.serialize_field("expandedRelation", &self.expanded_relation)?;
        }
        if let Some(v) = self.tree_type.as_ref() {
            match v {
                permission_relationship_tree::TreeType::Intermediate(v) => {
                    struct_ser.serialize_field("intermediate", v)?;
                }
                permission_relationship_tree::TreeType::Leaf(v) => {
                    struct_ser.serialize_field("leaf", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionRelationshipTree {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expanded_object",
            "expandedObject",
            "expanded_relation",
            "expandedRelation",
            "intermediate",
            "leaf",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExpandedObject,
            ExpandedRelation,
            Intermediate,
            Leaf,
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
                            "expandedObject" | "expanded_object" => Ok(GeneratedField::ExpandedObject),
                            "expandedRelation" | "expanded_relation" => Ok(GeneratedField::ExpandedRelation),
                            "intermediate" => Ok(GeneratedField::Intermediate),
                            "leaf" => Ok(GeneratedField::Leaf),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionRelationshipTree;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.PermissionRelationshipTree")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PermissionRelationshipTree, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expanded_object__ = None;
                let mut expanded_relation__ = None;
                let mut tree_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExpandedObject => {
                            if expanded_object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expandedObject"));
                            }
                            expanded_object__ = map.next_value()?;
                        }
                        GeneratedField::ExpandedRelation => {
                            if expanded_relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expandedRelation"));
                            }
                            expanded_relation__ = Some(map.next_value()?);
                        }
                        GeneratedField::Intermediate => {
                            if tree_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intermediate"));
                            }
                            tree_type__ = map.next_value::<::std::option::Option<_>>()?.map(permission_relationship_tree::TreeType::Intermediate)
;
                        }
                        GeneratedField::Leaf => {
                            if tree_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leaf"));
                            }
                            tree_type__ = map.next_value::<::std::option::Option<_>>()?.map(permission_relationship_tree::TreeType::Leaf)
;
                        }
                    }
                }
                Ok(PermissionRelationshipTree {
                    expanded_object: expanded_object__,
                    expanded_relation: expanded_relation__.unwrap_or_default(),
                    tree_type: tree_type__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.PermissionRelationshipTree", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Precondition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation != 0 {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.Precondition", len)?;
        if self.operation != 0 {
            let v = precondition::Operation::from_i32(self.operation)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.operation)))?;
            struct_ser.serialize_field("operation", &v)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Precondition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operation,
            Filter,
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
                            "operation" => Ok(GeneratedField::Operation),
                            "filter" => Ok(GeneratedField::Filter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Precondition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.Precondition")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Precondition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation__ = None;
                let mut filter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = Some(map.next_value::<precondition::Operation>()? as i32);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map.next_value()?;
                        }
                    }
                }
                Ok(Precondition {
                    operation: operation__.unwrap_or_default(),
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.Precondition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for precondition::Operation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OPERATION_UNSPECIFIED",
            Self::MustNotMatch => "OPERATION_MUST_NOT_MATCH",
            Self::MustMatch => "OPERATION_MUST_MATCH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for precondition::Operation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OPERATION_UNSPECIFIED",
            "OPERATION_MUST_NOT_MATCH",
            "OPERATION_MUST_MATCH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = precondition::Operation;

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
                    .and_then(precondition::Operation::from_i32)
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
                    .and_then(precondition::Operation::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OPERATION_UNSPECIFIED" => Ok(precondition::Operation::Unspecified),
                    "OPERATION_MUST_NOT_MATCH" => Ok(precondition::Operation::MustNotMatch),
                    "OPERATION_MUST_MATCH" => Ok(precondition::Operation::MustMatch),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReadCounterValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.relationship_count != 0 {
            len += 1;
        }
        if self.read_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ReadCounterValue", len)?;
        if self.relationship_count != 0 {
            struct_ser.serialize_field("relationshipCount", ToString::to_string(&self.relationship_count).as_str())?;
        }
        if let Some(v) = self.read_at.as_ref() {
            struct_ser.serialize_field("readAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadCounterValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relationship_count",
            "relationshipCount",
            "read_at",
            "readAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RelationshipCount,
            ReadAt,
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
                            "relationshipCount" | "relationship_count" => Ok(GeneratedField::RelationshipCount),
                            "readAt" | "read_at" => Ok(GeneratedField::ReadAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadCounterValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ReadCounterValue")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ReadCounterValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relationship_count__ = None;
                let mut read_at__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RelationshipCount => {
                            if relationship_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationshipCount"));
                            }
                            relationship_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ReadAt => {
                            if read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readAt"));
                            }
                            read_at__ = map.next_value()?;
                        }
                    }
                }
                Ok(ReadCounterValue {
                    relationship_count: relationship_count__.unwrap_or_default(),
                    read_at: read_at__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ReadCounterValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadRelationshipsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consistency.is_some() {
            len += 1;
        }
        if self.relationship_filter.is_some() {
            len += 1;
        }
        if self.optional_limit != 0 {
            len += 1;
        }
        if self.optional_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ReadRelationshipsRequest", len)?;
        if let Some(v) = self.consistency.as_ref() {
            struct_ser.serialize_field("consistency", v)?;
        }
        if let Some(v) = self.relationship_filter.as_ref() {
            struct_ser.serialize_field("relationshipFilter", v)?;
        }
        if self.optional_limit != 0 {
            struct_ser.serialize_field("optionalLimit", &self.optional_limit)?;
        }
        if let Some(v) = self.optional_cursor.as_ref() {
            struct_ser.serialize_field("optionalCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadRelationshipsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consistency",
            "relationship_filter",
            "relationshipFilter",
            "optional_limit",
            "optionalLimit",
            "optional_cursor",
            "optionalCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consistency,
            RelationshipFilter,
            OptionalLimit,
            OptionalCursor,
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
                            "consistency" => Ok(GeneratedField::Consistency),
                            "relationshipFilter" | "relationship_filter" => Ok(GeneratedField::RelationshipFilter),
                            "optionalLimit" | "optional_limit" => Ok(GeneratedField::OptionalLimit),
                            "optionalCursor" | "optional_cursor" => Ok(GeneratedField::OptionalCursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadRelationshipsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ReadRelationshipsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ReadRelationshipsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consistency__ = None;
                let mut relationship_filter__ = None;
                let mut optional_limit__ = None;
                let mut optional_cursor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = map.next_value()?;
                        }
                        GeneratedField::RelationshipFilter => {
                            if relationship_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationshipFilter"));
                            }
                            relationship_filter__ = map.next_value()?;
                        }
                        GeneratedField::OptionalLimit => {
                            if optional_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalLimit"));
                            }
                            optional_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OptionalCursor => {
                            if optional_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalCursor"));
                            }
                            optional_cursor__ = map.next_value()?;
                        }
                    }
                }
                Ok(ReadRelationshipsRequest {
                    consistency: consistency__,
                    relationship_filter: relationship_filter__,
                    optional_limit: optional_limit__.unwrap_or_default(),
                    optional_cursor: optional_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ReadRelationshipsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadRelationshipsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.read_at.is_some() {
            len += 1;
        }
        if self.relationship.is_some() {
            len += 1;
        }
        if self.after_result_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ReadRelationshipsResponse", len)?;
        if let Some(v) = self.read_at.as_ref() {
            struct_ser.serialize_field("readAt", v)?;
        }
        if let Some(v) = self.relationship.as_ref() {
            struct_ser.serialize_field("relationship", v)?;
        }
        if let Some(v) = self.after_result_cursor.as_ref() {
            struct_ser.serialize_field("afterResultCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadRelationshipsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "read_at",
            "readAt",
            "relationship",
            "after_result_cursor",
            "afterResultCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadAt,
            Relationship,
            AfterResultCursor,
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
                            "readAt" | "read_at" => Ok(GeneratedField::ReadAt),
                            "relationship" => Ok(GeneratedField::Relationship),
                            "afterResultCursor" | "after_result_cursor" => Ok(GeneratedField::AfterResultCursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadRelationshipsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ReadRelationshipsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ReadRelationshipsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut read_at__ = None;
                let mut relationship__ = None;
                let mut after_result_cursor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReadAt => {
                            if read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readAt"));
                            }
                            read_at__ = map.next_value()?;
                        }
                        GeneratedField::Relationship => {
                            if relationship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            relationship__ = map.next_value()?;
                        }
                        GeneratedField::AfterResultCursor => {
                            if after_result_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("afterResultCursor"));
                            }
                            after_result_cursor__ = map.next_value()?;
                        }
                    }
                }
                Ok(ReadRelationshipsResponse {
                    read_at: read_at__,
                    relationship: relationship__,
                    after_result_cursor: after_result_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ReadRelationshipsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadSchemaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("authzed.api.v1.ReadSchemaRequest", len)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ReadSchemaRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ReadSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReadSchemaRequest {
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ReadSchemaRequest", FIELDS, GeneratedVisitor)
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
        if !self.schema_text.is_empty() {
            len += 1;
        }
        if self.read_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ReadSchemaResponse", len)?;
        if !self.schema_text.is_empty() {
            struct_ser.serialize_field("schemaText", &self.schema_text)?;
        }
        if let Some(v) = self.read_at.as_ref() {
            struct_ser.serialize_field("readAt", v)?;
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
            "schema_text",
            "schemaText",
            "read_at",
            "readAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SchemaText,
            ReadAt,
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
                            "schemaText" | "schema_text" => Ok(GeneratedField::SchemaText),
                            "readAt" | "read_at" => Ok(GeneratedField::ReadAt),
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
                formatter.write_str("struct authzed.api.v1.ReadSchemaResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ReadSchemaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schema_text__ = None;
                let mut read_at__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SchemaText => {
                            if schema_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaText"));
                            }
                            schema_text__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReadAt => {
                            if read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readAt"));
                            }
                            read_at__ = map.next_value()?;
                        }
                    }
                }
                Ok(ReadSchemaResponse {
                    schema_text: schema_text__.unwrap_or_default(),
                    read_at: read_at__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ReadSchemaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Relationship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if self.subject.is_some() {
            len += 1;
        }
        if self.optional_caveat.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.Relationship", len)?;
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if let Some(v) = self.optional_caveat.as_ref() {
            struct_ser.serialize_field("optionalCaveat", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Relationship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource",
            "relation",
            "subject",
            "optional_caveat",
            "optionalCaveat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resource,
            Relation,
            Subject,
            OptionalCaveat,
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
                            "resource" => Ok(GeneratedField::Resource),
                            "relation" => Ok(GeneratedField::Relation),
                            "subject" => Ok(GeneratedField::Subject),
                            "optionalCaveat" | "optional_caveat" => Ok(GeneratedField::OptionalCaveat),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Relationship;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.Relationship")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Relationship, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                let mut relation__ = None;
                let mut subject__ = None;
                let mut optional_caveat__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map.next_value()?;
                        }
                        GeneratedField::OptionalCaveat => {
                            if optional_caveat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalCaveat"));
                            }
                            optional_caveat__ = map.next_value()?;
                        }
                    }
                }
                Ok(Relationship {
                    resource: resource__,
                    relation: relation__.unwrap_or_default(),
                    subject: subject__,
                    optional_caveat: optional_caveat__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.Relationship", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationshipFilter {
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
        if !self.optional_resource_id.is_empty() {
            len += 1;
        }
        if !self.optional_resource_id_prefix.is_empty() {
            len += 1;
        }
        if !self.optional_relation.is_empty() {
            len += 1;
        }
        if self.optional_subject_filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.RelationshipFilter", len)?;
        if !self.resource_type.is_empty() {
            struct_ser.serialize_field("resourceType", &self.resource_type)?;
        }
        if !self.optional_resource_id.is_empty() {
            struct_ser.serialize_field("optionalResourceId", &self.optional_resource_id)?;
        }
        if !self.optional_resource_id_prefix.is_empty() {
            struct_ser.serialize_field("optionalResourceIdPrefix", &self.optional_resource_id_prefix)?;
        }
        if !self.optional_relation.is_empty() {
            struct_ser.serialize_field("optionalRelation", &self.optional_relation)?;
        }
        if let Some(v) = self.optional_subject_filter.as_ref() {
            struct_ser.serialize_field("optionalSubjectFilter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationshipFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_type",
            "resourceType",
            "optional_resource_id",
            "optionalResourceId",
            "optional_resource_id_prefix",
            "optionalResourceIdPrefix",
            "optional_relation",
            "optionalRelation",
            "optional_subject_filter",
            "optionalSubjectFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceType,
            OptionalResourceId,
            OptionalResourceIdPrefix,
            OptionalRelation,
            OptionalSubjectFilter,
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
                            "optionalResourceId" | "optional_resource_id" => Ok(GeneratedField::OptionalResourceId),
                            "optionalResourceIdPrefix" | "optional_resource_id_prefix" => Ok(GeneratedField::OptionalResourceIdPrefix),
                            "optionalRelation" | "optional_relation" => Ok(GeneratedField::OptionalRelation),
                            "optionalSubjectFilter" | "optional_subject_filter" => Ok(GeneratedField::OptionalSubjectFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationshipFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.RelationshipFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RelationshipFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_type__ = None;
                let mut optional_resource_id__ = None;
                let mut optional_resource_id_prefix__ = None;
                let mut optional_relation__ = None;
                let mut optional_subject_filter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceType"));
                            }
                            resource_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalResourceId => {
                            if optional_resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalResourceId"));
                            }
                            optional_resource_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalResourceIdPrefix => {
                            if optional_resource_id_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalResourceIdPrefix"));
                            }
                            optional_resource_id_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalRelation => {
                            if optional_relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalRelation"));
                            }
                            optional_relation__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalSubjectFilter => {
                            if optional_subject_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalSubjectFilter"));
                            }
                            optional_subject_filter__ = map.next_value()?;
                        }
                    }
                }
                Ok(RelationshipFilter {
                    resource_type: resource_type__.unwrap_or_default(),
                    optional_resource_id: optional_resource_id__.unwrap_or_default(),
                    optional_resource_id_prefix: optional_resource_id_prefix__.unwrap_or_default(),
                    optional_relation: optional_relation__.unwrap_or_default(),
                    optional_subject_filter: optional_subject_filter__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.RelationshipFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationshipUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation != 0 {
            len += 1;
        }
        if self.relationship.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.RelationshipUpdate", len)?;
        if self.operation != 0 {
            let v = relationship_update::Operation::from_i32(self.operation)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.operation)))?;
            struct_ser.serialize_field("operation", &v)?;
        }
        if let Some(v) = self.relationship.as_ref() {
            struct_ser.serialize_field("relationship", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationshipUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation",
            "relationship",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operation,
            Relationship,
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
                            "operation" => Ok(GeneratedField::Operation),
                            "relationship" => Ok(GeneratedField::Relationship),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationshipUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.RelationshipUpdate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RelationshipUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation__ = None;
                let mut relationship__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = Some(map.next_value::<relationship_update::Operation>()? as i32);
                        }
                        GeneratedField::Relationship => {
                            if relationship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            relationship__ = map.next_value()?;
                        }
                    }
                }
                Ok(RelationshipUpdate {
                    operation: operation__.unwrap_or_default(),
                    relationship: relationship__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.RelationshipUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for relationship_update::Operation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OPERATION_UNSPECIFIED",
            Self::Create => "OPERATION_CREATE",
            Self::Touch => "OPERATION_TOUCH",
            Self::Delete => "OPERATION_DELETE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for relationship_update::Operation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OPERATION_UNSPECIFIED",
            "OPERATION_CREATE",
            "OPERATION_TOUCH",
            "OPERATION_DELETE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = relationship_update::Operation;

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
                    .and_then(relationship_update::Operation::from_i32)
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
                    .and_then(relationship_update::Operation::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OPERATION_UNSPECIFIED" => Ok(relationship_update::Operation::Unspecified),
                    "OPERATION_CREATE" => Ok(relationship_update::Operation::Create),
                    "OPERATION_TOUCH" => Ok(relationship_update::Operation::Touch),
                    "OPERATION_DELETE" => Ok(relationship_update::Operation::Delete),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ResolvedSubject {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subject_object_id.is_empty() {
            len += 1;
        }
        if self.permissionship != 0 {
            len += 1;
        }
        if self.partial_caveat_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ResolvedSubject", len)?;
        if !self.subject_object_id.is_empty() {
            struct_ser.serialize_field("subjectObjectId", &self.subject_object_id)?;
        }
        if self.permissionship != 0 {
            let v = LookupPermissionship::from_i32(self.permissionship)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.permissionship)))?;
            struct_ser.serialize_field("permissionship", &v)?;
        }
        if let Some(v) = self.partial_caveat_info.as_ref() {
            struct_ser.serialize_field("partialCaveatInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolvedSubject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject_object_id",
            "subjectObjectId",
            "permissionship",
            "partial_caveat_info",
            "partialCaveatInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubjectObjectId,
            Permissionship,
            PartialCaveatInfo,
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
                            "subjectObjectId" | "subject_object_id" => Ok(GeneratedField::SubjectObjectId),
                            "permissionship" => Ok(GeneratedField::Permissionship),
                            "partialCaveatInfo" | "partial_caveat_info" => Ok(GeneratedField::PartialCaveatInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolvedSubject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ResolvedSubject")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResolvedSubject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject_object_id__ = None;
                let mut permissionship__ = None;
                let mut partial_caveat_info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubjectObjectId => {
                            if subject_object_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectObjectId"));
                            }
                            subject_object_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Permissionship => {
                            if permissionship__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionship"));
                            }
                            permissionship__ = Some(map.next_value::<LookupPermissionship>()? as i32);
                        }
                        GeneratedField::PartialCaveatInfo => {
                            if partial_caveat_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partialCaveatInfo"));
                            }
                            partial_caveat_info__ = map.next_value()?;
                        }
                    }
                }
                Ok(ResolvedSubject {
                    subject_object_id: subject_object_id__.unwrap_or_default(),
                    permissionship: permissionship__.unwrap_or_default(),
                    partial_caveat_info: partial_caveat_info__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ResolvedSubject", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subject_type.is_empty() {
            len += 1;
        }
        if !self.optional_subject_id.is_empty() {
            len += 1;
        }
        if self.optional_relation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.SubjectFilter", len)?;
        if !self.subject_type.is_empty() {
            struct_ser.serialize_field("subjectType", &self.subject_type)?;
        }
        if !self.optional_subject_id.is_empty() {
            struct_ser.serialize_field("optionalSubjectId", &self.optional_subject_id)?;
        }
        if let Some(v) = self.optional_relation.as_ref() {
            struct_ser.serialize_field("optionalRelation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject_type",
            "subjectType",
            "optional_subject_id",
            "optionalSubjectId",
            "optional_relation",
            "optionalRelation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubjectType,
            OptionalSubjectId,
            OptionalRelation,
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
                            "subjectType" | "subject_type" => Ok(GeneratedField::SubjectType),
                            "optionalSubjectId" | "optional_subject_id" => Ok(GeneratedField::OptionalSubjectId),
                            "optionalRelation" | "optional_relation" => Ok(GeneratedField::OptionalRelation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.SubjectFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubjectFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject_type__ = None;
                let mut optional_subject_id__ = None;
                let mut optional_relation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubjectType => {
                            if subject_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectType"));
                            }
                            subject_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalSubjectId => {
                            if optional_subject_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalSubjectId"));
                            }
                            optional_subject_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalRelation => {
                            if optional_relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalRelation"));
                            }
                            optional_relation__ = map.next_value()?;
                        }
                    }
                }
                Ok(SubjectFilter {
                    subject_type: subject_type__.unwrap_or_default(),
                    optional_subject_id: optional_subject_id__.unwrap_or_default(),
                    optional_relation: optional_relation__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.SubjectFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for subject_filter::RelationFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.SubjectFilter.RelationFilter", len)?;
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for subject_filter::RelationFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = subject_filter::RelationFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.SubjectFilter.RelationFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<subject_filter::RelationFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(subject_filter::RelationFilter {
                    relation: relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.SubjectFilter.RelationFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.object.is_some() {
            len += 1;
        }
        if !self.optional_relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.SubjectReference", len)?;
        if let Some(v) = self.object.as_ref() {
            struct_ser.serialize_field("object", v)?;
        }
        if !self.optional_relation.is_empty() {
            struct_ser.serialize_field("optionalRelation", &self.optional_relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object",
            "optional_relation",
            "optionalRelation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Object,
            OptionalRelation,
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
                            "object" => Ok(GeneratedField::Object),
                            "optionalRelation" | "optional_relation" => Ok(GeneratedField::OptionalRelation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.SubjectReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubjectReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object__ = None;
                let mut optional_relation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = map.next_value()?;
                        }
                        GeneratedField::OptionalRelation => {
                            if optional_relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalRelation"));
                            }
                            optional_relation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SubjectReference {
                    object: object__,
                    optional_relation: optional_relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.SubjectReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.optional_object_types.is_empty() {
            len += 1;
        }
        if self.optional_start_cursor.is_some() {
            len += 1;
        }
        if !self.optional_relationship_filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.WatchRequest", len)?;
        if !self.optional_object_types.is_empty() {
            struct_ser.serialize_field("optionalObjectTypes", &self.optional_object_types)?;
        }
        if let Some(v) = self.optional_start_cursor.as_ref() {
            struct_ser.serialize_field("optionalStartCursor", v)?;
        }
        if !self.optional_relationship_filters.is_empty() {
            struct_ser.serialize_field("optionalRelationshipFilters", &self.optional_relationship_filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "optional_object_types",
            "optionalObjectTypes",
            "optional_start_cursor",
            "optionalStartCursor",
            "optional_relationship_filters",
            "optionalRelationshipFilters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OptionalObjectTypes,
            OptionalStartCursor,
            OptionalRelationshipFilters,
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
                            "optionalObjectTypes" | "optional_object_types" => Ok(GeneratedField::OptionalObjectTypes),
                            "optionalStartCursor" | "optional_start_cursor" => Ok(GeneratedField::OptionalStartCursor),
                            "optionalRelationshipFilters" | "optional_relationship_filters" => Ok(GeneratedField::OptionalRelationshipFilters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.WatchRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut optional_object_types__ = None;
                let mut optional_start_cursor__ = None;
                let mut optional_relationship_filters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OptionalObjectTypes => {
                            if optional_object_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalObjectTypes"));
                            }
                            optional_object_types__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalStartCursor => {
                            if optional_start_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalStartCursor"));
                            }
                            optional_start_cursor__ = map.next_value()?;
                        }
                        GeneratedField::OptionalRelationshipFilters => {
                            if optional_relationship_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalRelationshipFilters"));
                            }
                            optional_relationship_filters__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WatchRequest {
                    optional_object_types: optional_object_types__.unwrap_or_default(),
                    optional_start_cursor: optional_start_cursor__,
                    optional_relationship_filters: optional_relationship_filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.WatchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchResponse {
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
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.WatchResponse", len)?;
        if !self.updates.is_empty() {
            struct_ser.serialize_field("updates", &self.updates)?;
        }
        if let Some(v) = self.changes_through.as_ref() {
            struct_ser.serialize_field("changesThrough", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchResponse {
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
            type Value = WatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.WatchResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchResponse, V::Error>
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
                Ok(WatchResponse {
                    updates: updates__.unwrap_or_default(),
                    changes_through: changes_through__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.WatchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteRelationshipsRequest {
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
        if !self.optional_preconditions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.WriteRelationshipsRequest", len)?;
        if !self.updates.is_empty() {
            struct_ser.serialize_field("updates", &self.updates)?;
        }
        if !self.optional_preconditions.is_empty() {
            struct_ser.serialize_field("optionalPreconditions", &self.optional_preconditions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteRelationshipsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "updates",
            "optional_preconditions",
            "optionalPreconditions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Updates,
            OptionalPreconditions,
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
                            "optionalPreconditions" | "optional_preconditions" => Ok(GeneratedField::OptionalPreconditions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteRelationshipsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.WriteRelationshipsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WriteRelationshipsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut updates__ = None;
                let mut optional_preconditions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Updates => {
                            if updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updates"));
                            }
                            updates__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalPreconditions => {
                            if optional_preconditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalPreconditions"));
                            }
                            optional_preconditions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WriteRelationshipsRequest {
                    updates: updates__.unwrap_or_default(),
                    optional_preconditions: optional_preconditions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.WriteRelationshipsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteRelationshipsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.written_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.WriteRelationshipsResponse", len)?;
        if let Some(v) = self.written_at.as_ref() {
            struct_ser.serialize_field("writtenAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteRelationshipsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "written_at",
            "writtenAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WrittenAt,
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
                            "writtenAt" | "written_at" => Ok(GeneratedField::WrittenAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteRelationshipsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.WriteRelationshipsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WriteRelationshipsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut written_at__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WrittenAt => {
                            if written_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("writtenAt"));
                            }
                            written_at__ = map.next_value()?;
                        }
                    }
                }
                Ok(WriteRelationshipsResponse {
                    written_at: written_at__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.WriteRelationshipsResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.WriteSchemaRequest", len)?;
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
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
            type Value = WriteSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.WriteSchemaRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WriteSchemaRequest, V::Error>
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
                Ok(WriteSchemaRequest {
                    schema: schema__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.WriteSchemaRequest", FIELDS, GeneratedVisitor)
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
        if self.written_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.WriteSchemaResponse", len)?;
        if let Some(v) = self.written_at.as_ref() {
            struct_ser.serialize_field("writtenAt", v)?;
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
            "written_at",
            "writtenAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WrittenAt,
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
                            "writtenAt" | "written_at" => Ok(GeneratedField::WrittenAt),
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
                formatter.write_str("struct authzed.api.v1.WriteSchemaResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WriteSchemaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut written_at__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WrittenAt => {
                            if written_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("writtenAt"));
                            }
                            written_at__ = map.next_value()?;
                        }
                    }
                }
                Ok(WriteSchemaResponse {
                    written_at: written_at__,
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.WriteSchemaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZedToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authzed.api.v1.ZedToken", len)?;
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZedToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
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
                            "token" => Ok(GeneratedField::Token),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZedToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authzed.api.v1.ZedToken")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ZedToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ZedToken {
                    token: token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authzed.api.v1.ZedToken", FIELDS, GeneratedVisitor)
    }
}
