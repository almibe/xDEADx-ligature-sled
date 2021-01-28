// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod codec;

use ligature::{
    BlankNode, Dataset, Ligature, LigatureError, QueryResult, QueryTx, Statement, WriteTx,
};

use codec::decode_dataset;

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
                    match decode_dataset(dataset.0.to_vec()) {
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
        //check if dataset exists
        //add dataset to default tree
        //create new tree
        todo!()
    }

    fn delete_dataset(&self, dataset: Dataset) -> Result<(), LigatureError> {
        //check if dataset exists
        //remove dataset from default tree
        //drop dataset's tree
        todo!()
    }

    fn query(&self, dataset: Dataset) -> Result<Box<dyn QueryTx>, LigatureError> {
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
    fn all_statements(&self) -> Box<dyn Iterator<Item = Result<Statement, LigatureError>>> {
        todo!()
    }

    fn sparql_query(&self, query: String) -> Result<QueryResult, LigatureError> {
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
    fn new_blank_node(&self) -> Result<BlankNode, LigatureError> {
        todo!()
    }

    fn add_statement(&self, statement: Statement) -> Result<Statement, LigatureError> {
        todo!()
    }

    fn remove_statement(&self, statement: Statement) -> Result<Statement, LigatureError> {
        todo!()
    }

    fn cancel(&self) -> Result<(), LigatureError> {
        todo!()
    }

    fn commit(&self) -> Result<(), LigatureError> {
        todo!()
    }
}
