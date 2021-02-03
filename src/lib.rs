// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod codec;

use ligature::{
    Dataset, Ligature, LigatureError, Vertex, Node, Range, Arrow, QueryResult, QueryTx, WriteTx, Link, PersistedLink,
};

use codec::{encode_dataset, decode_dataset};

pub struct LigatureSled {
    store: sled::Db,
}

impl LigatureSled {
    pub fn new(path: String) -> Result<Self, sled::Error> {
        Ok(Self {
            store: sled::open(path)?,
        })
    }

    pub fn temp(path: Option<String>) -> Result<Self, sled::Error> {
        match path {
            None => Ok(Self {
                store: sled::Config::default().temporary(true).open()?,
            }),
            Some(p) => Ok(Self {
                store: sled::Config::default().temporary(true).path(p).open()?,
            }),
        }
    }

    pub fn from_config(config: sled::Config) -> Result<Self, sled::Error> {
        Ok(Self {
            store: config.open()?,
        })
    }
}

impl Ligature for LigatureSled {
    fn all_datasets(&self) -> Box<dyn Iterator<Item = Result<Dataset, LigatureError>>> {
        let iter = self.store.iter();
        Box::new(iter.map(|ds| {
            match ds {
                Ok(dataset) => {
                    match decode_dataset(dataset.0.to_vec()) { //TODO use map_err here
                        Ok(d) => Ok(d),
                        Err(err) => Err(LigatureError("Error decoding dataset.".to_string())),
                    }
                },
                Err(_) => Err(LigatureError("Error iterating Datasets.".to_string())),
            }
        }))
    }

    fn match_datasets(&self, prefix: &str) -> Box<dyn Iterator<Item = Result<Dataset, LigatureError>>> {
        todo!()
    }

    fn match_datasets_range(&self, from: &str, to: &str) -> Box<dyn Iterator<Item = Result<Dataset, LigatureError>>> {
        todo!()
    }

    fn create_dataset(&self, dataset: Dataset) -> Result<(), LigatureError> {
        let encoded_dataset = encode_dataset(&dataset).map_err(|_| LigatureError("Error checking for Dataset".to_string()))?;
        let exists = self.store.contains_key(&encoded_dataset).map_err(|_| LigatureError("Error checking for Dataset".to_string()))?;
        if !exists {
            self.store.insert(encoded_dataset, vec![]); //TODO error check here -- probably just map_err and ?
            self.store.open_tree(dataset.name());
        }
        Ok(())
    }

    fn delete_dataset(&self, dataset: Dataset) -> Result<(), LigatureError> {
        let encoded_dataset = encode_dataset(&dataset).map_err(|_| LigatureError("Error checking for Dataset".to_string()))?;
        let exists = self.store.contains_key(&encoded_dataset).map_err(|_| LigatureError("Error checking for Dataset".to_string()))?;
        if exists {
            self.store.remove(&encoded_dataset);
            self.store.drop_tree(dataset.name());
        }
        Ok(())
    }

    fn query(&self, dataset: Dataset) -> Result<Box<dyn QueryTx>, LigatureError> {
        //TODO this method should start a readtx in sled
        //TODO this should check the subtree exists
        //TODO pass only the subtree to LigatureSledQueryTx
        Ok(Box::new(LigatureSledQueryTx {
            store: self.store.clone()
        }))
    }

    fn write(&self, dataset: Dataset) -> Result<Box<dyn WriteTx>, LigatureError> {
        Ok(Box::new(LigatureSledWriteTx {
            store: self.store.clone()
        }))
    }
}

struct LigatureSledQueryTx {
    store: sled::Db,
}

impl QueryTx for LigatureSledQueryTx {
    fn all_links(&self) -> Box<dyn Iterator<Item = Result<PersistedLink, LigatureError>>> {
        //check dataset exists
        //
        todo!()
    }

    /// Returns all PersistedLinks that match the given criteria.
    /// If a parameter is None then it matches all, so passing all Nones is the same as calling all_statements.
    fn match_links(
        &self,
        source: Option<Vertex>,
        arrow: Option<Arrow>,
        target: Option<Vertex>,
    ) -> Box<dyn Iterator<Item = Result<PersistedLink, LigatureError>>> {
        todo!()
    }

    /// Retuns all PersistedLinks that match the given criteria.
    /// If a parameter is None then it matches all.
    fn match_links_range(
        &self,
        source: Option<Vertex>,
        arrow: Option<Arrow>,
        target: Range,
    ) -> Box<dyn Iterator<Item = Result<PersistedLink, LigatureError>>> {
        todo!()
    }

    /// Returns the PersistedLink for the given context.
    fn link_for_context(&self, context: Node) -> Result<PersistedLink, LigatureError> {
        todo!()
    }

    fn wander_query(&self, query: String) -> Result<QueryResult, LigatureError> {
        todo!()
    }
}

struct LigatureSledWriteTx {
    store: sled::Db,
}

impl WriteTx for LigatureSledWriteTx {
    fn new_node(&self) -> Result<Node, LigatureError> {
        todo!()
    }

    fn add_link(&self, link: Link) -> Result<PersistedLink, LigatureError> {
        todo!()
    }

    fn remove_link(&self, persisted_link: PersistedLink) -> Result<(), LigatureError> {
        todo!()
    }

    fn cancel(&self) -> Result<(), LigatureError> {
        todo!()
    }

    fn commit(&self) -> Result<(), LigatureError> {
        todo!()
    }
}
