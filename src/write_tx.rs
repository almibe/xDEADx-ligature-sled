// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::codec::{
    decode_dataset, decode_id, encode_attribute, encode_dataset, encode_dataset_match, encode_id,
    ENTITY_ID_PREFIX,
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

    fn read_id(&self, id: u8) -> Result<u64, LigatureError> {
        let id_opt = self
            .store
            .get(vec![id])
            .map_err(|_| LigatureError(format!("Could not id {}", id)))?;
        match id_opt {
            Some(id) => {
                let id_value = decode_id(id.to_vec())?;
                Ok(id_value)
            }
            None => Err(LigatureError("Could not find Dataset Counter".to_string())),
        }
    }

    /// Checks if the passed Entity is valid and if so returns the id of the entity.
    /// Otherwise a LigatureError is returned.
    fn check_entity(&self, entity: &Entity) -> Result<u64, LigatureError> {
        let current_id = self.read_id(ENTITY_ID_PREFIX)?;
        if (entity.0 <= current_id) {
            Ok(entity.0)
        } else {
            Err(LigatureError(format!("Invalid Entity {:?}", entity)))
        }
    }

    /// Checks if an Attribute exists and returns it's id if it does.
    /// Otherwise it creates a new Attribute and returns the new id.
    fn check_or_create_attribute(&self, attribute: &Attribute) -> Result<u64, LigatureError> {
        let encoded_attribute = encode_attribute(attribute);
        let attribute_opt = self
            .store
            .get(encoded_attribute)
            .map_err(|_| LigatureError(format!("Could not fetch Attribute {:?}", attribute)))?;
        match attribute_opt {
            Some(a) => decode_id(a.to_vec()),
            None => self.create_attribute(attribute),
        }
    }

    /// Creates an Attribute that doesn't exist (doesn't check whether it does or not!) and returns the Attribute's id.
    fn create_attribute(&self, attribute: &Attribute) -> Result<u64, LigatureError> {
        //TODO read attribute id
        //TODO increment and store id
        //TODO store attribute name to id
        //TODO store attribute id to name
        todo!()
    }

    /// Checks if a value exists and if it does returns the Value's type prefix and the Value's id.
    /// Otherwise it create a new instance of the value and returns the same.
    fn check_or_create_value(&self, value: &Value) -> Result<(u8, u64), LigatureError> {
        match value {
            Value::Entity(entity) => {
                //TODO see handle entity above
                todo!()
            }
            Value::StringLiteral(value) => {
                //TODO handle String literals similarly to Entities and Attributes
                todo!()
            }
            Value::IntegerLiteral(value) => {
                //TODO encode long and use
                todo!()
            }
            Value::FloatLiteral(value) => {
                //TODO encode float and use
                todo!()
            }
        }
    }
}

impl WriteTx for LigatureSledWriteTx {
    fn new_entity(&self) -> Result<Entity, LigatureError> {
        let next_id_value = self.read_id(ENTITY_ID_PREFIX)? + 1;
        self.store
            .insert(vec![ENTITY_ID_PREFIX], encode_id(next_id_value))
            .map_err(|_| LigatureError("Could not increment Dataset Counter".to_string()))?;
        Ok(Entity(next_id_value))
    }

    fn add_statement(&self, statement: &Statement) -> Result<PersistedStatement, LigatureError> {
        let entity = self.check_entity(&statement.entity)?;
        let attribute_id = self.check_or_create_attribute(&statement.attribute);
        let value_id = self.check_or_create_value(&statement.value);
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
