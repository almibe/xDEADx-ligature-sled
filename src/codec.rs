// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{
    BlankNode, Dataset, Ligature, LigatureError, QueryResult, QueryTx, Statement, WriteTx, Graph, Object, Predicate, Subject,
};

use bincode::Error;

const DATASET_PREFIX: u8 = 0;

pub fn encode_dataset(dataset: &Dataset) -> Result<Vec<u8>, Error> {
    let encoded_dataset = bincode::serialize(dataset.name())?;
    let mut res = vec!(DATASET_PREFIX);
    res.extend(&encoded_dataset);
    Ok(res)
}

pub fn decode_dataset(dataset: Vec<u8>) -> Result<Dataset, Error> {
    bincode::deserialize(&dataset)
}

pub fn encode_statement(statement: &Statement) -> Vec<u8> {
    todo!()
}

pub fn decode_statement(statement: Vec<u8>) -> Statement {
    todo!()
}

pub fn encode_subject(subject: &Subject) -> Vec<u8> {
    todo!()
}

pub fn decode_subject(subject: Vec<u8>) -> Subject {
    todo!()
}

pub fn encode_predicate(predicate: &Predicate) -> Vec<u8> {
    todo!()
}

pub fn decode_predicate(predicate: Vec<u8>) -> Predicate {
    todo!()
}

pub fn encode_object(object: &Object) -> Vec<u8> {
    todo!()
}

pub fn decode_object(object: Vec<u8>) -> Object {
    todo!()
}

pub fn encode_graph(graph: &Graph) -> Vec<u8> {
    todo!()
}

pub fn decode_graph(graph: Vec<u8>) -> Graph {
    todo!()
}

