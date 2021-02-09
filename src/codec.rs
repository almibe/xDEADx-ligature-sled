// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{
    Attribute, Dataset, Entity, Ligature, LigatureError, PersistedStatement, QueryTx, Statement,
    Value, WriteTx,
};

pub const DATASET_PREFIX: u8 = 0;
pub const ENTITY_COUNTER_PREFIX: u8 = 1;
pub const ATTRIBUTE_COUNTER_PREFIX: u8 = 2;
pub const ATTRIBUTE_NAME_TO_ID_PREFIX: u8 = 3;

/// Takes a Dataset and encodes it with the DATASET_PREFIX.
pub fn encode_dataset(dataset: &Dataset) -> Vec<u8> {
    let encoded_dataset = dataset.name().as_bytes();
    let mut res = vec![DATASET_PREFIX];
    res.extend(encoded_dataset);
    res
}

/// Encodes a str prefix used to match Dataset names w/ the DATASET_PREFIX.
/// This is needed since a Dataset prefix might not be a valid Dataset name.
pub fn encode_dataset_match(dataset_match: &str) -> Vec<u8> {
    let encoded_dataset = dataset_match.as_bytes();
    let mut res = vec![DATASET_PREFIX];
    res.extend(encoded_dataset);
    res
}

/// Decodes a Dataset's name (ignoring the DATASET_PREFIX) and returns a Dataset.
/// TODO maybe this should return an error if the prefix isn't a DATASET_PREFIX.
pub fn decode_dataset(dataset: Vec<u8>) -> Result<Dataset, LigatureError> {
    let mut dataset_clone = dataset;
    dataset_clone.remove(0);
    let name = std::str::from_utf8(&dataset_clone)
        .map_err(|_| LigatureError("Error checking for Dataset".to_string()))?;
    Dataset::new(name)
}

/// Encodes the value stored for an id.
pub fn encode_id(id: u64) -> Vec<u8> {
    id.to_be_bytes().to_vec()
}

/// Decodes the value stored for an id.
pub fn decode_id(id: Vec<u8>) -> Result<u64, LigatureError> {
    if (id.len() == 8) {
        Ok(u64::from_be_bytes([
            id[0], id[1], id[2], id[3], id[4], id[5], id[6],
            id[7],
        ]))
    } else {
        Err(LigatureError(format!(
            "Could not convert {:?} to u64",
            id
        )))
    }
}

/// Encodes a value with the fitting prefix.
/// This function just delegates to the specific impl.
pub fn encode_value(value: &Value) -> Vec<u8> {
    match value {
        Value::Entity(entity) => encode_entity(entity),
        Value::StringLiteral(value) => encode_string(value),
        Value::IntegerLiteral(value) => encode_long(value),
        Value::FloatLiteral(value) => encode_float(value),
    }
}

// pub fn decode_value(value: Vec<u8>) -> Value {
//     todo!()
// }

pub fn encode_entity(entity: &Entity) -> Vec<u8> {
    todo!()
}

// pub fn decode_entity(entity: Vec<u8>) -> Entity {
//     todo!()
// }

pub fn encode_string(string: &str) -> Vec<u8> {
    todo!()
}

// pub fn decode_string(string: Vec<u8>) -> String {
//     todo!()
// }

pub fn encode_long(long: &i64) -> Vec<u8> {
    todo!()
}

// pub fn decode_long(long: Vec<u8>) -> i64 {
//     todo!()
// }

pub fn encode_float(float: &f64) -> Vec<u8> {
    todo!()
}

// pub fn decode_float(entity: Vec<u8>) -> f64 {
//     todo!()
// }

/// Enocdes an Attribute with the Attribute's name and the ATTRIBUTE_NAME_TO_ID_PREFIX.
pub fn encode_attribute(attribute: &Attribute) -> Vec<u8> {
    let encoded_attribute = attribute.name().as_bytes();
    let mut res = vec![ATTRIBUTE_NAME_TO_ID_PREFIX];
    res.extend(encoded_attribute);
    res
}

// pub fn decode_attribute(attribute: Vec<u8>) -> Attribute {
//     todo!()
// }

// pub fn encode_statement(statement: &Statement) -> Vec<u8> {
//     todo!()
// }

// pub fn decode_statement(statement: Vec<u8>) -> Statement {
//     todo!()
// }

// pub fn encode_persisted_statement(persisted_statement: &PersistedStatement) -> Vec<u8> {
//     todo!()
// }

// pub fn decode_persisted_statement(persisted_statement: Vec<u8>) -> PersistedStatement {
//     todo!()
// }
