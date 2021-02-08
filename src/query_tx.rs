// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//#![deny(missing_docs)]

use super::codec::{decode_dataset, encode_dataset, encode_dataset_match};
use ligature::{
    Attribute, Dataset, Entity, Ligature, LigatureError, PersistedStatement, QueryTx, Range,
    Statement, Value, WriteTx,
};

pub struct LigatureSledQueryTx {
    store: sled::Tree,
}

impl LigatureSledQueryTx {
    pub fn new(store: sled::Tree) -> Self {
        Self { store: store }
    }
}

impl QueryTx for LigatureSledQueryTx {
    fn all_statements(
        &self,
    ) -> Box<dyn Iterator<Item = Result<PersistedStatement, LigatureError>>> {
        Box::new(std::iter::empty())
    }

    fn match_statements(
        &self,
        entity: Option<Entity>,
        attribute: Option<Attribute>,
        value: Option<Value>,
    ) -> Box<dyn Iterator<Item = Result<PersistedStatement, LigatureError>>> {
        todo!()
    }

    fn match_statements_range(
        &self,
        entity: Option<Entity>,
        attribute: Option<Attribute>,
        value: Range,
    ) -> Box<dyn Iterator<Item = Result<PersistedStatement, LigatureError>>> {
        todo!()
    }

    fn statement_for_context(
        &self,
        context: &Entity,
    ) -> Result<Option<PersistedStatement>, LigatureError> {
        todo!()
    }
}
