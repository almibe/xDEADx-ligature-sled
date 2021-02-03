// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{
    Dataset, Ligature, LigatureError, Link, PersistedLink, Node, Arrow, Vertex, QueryResult, QueryTx, WriteTx,
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
    let mut dataset_clone = dataset.clone();
    dataset_clone.remove(0);
    bincode::deserialize(&dataset_clone)
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

pub fn encode_vertex(vertex: &Vertex) -> Vec<u8> {
    todo!()
}

pub fn decode_vertex(vertex: Vec<u8>) -> Vertex {
    todo!()
}

pub fn encode_arrow(arrow: &Arrow) -> Vec<u8> {
    todo!()
}

pub fn decode_arrow(arrow: Vec<u8>) -> Arrow {
    todo!()
}

pub fn encode_node(node: &Node) -> Vec<u8> {
    todo!()
}

pub fn decode_node(node: Vec<u8>) -> Node {
    todo!()
}
