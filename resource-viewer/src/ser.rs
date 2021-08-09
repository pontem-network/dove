#![allow(clippy::field_reassign_with_default)]

use serde::Serialize;
use resource_viewer::{AnnotatedMoveStruct, AnnotatedMoveValue};

#[cfg(feature = "json-schema")]
use schemars::{JsonSchema, schema_for, schema::RootSchema};
use move_core_types::identifier::Identifier;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_binary_format::file_format::AbilitySet;

#[cfg(feature = "json-schema")]
pub fn produce_json_schema() -> RootSchema {
    schema_for!(AnnotatedMoveStructExt)
}

#[derive(Serialize)]
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
pub struct AnnotatedMoveStructWrapper {
    /// Block number, current for the state
    pub height: String,

    #[serde(with = "AnnotatedMoveStructExt")]
    pub result: AnnotatedMoveStruct,
}

#[derive(Serialize)]
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[serde(remote = "resource_viewer::AnnotatedMoveStruct")]
struct AnnotatedMoveStructExt {
    #[serde(with = "schema_support::AbilitySetExt")]
    abilities: AbilitySet,
    #[serde(rename = "type")]
    #[cfg_attr(feature = "json-schema", serde(with = "schema_support::StructTagExt"))]
    type_: StructTag,
    #[cfg_attr(
        feature = "json-schema",
        schemars(schema_with = "schema_support::vec_identifier_annotated_move_value")
    )]
    #[serde(serialize_with = "vec_annotated_move_value_mapped::serialize")]
    value: Vec<(Identifier, AnnotatedMoveValue)>,
}

#[derive(Serialize)]
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[serde(remote = "resource_viewer::AnnotatedMoveValue")]
enum AnnotatedMoveValueExt {
    U8(u8),
    U64(u64),
    U128(u128),
    Bool(bool),
    Address(#[serde(with = "AccountAddressExt")] AccountAddress),
    Vector(
        #[serde(with = "schema_support::TypeTagExt")] TypeTag,
        #[cfg_attr(
            feature = "json-schema",
            schemars(schema_with = "schema_support::vec_annotated_move_value")
        )]
        #[serde(serialize_with = "vec_annotated_move_value::serialize")]
        Vec<AnnotatedMoveValue>,
    ),
    Bytes(Vec<u8>),
    Struct(#[serde(with = "AnnotatedMoveStructExt")] AnnotatedMoveStruct),
}

#[derive(Serialize)]
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[serde(remote = "AccountAddress")]
struct AccountAddressExt(
    #[serde(getter = "AccountAddressExt::ext_to_u8")] pub [u8; AccountAddress::LENGTH],
);
impl AccountAddressExt {
    pub fn ext_to_u8(addr: &AccountAddress) -> [u8; AccountAddress::LENGTH] {
        addr.to_u8()
    }
}

#[derive(Serialize)]
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[serde(remote = "Identifier")]
struct IdentifierExt(#[serde(getter = "Identifier::to_string")] pub String);

mod vec_annotated_move_value {
    use super::{AnnotatedMoveValue, AnnotatedMoveValueExt};
    use serde::{Serialize, Serializer};
    #[cfg(feature = "json-schema")]
    use schemars::JsonSchema;

    pub fn serialize<S>(vec: &[AnnotatedMoveValue], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[cfg_attr(feature = "json-schema", derive(JsonSchema))]
        struct Helper<'a>(#[serde(with = "AnnotatedMoveValueExt")] &'a AnnotatedMoveValue);

        vec.iter()
            .map(Helper)
            .collect::<Vec<_>>()
            .serialize(serializer)
    }
}

mod vec_annotated_move_value_mapped {
    use super::{AnnotatedMoveValue, AnnotatedMoveValueExt};
    use super::{Identifier, IdentifierExt};
    use serde::{Serialize, Serializer};
    #[cfg(feature = "json-schema")]
    use schemars::JsonSchema;

    pub fn serialize<S>(
        vec: &[(Identifier, AnnotatedMoveValue)],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[cfg_attr(feature = "json-schema", derive(JsonSchema))]
        struct Helper<'a> {
            #[serde(with = "IdentifierExt")]
            id: &'a Identifier,
            #[serde(with = "AnnotatedMoveValueExt")]
            value: &'a AnnotatedMoveValue,
        }

        vec.iter()
            .map(|(id, value)| Helper { id, value })
            .collect::<Vec<_>>()
            .serialize(serializer)
    }
}

#[cfg(feature = "json-schema")]
mod schema_support {
    use super::*;
    use schemars::{
        gen::SchemaGenerator,
        schema::{ArrayValidation, InstanceType, Schema, SchemaObject, SingleOrVec},
    };
    use move_core_types::language_storage::TypeTag;
    use move_binary_format::file_format::AbilitySet;

    #[derive(Serialize, JsonSchema)]
    #[serde(remote = "AbilitySet")]
    pub struct AbilitySetExt(#[serde(getter = "AbilitySetExt::ext_to_u8")] pub u8);

    impl AbilitySetExt {
        pub fn ext_to_u8(ability: &AbilitySet) -> u8 {
            ability.into_u8()
        }
    }

    #[derive(Serialize, JsonSchema)]
    #[serde(remote = "StructTag")]
    pub struct StructTagExt {
        #[serde(with = "AccountAddressExt")]
        pub address: AccountAddress,
        #[serde(with = "IdentifierExt")]
        pub module: Identifier,
        #[serde(with = "IdentifierExt")]
        pub name: Identifier,
        #[schemars(schema_with = "vec_type_tag")]
        pub type_params: Vec<TypeTag>,
    }

    #[derive(Serialize, JsonSchema)]
    #[serde(remote = "TypeTag")]
    pub enum TypeTagExt {
        Bool,
        U8,
        U64,
        U128,
        Address,
        Signer,
        Vector(#[serde(with = "schema_support::TypeTagExt")] Box<TypeTag>),
        Struct(#[serde(with = "schema_support::StructTagExt")] StructTag),
    }

    pub fn vec_type_tag(gen: &mut SchemaGenerator) -> Schema {
        schema_vec::<TypeTagExt>(gen)
    }

    pub fn vec_annotated_move_value(gen: &mut SchemaGenerator) -> Schema {
        schema_vec::<AnnotatedMoveValueExt>(gen)
    }

    pub fn vec_identifier_annotated_move_value(gen: &mut SchemaGenerator) -> Schema {
        schema_vec::<(IdentifierExt, AnnotatedMoveValueExt)>(gen)
    }

    pub fn schema_vec<T: JsonSchema>(gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(InstanceType::Array.into())),
            array: Some(
                ArrayValidation {
                    items: Some(SingleOrVec::Single(gen.subschema_for::<T>().into())),
                    ..Default::default()
                }
                .into(),
            ),
            ..Default::default()
        })
    }
}
