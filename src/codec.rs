// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{
    Arrow, Dataset, Ligature, LigatureError, Link, Node, PersistedLink, QueryResult, QueryTx,
    Vertex, WriteTx,
};

const DATASET_PREFIX: u8 = 0;

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
    let mut dataset_clone = dataset.clone();
    dataset_clone.remove(0);
    let name = std::str::from_utf8(&dataset_clone)
        .map_err(|_| LigatureError("Error checking for Dataset".to_string()))?;
    Dataset::new(name)
}

pub fn encode_vertex(vertex: &Vertex) -> Vec<u8> {
    match vertex {
        Vertex::Node(node) => encode_node(node),
        Vertex::StringLiteral(value) => encode_string(value),
        Vertex::BooleanLiteral(value) => encode_boolean(value),
        Vertex::LongLiteral(value) => encode_long(value),
        Vertex::FloatLiteral(value) => encode_float(value),
    }
}

pub fn decode_vertex(vertex: Vec<u8>) -> Vertex {
    todo!()
}

pub fn encode_node(node: &Node) -> Vec<u8> {
    todo!()
}

pub fn decode_node(node: Vec<u8>) -> Node {
    todo!()
}

pub fn encode_string(string: &String) -> Vec<u8> {
    todo!()
}

pub fn decode_string(string: Vec<u8>) -> String {
    todo!()
}

pub fn encode_boolean(boolean: &bool) -> Vec<u8> {
    todo!()
}

pub fn decode_boolean(node: Vec<u8>) -> bool {
    todo!()
}

pub fn encode_long(long: &i64) -> Vec<u8> {
    todo!()
}

pub fn decode_long(long: Vec<u8>) -> i64 {
    todo!()
}

pub fn encode_float(float: &f64) -> Vec<u8> {
    todo!()
}

pub fn decode_float(node: Vec<u8>) -> f64 {
    todo!()
}

pub fn encode_arrow(arrow: &Arrow) -> Vec<u8> {
    todo!()
}

pub fn decode_arrow(arrow: Vec<u8>) -> Arrow {
    todo!()
}

pub fn encode_link(link: &Link) -> Vec<u8> {
    todo!()
}

pub fn decode_link(link: Vec<u8>) -> Link {
    todo!()
}

pub fn encode_persisted_link(persisted_link: &PersistedLink) -> Vec<u8> {
    todo!()
}

pub fn decode_persisted_link(persisted_link: Vec<u8>) -> PersistedLink {
    todo!()
}
