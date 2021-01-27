// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{
    BlankNode, Dataset, Ligature, LigatureError, QueryResult, QueryTx, Statement, WriteTx,
};

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
    fn all_datasets(&self) -> Box<dyn Iterator<Item = Dataset>> {
        todo!()
    }

    fn match_datasets(&self, prefix: &str) -> Box<dyn Iterator<Item = Dataset>> {
        todo!()
    }

    fn match_datasets_range(&self, from: &str, to: &str) -> Box<dyn Iterator<Item = Dataset>> {
        todo!()
    }

    fn create_dataset(&self, dataset: Dataset) -> Result<(), LigatureError> {
        todo!()
    }

    fn delete_dataset(&self, dataset: Dataset) -> Result<(), LigatureError> {
        todo!()
    }

    fn query(&self, dataset: Dataset) -> Result<Box<dyn QueryTx>, LigatureError> {
        todo!()
    }

    fn write(&self, dataset: Dataset) -> Result<Box<dyn WriteTx>, LigatureError> {
        todo!()
    }
}

struct LigatureSledQueryTx {
    //TODO
}

impl QueryTx for LigatureSledQueryTx {
    fn all_statements(&self) -> Box<dyn Iterator<Item = Statement>> {
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
    //TODO
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
