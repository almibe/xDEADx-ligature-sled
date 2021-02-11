// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::codec::{
    decode_dataset, decode_id, encode_attribute, encode_dataset, encode_dataset_match, encode_id,
    encode_statement_permutations, encode_string_literal, prepend, StatementIDSet,
    ATTRIBUTE_ID_COUNTER_KEY, ATTRIBUTE_ID_TO_NAME_PREFIX, ATTRIBUTE_NAME_TO_ID_PREFIX,
    ENTITY_ID_COUNTER_KEY, STRING_LITERAL_ID_COUNTER_KEY, STRING_LITERAL_ID_TO_VALUE_PREFIX,
    STRING_LITERAL_VALUE_TO_ID_PREFIX, STRING_VALUE_PREFIX,
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
            None => Err(LigatureError(format!(
                "Could not find ID Counter for {}",
                id
            ))),
        }
    }

    /// Checks if the passed Entity is valid and if so returns the id of the entity.
    /// Otherwise a LigatureError is returned.
    fn check_entity(&self, entity: &Entity) -> Result<u64, LigatureError> {
        let current_id = self.read_id(ENTITY_ID_COUNTER_KEY)?;
        if entity.0 <= current_id {
            Ok(entity.0)
        } else {
            Err(LigatureError(format!("Invalid Entity {:?}", entity)))
        }
    }

    /// Checks if an Attribute exists and returns it's id if it does.
    /// Otherwise it creates a new Attribute and returns the new id.
    fn check_or_create_attribute(&self, attribute: &Attribute) -> Result<u64, LigatureError> {
        let encoded_attribute = prepend(ATTRIBUTE_NAME_TO_ID_PREFIX, encode_attribute(attribute));
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
        let next_attribute_id = self.read_id(ATTRIBUTE_ID_COUNTER_KEY)? + 1;
        self.store
            .insert(vec![ATTRIBUTE_ID_COUNTER_KEY], encode_id(next_attribute_id))
            .map_err(|_| LigatureError("Could not increment Attribute Counter".to_string()))?;
        self.store
            .insert(
                prepend(ATTRIBUTE_NAME_TO_ID_PREFIX, encode_attribute(attribute)),
                encode_id(next_attribute_id),
            )
            .map_err(|_| LigatureError(format!("Error saving attribute {:?}", attribute)))?;
        self.store
            .insert(
                prepend(ATTRIBUTE_ID_TO_NAME_PREFIX, encode_id(next_attribute_id)),
                encode_attribute(attribute),
            )
            .map_err(|_| LigatureError(format!("Error saving attribute {:?}", attribute)))?;
        Ok(next_attribute_id)
    }

    /// Checks if a value exists and if it does returns the Value's type prefix and the Value's id.
    /// Otherwise it create a new instance of the value and returns the same.
    fn check_or_create_value(&self, value: &Value) -> Result<(u8, u64), LigatureError> {
        match value {
            Value::Entity(entity) => {
                //TODO see handle entity above
                todo!()
            }
            Value::StringLiteral(value) => self.check_or_create_string_literal(value),
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

    /// Checks if a String Literal already exists or creates a new one if it doesn't.
    /// Returns a tuple of the STRING_LITERAL_PREFIX and the String Literal's ID.
    fn check_or_create_string_literal(
        &self,
        string_literal: &String,
    ) -> Result<(u8, u64), LigatureError> {
        let encoded_string = prepend(
            STRING_LITERAL_VALUE_TO_ID_PREFIX,
            encode_string_literal(string_literal),
        );
        let string_opt = self
            .store
            .get(encoded_string)
            .map_err(|_| LigatureError(format!("Could not fetch String {:?}", string_literal)))?;
        match string_opt {
            Some(s) => Ok((STRING_VALUE_PREFIX, decode_id(s.to_vec())?)),
            None => Ok((
                STRING_VALUE_PREFIX,
                self.create_string_literal(string_literal)?,
            )),
        }
    }

    /// Creates a new String Literal (does not check if it already exists!).
    /// Returns the String Literal's new ID.
    fn create_string_literal(&self, string_literal: &String) -> Result<u64, LigatureError> {
        let next_string_literal_id = self.read_id(STRING_LITERAL_ID_COUNTER_KEY)? + 1;
        self.store
            .insert(
                vec![STRING_LITERAL_ID_COUNTER_KEY],
                encode_id(next_string_literal_id),
            )
            .map_err(|_| {
                LigatureError("Could not increment String Literal ID Counter".to_string())
            })?;
        self.store
            .insert(
                prepend(
                    STRING_LITERAL_VALUE_TO_ID_PREFIX,
                    encode_string_literal(string_literal),
                ),
                encode_id(next_string_literal_id),
            )
            .map_err(|_| {
                LigatureError(format!("Error saving String Literal {:?}", string_literal))
            })?;
        self.store
            .insert(
                prepend(
                    STRING_LITERAL_ID_TO_VALUE_PREFIX,
                    encode_id(next_string_literal_id),
                ),
                encode_string_literal(string_literal),
            )
            .map_err(|_| {
                LigatureError(format!("Error saving String Literal {:?}", string_literal))
            })?;
        Ok(next_string_literal_id)
    }
}

impl WriteTx for LigatureSledWriteTx {
    fn new_entity(&self) -> Result<Entity, LigatureError> {
        let next_id_value = self.read_id(ENTITY_ID_COUNTER_KEY)? + 1;
        self.store
            .insert(vec![ENTITY_ID_COUNTER_KEY], encode_id(next_id_value))
            .map_err(|_| LigatureError("Could not increment Dataset Counter".to_string()))?;
        Ok(Entity(next_id_value))
    }

    fn add_statement(&self, statement: &Statement) -> Result<PersistedStatement, LigatureError> {
        let entity_id = self.check_entity(&statement.entity)?;
        let attribute_id = self.check_or_create_attribute(&statement.attribute)?;
        let (value_type_prefix, value_id) = self.check_or_create_value(&statement.value)?;
        let context = self.new_entity()?;

        let statement_id_set = StatementIDSet {
            entity_id: entity_id,
            attribute_id: attribute_id,
            value_prefix: value_type_prefix,
            value_id: value_id,
            context_id: context.0,
        };

        let permutations = encode_statement_permutations(&statement_id_set);

        for p in permutations {
            self.store.insert(p, vec![]);
        }

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
}
