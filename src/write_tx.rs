// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//#![deny(missing_docs)]

use super::codec::{decode_dataset, encode_dataset, encode_dataset_match};
use ligature::{
    Attribute, Dataset, Ligature, LigatureError, Statement, PersistedStatement, QueryTx,
    Range, WriteTx, Value, Entity,
};

pub struct LigatureSledWriteTx {
    store: sled::Tree,
}

impl LigatureSledWriteTx {
    pub fn new(store: sled::Tree) -> Self {
        Self {
            store: store,
        }
    }
}

impl WriteTx for LigatureSledWriteTx {
    fn new_entity(&self) -> Result<Entity, LigatureError> {
//        store.get()
        todo!()
    }

    fn add_statement(&self, statement: &Statement) -> Result<PersistedStatement, LigatureError> {
        //handle entity
        //handle attribute
        //handle value
        match &statement.value {
            Value::Entity(entity) => { todo!() },
            Value::StringLiteral(value) => { todo!() },
            Value::LongLiteral(value) => { todo!() },
            Value::FloatLiteral(value) => { todo!() },
        }
        //handle context
        todo!()
    }

    fn remove_statement(&self, persisted_statement: &PersistedStatement) -> Result<(), LigatureError> {
        todo!()
    }

    fn cancel(&self) -> Result<(), LigatureError> {
        todo!()
    }

    fn commit(&self) -> Result<(), LigatureError> {
        todo!()
    }
}
