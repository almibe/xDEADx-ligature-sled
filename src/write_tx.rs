// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::codec::{
    decode_dataset, decode_id, encode_attribute, encode_dataset, encode_dataset_match, encode_id,
    encode_statement_permutations, encode_string_literal, prepend, StatementIDSet, AEVC_PREFIX,
    ATTRIBUTE_ID_COUNTER_KEY, ATTRIBUTE_ID_TO_NAME_PREFIX, ATTRIBUTE_NAME_TO_ID_PREFIX,
    CEAV_PREFIX, ENTITY_ID_COUNTER_KEY, ENTITY_VALUE_PREFIX, FLOAT_VALUE_PREFIX,
    INTEGER_VALUE_PREFIX, STRING_LITERAL_ID_COUNTER_KEY, STRING_LITERAL_ID_TO_VALUE_PREFIX,
    STRING_LITERAL_VALUE_TO_ID_PREFIX, STRING_VALUE_PREFIX, VEAC_PREFIX,
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
    fn check_or_create_value(&self, value: &Value) -> Result<(u8, Vec<u8>), LigatureError> {
        match value {
            Value::Entity(entity) => {
                let res = self.check_entity(entity)?;
                Ok((ENTITY_VALUE_PREFIX, res.to_be_bytes().to_vec()))
            }
            Value::StringLiteral(value) => self.check_or_create_string_literal(value),
            Value::IntegerLiteral(value) => {
                Ok((INTEGER_VALUE_PREFIX, value.to_be_bytes().to_vec()))
            }
            Value::FloatLiteral(value) => Ok((FLOAT_VALUE_PREFIX, value.to_be_bytes().to_vec())),
        }
    }

    /// Checks if a String Literal already exists or creates a new one if it doesn't.
    /// Returns a tuple of the STRING_LITERAL_PREFIX and the String Literal's ID.
    fn check_or_create_string_literal(
        &self,
        string_literal: &String,
    ) -> Result<(u8, Vec<u8>), LigatureError> {
        let encoded_string = prepend(
            STRING_LITERAL_VALUE_TO_ID_PREFIX,
            encode_string_literal(string_literal),
        );
        let string_opt = self
            .store
            .get(encoded_string)
            .map_err(|_| LigatureError(format!("Could not fetch String {:?}", string_literal)))?;
        match string_opt {
            Some(s) => Ok((STRING_VALUE_PREFIX, s.to_vec())),
            None => Ok((
                STRING_VALUE_PREFIX,
                self.create_string_literal(string_literal)?
                    .to_be_bytes()
                    .to_vec(),
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

    fn lookup_statement_id_set(
        &self,
        statement: &Statement,
        context: &Entity,
    ) -> Result<StatementIDSet, LigatureError> {
        let entity_id = self.check_entity(&statement.entity)?;
        let attribute_id = self.check_or_create_attribute(&statement.attribute)?;
        let (value_type_prefix, value_body) = self.check_or_create_value(&statement.value)?;

        Ok(StatementIDSet {
            entity_id: entity_id,
            attribute_id: attribute_id,
            value_prefix: value_type_prefix,
            value_body: value_body,
            context_id: context.0,
        })
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
        let context = self.new_entity()?;
        let statement_id_set = self.lookup_statement_id_set(statement, &context)?;
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
    ) -> Result<bool, LigatureError> {
        let prefix = prepend(CEAV_PREFIX, encode_id(persisted_statement.context.0));
        let lookup: Vec<Result<(sled::IVec, sled::IVec), sled::Error>> =
            self.store.scan_prefix(prefix).collect();
        if lookup.len() > 1 {
            panic!(
                "Invalid state of Ligature, Contexts must be unique, {:?}!!!",
                persisted_statement.context.0
            ); //TODO not sure the best way to handle this
        }
        if lookup.len() == 0 {
            return Ok(false);
        }
        let potential_match_encoded: Vec<u8> = lookup
            .first()
            .ok_or(LigatureError(format!(
                "Error creating Statements permutations for {:?}",
                persisted_statement
            )))?
            .clone()
            .map_err(|_| {
                LigatureError(format!(
                    "Error creating Statements permutations for {:?}",
                    persisted_statement
                ))
            })?
            .0
            .to_vec();
        let statement_id_set = self.lookup_statement_id_set(
            &persisted_statement.statement,
            &persisted_statement.context,
        )?;
        let encoded_statement_permutations: Vec<Vec<u8>> =
            encode_statement_permutations(&statement_id_set); //this potentially does an uneeded lookup
        if potential_match_encoded
            != encoded_statement_permutations
                .last()
                .ok_or(LigatureError(format!(
                    "Error creating Statements permutations for {:?}",
                    persisted_statement
                )))?
                .clone()
        {
            return Ok(false);
        }
        for encoded_statement in encoded_statement_permutations.iter() {
            self.store.remove(encoded_statement).map_err(|_| {
                LigatureError(format!(
                    "Could not remove Statement permutation {:?} for {:?}",
                    encoded_statement, persisted_statement
                ))
            })?;
        }

        //TODO clean up by checking if the attribute is used in any remaining statements by checking AEVC
        let attribute_prefix = prepend(
            AEVC_PREFIX,
            statement_id_set.attribute_id.to_be_bytes().to_vec(), //TODO should be encode_id
        );
        let attribute_lookup: Vec<Result<(sled::IVec, sled::IVec), sled::Error>> =
            self.store.scan_prefix(attribute_prefix).collect();
        if attribute_lookup.len() == 0 { //if it isn't used in other Statements then delete the Attribute
            let attribute_name_to_id_key = prepend(ATTRIBUTE_NAME_TO_ID_PREFIX, encode_attribute(&persisted_statement.statement.attribute));
            let attribute_id_to_name_key = prepend(ATTRIBUTE_ID_TO_NAME_PREFIX, encode_id(statement_id_set.attribute_id));

            self.store.remove(attribute_name_to_id_key); //TODO sanity check result
            self.store.remove(attribute_id_to_name_key); //TODO sanity check result
        }

        //TODO clean up by checking if the Value is a String Literal
        //TODO if it is check if it is being used in any other Statements by checking VAEC
        //TODO if it isn't then delete the String Literal
        if statement_id_set.value_prefix == STRING_VALUE_PREFIX {
            let value_prefix = prepend(VEAC_PREFIX, statement_id_set.value_body.clone());
            let value_lookup: Vec<Result<(sled::IVec, sled::IVec), sled::Error>> =
                self.store.scan_prefix(value_prefix).collect();
            if value_lookup.len() == 0 { //if it isn't used in other Statements then delete the Attribute
                let string_liteal_value = match &persisted_statement.statement.value {
                    Value::StringLiteral(value) => {
                        let string_literal_value_to_id_key = prepend(STRING_LITERAL_VALUE_TO_ID_PREFIX, encode_string_literal(&value));
                        let string_literal_id_to_value_key = prepend(STRING_LITERAL_ID_TO_VALUE_PREFIX, statement_id_set.value_body);

                        self.store.remove(string_literal_value_to_id_key); //TODO sanity check result
                        self.store.remove(string_literal_id_to_value_key); //TODO sanity check result        
                    },
                    _ => {
                        return Err(LigatureError("Excepted StringLiteral".to_string()));
                    }
                };
            }
        }
        Ok(true)
    }

    fn cancel(&self) -> Result<(), LigatureError> {
        todo!()
    }
}
