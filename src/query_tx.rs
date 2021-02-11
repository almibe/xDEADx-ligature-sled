// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::codec::{
    decode_dataset, decode_statement_permutation, encode_dataset, encode_dataset_match, EAVC_PREFIX,
};
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

    fn load_statement(
        &self,
        encoded_statement: Vec<u8>,
    ) -> Result<PersistedStatement, LigatureError> {
        let statement_id_set = decode_statement_permutation(encoded_statement);
        todo!()
    }
}

impl QueryTx for LigatureSledQueryTx {
    fn all_statements(
        &self,
    ) -> Box<dyn Iterator<Item = Result<PersistedStatement, LigatureError>> + '_> {
        let itr = self.store.scan_prefix(vec![EAVC_PREFIX]);
        Box::new(itr.map(move |eavc_res| match eavc_res {
            Ok(eavc) => Ok(self.load_statement(eavc.0.to_vec())?),
            Err(_) => Err(LigatureError("Error iterating Statements.".to_string())),
        }))
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
