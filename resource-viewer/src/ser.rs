#![allow(clippy::field_reassign_with_default)]

use libra::rv;
use libra::prelude::*;
use libra::move_core_types::language_storage::StructTag;
use libra::account::Identifier;
use rv::{AnnotatedMoveStruct, AnnotatedMoveValue};
use serde::Serialize;
use schemars::{JsonSchema, schema_for};
use schemars::schema::RootSchema;

pub fn produce_json_schema() -> RootSchema {
    schema_for!(AnnotatedMoveStructExt)
}

#[derive(Serialize, JsonSchema)]
pub struct AnnotatedMoveStructWrapper {
    /// Block number, current for the state
    pub height: u128,

    #[serde(with = "AnnotatedMoveStructExt")]
    pub result: AnnotatedMoveStruct,
}

#[derive(Serialize, JsonSchema)]
#[serde(remote = "rv::AnnotatedMoveStruct")]
struct AnnotatedMoveStructExt {
    is_resource: bool,
    #[serde(rename = "type")]
    #[serde(with = "schema_support::StructTagExt")]
    type_: StructTag,
    #[schemars(schema_with = "schema_support::vec_identifier_annotated_move_value")]
    #[serde(serialize_with = "vec_annotated_move_value_mapped::serialize")]
    value: Vec<(Identifier, AnnotatedMoveValue)>,
}

#[derive(Serialize, JsonSchema)]
#[serde(remote = "rv::AnnotatedMoveValue")]
enum AnnotatedMoveValueExt {
    U8(u8),
    U64(u64),
    U128(u128),
    Bool(bool),
    Address(#[serde(with = "AccountAddressExt")] AccountAddress),
    Vector(
        // #[serde(with = "AnnotatedMoveValueExt")]
        #[schemars(schema_with = "schema_support::vec_annotated_move_value")]
        #[serde(serialize_with = "vec_annotated_move_value::serialize")]
        Vec<AnnotatedMoveValue>,
    ),
    Bytes(Vec<u8>),
    Struct(#[serde(with = "AnnotatedMoveStructExt")] AnnotatedMoveStruct),
}

#[derive(Serialize, JsonSchema)]
#[serde(remote = "AccountAddress")]
struct AccountAddressExt(
    #[serde(getter = "AccountAddressExt::ext_to_u8")] pub [u8; AccountAddress::LENGTH],
);
impl AccountAddressExt {
    pub fn ext_to_u8(addr: &AccountAddress) -> [u8; AccountAddress::LENGTH] {
        addr.to_u8()
    }
}

#[derive(Serialize, JsonSchema)]
#[serde(remote = "Identifier")]
struct IdentifierExt(#[serde(getter = "Identifier::to_string")] pub String);

mod vec_annotated_move_value {
    use super::{AnnotatedMoveValue, AnnotatedMoveValueExt};
    use serde::{Serialize, Serializer};
    use schemars::JsonSchema;

    pub fn serialize<S>(vec: &[AnnotatedMoveValue], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize, JsonSchema)]
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
    use schemars::JsonSchema;

    pub fn serialize<S>(
        vec: &[(Identifier, AnnotatedMoveValue)],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize, JsonSchema)]
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

mod schema_support {
    use super::*;
    use schemars::{
        gen::SchemaGenerator,
        schema::{ArrayValidation, InstanceType, Schema, SchemaObject, SingleOrVec},
    };

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
