// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//#![deny(missing_docs)]

use super::codec::{
    decode_counter, decode_dataset, encode_counter, encode_dataset, encode_dataset_match,
    ENTITY_COUNTER_PREFIX,
};
use ligature::{
    Attribute, Dataset, Entity, Ligature, LigatureError, PersistedStatement, QueryTx, Range,
    Statement, Value, WriteTx,
};

pub struct LigatureSledWriteTx {
    store: sled::Tree,
}

impl LigatureSledWriteTx {
    pub fn new(store: sled::Tree) -> Self {
        Self { store: store }
    }

    fn read_entity_counter(&self) -> Result<u64, LigatureError> {
        let counter_opt = self
            .store
            .get(vec![ENTITY_COUNTER_PREFIX])
            .map_err(|_| LigatureError("Could not find Dataset Counter".to_string()))?;
        match counter_opt {
            Some(counter) => {
                let counter_value = decode_counter(counter.to_vec())?;
                Ok(counter_value)
            }
            None => Err(LigatureError("Could not find Dataset Counter".to_string())),
        }
    }

    fn check_entity(&self, entity: &Entity) -> Result<(), LigatureError> {
        let current_counter = self.read_entity_counter()?;
        if (entity.0 > current_counter) {
            Ok(())
        } else {
            Err(LigatureError(format!("Invalid Entity {:?}", entity)))
        }
    }

    fn check_or_create_attribute(&self, attribute: &Attribute) -> Result<(), LigatureError> {
        //TODO handle attribute
        let encoded_attribute = encode_attribute(attribute);
        let attribute_opt = self.store.get(encode_attribute)?;
        match attribute_opt {
            Some(a) => todo!(),
            None    => todo!(),
        }
        //TODO - check if attribute exists
        //TODO - if so use attribute id
        //TODO - if not create new attribute and use its id
        todo!()
    }
}

impl WriteTx for LigatureSledWriteTx {
    fn new_entity(&self) -> Result<Entity, LigatureError> {
        let next_counter_value = self.read_entity_counter()? + 1;
        self.store
            .insert(vec![ENTITY_COUNTER_PREFIX], encode_counter(next_counter_value))
            .map_err(|_| {
                LigatureError("Could not increment Dataset Counter".to_string())
            })?;
        Ok(Entity(next_counter_value))
    }

    fn add_statement(&self, statement: &Statement) -> Result<PersistedStatement, LigatureError> {
        let entity = self.check_entity(&statement.entity);
        let attribute = self.check_or_create_attribute(&statement.attribute);
        let value = match &statement.value {
            Value::Entity(entity) => {
                //TODO see handle entity above
                todo!()
            }
            Value::StringLiteral(value) => {
                //TODO handle String literals similarly to Entities and Attributes
                todo!()
            }
            Value::LongLiteral(value) => {
                //TODO encode long and use
                todo!()
            }
            Value::FloatLiteral(value) => {
                //TODO encode float and use
                todo!()
            }
        };
        let context = self.new_entity()?;

        //TODO store permutations
        //TODO - EAVC
        //TODO - EVAC
        //TODO - AEVC
        //TODO - AVEC
        //TODO - VEAC
        //TODO - VAEC
        //TODO - CEAV

        Ok(PersistedStatement {
            statement: statement.clone(),
            context: context,
        })
    }

    fn remove_statement(
        &self,
        persisted_statement: &PersistedStatement,
    ) -> Result<(), LigatureError> {
        todo!()
    }

    fn cancel(&self) -> Result<(), LigatureError> {
        todo!()
    }

    fn commit(&self) -> Result<(), LigatureError> {
        todo!()
    }
}
