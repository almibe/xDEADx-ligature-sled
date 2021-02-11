// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{
    Attribute, Dataset, Entity, Ligature, LigatureError, PersistedStatement, QueryTx, Statement,
    Value, WriteTx,
};
use std::collections::HashMap;

pub const DATASET_PREFIX: u8 = 0;
pub const ENTITY_ID_COUNTER_KEY: u8 = 1;
pub const ATTRIBUTE_ID_COUNTER_KEY: u8 = 2;
pub const ATTRIBUTE_NAME_TO_ID_PREFIX: u8 = 3;
pub const ATTRIBUTE_ID_TO_NAME_PREFIX: u8 = 4;
pub const EAVC_PREFIX: u8 = 5;
pub const EVAC_PREFIX: u8 = 6;
pub const AEVC_PREFIX: u8 = 7;
pub const AVEC_PREFIX: u8 = 8;
pub const VEAC_PREFIX: u8 = 9;
pub const VAEC_PREFIX: u8 = 10;
pub const CEAV_PREFIX: u8 = 11;
pub const STRING_LITERAL_ID_COUNTER_KEY: u8 = 12;
pub const STRING_LITERAL_VALUE_TO_ID_PREFIX: u8 = 13;
pub const STRING_LITERAL_ID_TO_VALUE_PREFIX: u8 = 14;

pub const ENTITY_VALUE_PREFIX: u8 = 0;
pub const STRING_VALUE_PREFIX: u8 = 1;
pub const INTEGER_VALUE_PREFIX: u8 = 2;
pub const FLOAT_VALUE_PREFIX: u8 = 3;

pub enum EncodedStatementKey {
    EAVC,
    EVAC,
    AEVC,
    AVEC,
    VEAC,
    VAEC,
    CEAV,
}

pub struct StatementIDSet {
    pub entity_id: u64,
    pub attribute_id: u64,
    pub value_prefix: u8,
    pub value_id: u64,
    pub context_id: u64,
}

/// Prepends the given prefix to the given vector.
pub fn prepend(prefix: u8, vector: Vec<u8>) -> Vec<u8> {
    let mut res = vec![prefix];
    res.extend(vector);
    res
}

/// Removes the first value from a Vec<u8> and checks that it's the expected value.
pub fn chomp_assert(prefix: u8, vector: Vec<u8>) -> Result<Vec<u8>, LigatureError> {
    let mut vector_clone = vector;
    let vec_prefix = vector_clone.remove(0);
    if prefix == vec_prefix {
        Ok(vector_clone)
    } else {
        Err(LigatureError(format!("Expected prefix {}, actual prefix {}.", prefix, vec_prefix)))
    }
}

/// Encodes a Dataset's Name.
pub fn encode_dataset(dataset: &Dataset) -> Vec<u8> {
    dataset.name().as_bytes().to_vec()
}

/// Encodes a str prefix used to match Dataset names.
/// This is needed since a Dataset prefix might not be a valid Dataset name.
pub fn encode_dataset_match(dataset_match: &str) -> Vec<u8> {
    dataset_match.as_bytes().to_vec()
}

/// Decodes a Dataset's name and returns a Dataset.
pub fn decode_dataset(dataset: Vec<u8>) -> Result<Dataset, LigatureError> {
    let name = std::str::from_utf8(&dataset)
        .map_err(|_| LigatureError("Error checking for Dataset".to_string()))?;
    Dataset::new(name)
}

/// Encodes the value stored for an id.
pub fn encode_id(id: u64) -> Vec<u8> {
    id.to_be_bytes().to_vec()
}

/// Decodes the value stored for an id.
pub fn decode_id(id: Vec<u8>) -> Result<u64, LigatureError> {
    if id.len() == 8 {
        Ok(u64::from_be_bytes([
            id[0], id[1], id[2], id[3], id[4], id[5], id[6], id[7],
        ]))
    } else {
        Err(LigatureError(format!("Could not convert {:?} to u64", id)))
    }
}

pub fn encode_entity(entity: &Entity) -> Vec<u8> {
    todo!()
}

pub fn decode_entity(entity: Vec<u8>) -> Entity {
    todo!()
}

pub fn encode_attribute(attribute: &Attribute) -> Vec<u8> {
    attribute.name().as_bytes().to_vec()
}

pub fn decode_attribute(attribute: Vec<u8>) -> Attribute {
    todo!()
}

/// Encodes a value by delegating to the specific impl.
pub fn encode_value(value: &Value) -> Vec<u8> {
    match value {
        Value::Entity(entity) => encode_entity(entity),
        Value::StringLiteral(value) => encode_string_literal(value),
        Value::IntegerLiteral(value) => encode_integer_literal(value),
        Value::FloatLiteral(value) => encode_float_literal(value),
    }
}

pub fn decode_value(value: Vec<u8>) -> Value {
    todo!()
}

/// Encodes a String Literal and prefixes it with STRING_VALUE_PREFIX.
pub fn encode_string_literal(string: &str) -> Vec<u8> {
    string.as_bytes().to_vec()
}

pub fn decode_string_literal(string: Vec<u8>) -> String {
    todo!()
}

pub fn encode_integer_literal(integer: &i64) -> Vec<u8> {
    todo!()
}

pub fn decode_integer_literal(integer: Vec<u8>) -> i64 {
    todo!()
}

pub fn encode_float_literal(float: &f64) -> Vec<u8> {
    todo!()
}

pub fn decode_float_literal(entity: Vec<u8>) -> f64 {
    todo!()
}

/// Accepts a StatementIDs struct and returns a map with 7 entries, one for each of the different permutations.
pub fn encode_statement(statement_ids: &StatementIDSet) -> HashMap<EncodedStatementKey, Vec<u8>> {
    todo!()
}

/// Accepts an EncodedStatementKey and an encoded statement and uses that to break up the parts into a StatementIDSet.
pub fn decode_statement(statement: (EncodedStatementKey, Vec<u8>)) -> StatementIDSet {
    todo!()
}

// pub fn encode_persisted_statement(persisted_statement: &PersistedStatement) -> Vec<u8> {
//     todo!()
// }

// pub fn decode_persisted_statement(persisted_statement: Vec<u8>) -> PersistedStatement {
//     todo!()
// }
