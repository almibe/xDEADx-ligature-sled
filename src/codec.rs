// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{
    Attribute, Dataset, Ligature, LigatureError, Statement, Entity, PersistedStatement, QueryTx,
    WriteTx, Value,
};

pub const DATASET_PREFIX: u8 = 0;
pub const DATASET_COUNTER: u8 = 0;

pub fn encode_dataset(dataset: &Dataset) -> Vec<u8> {
    let encoded_dataset = dataset.name().as_bytes();
    let mut res = vec![DATASET_PREFIX];
    res.extend(encoded_dataset);
    res
}

pub fn encode_dataset_match(dataset_match: &str) -> Vec<u8> {
    let encoded_dataset = dataset_match.as_bytes();
    let mut res = vec![DATASET_PREFIX];
    res.extend(encoded_dataset);
    res
}

pub fn decode_dataset(dataset: Vec<u8>) -> Result<Dataset, LigatureError> {
    let mut dataset_clone = dataset;
    dataset_clone.remove(0);
    let name = std::str::from_utf8(&dataset_clone)
        .map_err(|_| LigatureError("Error checking for Dataset".to_string()))?;
    Dataset::new(name)
}

pub fn encode_value(value: &Value) -> Vec<u8> {
    match value {
        Value::Entity(entity) => encode_entity(entity),
        Value::StringLiteral(value) => encode_string(value),
        Value::LongLiteral(value) => encode_long(value),
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

// pub fn encode_attribute(attribute: &Attribute) -> Vec<u8> {
//     todo!()
// }

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
