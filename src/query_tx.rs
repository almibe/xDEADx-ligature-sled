// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::codec::{
    decode_attribute, decode_dataset, decode_statement_permutation, encode_dataset,
    encode_dataset_match, prepend, ATTRIBUTE_ID_TO_NAME_PREFIX, EAVC_PREFIX,
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
        let statement_id_set = decode_statement_permutation(encoded_statement)?;
        let entity = Entity(statement_id_set.entity_id);
        let attribute = self.load_attribute(statement_id_set.attribute_id)?;
        let value = self.load_value(statement_id_set.value_prefix, statement_id_set.value_id)?;
        let context = Entity(statement_id_set.context_id);
        Ok(PersistedStatement {
            statement: Statement {
                entity: entity,
                attribute: attribute,
                value: value,
            },
            context: context,
        })
    }

    fn load_attribute(&self, attribute_id: u64) -> Result<Attribute, LigatureError> {
        let encoded_attribute_opt = self
            .store
            .get(prepend(
                ATTRIBUTE_ID_TO_NAME_PREFIX,
                attribute_id.to_be_bytes().to_vec(),
            ))
            .map_err(|_| {
                LigatureError(format!(
                    "Error looking up attribute with id = {}",
                    attribute_id
                ))
            })?;
        match encoded_attribute_opt {
            Some(encoded_attribute) => decode_attribute(encoded_attribute.to_vec()),
            None => Err(LigatureError(format!(
                "Could not find attribute with id = {}",
                attribute_id
            ))),
        }
    }

    fn load_value(&self, value_type: u8, value_id: u64) -> Result<Value, LigatureError> {
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
