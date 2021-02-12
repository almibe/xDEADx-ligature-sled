// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use ligature::{
        Attribute, Dataset, Ligature, LigatureError, PersistedStatement, Statement, Value,
    };
    use ligature_sled::LigatureSled;

    fn dataset(name: &str) -> Dataset {
        Dataset::new(name).expect("")
    }

    fn instance() -> LigatureSled {
        LigatureSled::temp(None).unwrap()
    }

    #[test]
    fn create_and_close_store() {
        let instance = instance();
        let res: Vec<Result<Dataset, LigatureError>> = instance.all_datasets().collect();
        assert!(res.is_empty());
    }

    #[test]
    fn creating_a_new_dataset() -> Result<(), LigatureError> {
        let test_dataset = dataset("test/test");
        let instance = instance();
        instance.create_dataset(&test_dataset)?;
        let res: Vec<Result<Dataset, LigatureError>> = instance.all_datasets().collect();
        let expected: Vec<Result<Dataset, LigatureError>> = vec![Ok(test_dataset)];
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    fn check_if_datasets_exist() -> Result<(), LigatureError> {
        let test_dataset = dataset("test/test");
        let test_dataset2 = dataset("test/test2");
        let instance = instance();
        instance.create_dataset(&test_dataset)?;
        let res1 = instance.dataset_exists(&test_dataset)?;
        let res2 = instance.dataset_exists(&test_dataset2)?;
        assert!(res1);
        assert!(!res2);
        Ok(())
    }

    #[test]
    fn match_datasets_prefix() -> Result<(), LigatureError> {
        let test_dataset = dataset("test/test");
        let test_dataset2 = dataset("test/test2");
        let test_dataset3 = dataset("test3/test");
        let instance = instance();
        instance.create_dataset(&test_dataset)?;
        instance.create_dataset(&test_dataset2)?;
        instance.create_dataset(&test_dataset3)?;
        let res1: Vec<Result<Dataset, LigatureError>> =
            instance.match_datasets_prefix("test").collect();
        let res2: Vec<Result<Dataset, LigatureError>> =
            instance.match_datasets_prefix("test/").collect();
        let res3: Vec<Result<Dataset, LigatureError>> =
            instance.match_datasets_prefix("snoo").collect();
        assert_eq!(res1.len(), 3);
        assert_eq!(res2.len(), 2);
        assert_eq!(res3.len(), 0);
        Ok(())
    }

    #[test]
    fn match_datasets_range() -> Result<(), LigatureError> {
        let datasets = vec![
            dataset("a"),
            dataset("app"),
            dataset("b"),
            dataset("be"),
            dataset("bee"),
            dataset("test1/test"),
            dataset("test2/test2"),
            dataset("test3/test"),
            dataset("test4"),
            dataset("z"),
        ];
        let instance = instance();
        for dataset in datasets {
            instance.create_dataset(&dataset)?;
        }
        let res1: Vec<Result<Dataset, LigatureError>> =
            instance.match_datasets_range("a", "b").collect();
        let res2: Vec<Result<Dataset, LigatureError>> =
            instance.match_datasets_range("be", "test3").collect();
        let res3: Vec<Result<Dataset, LigatureError>> =
            instance.match_datasets_range("snoo", "zz").collect();
        assert_eq!(res1.len(), 2); //TODO check instances not just counts
        assert_eq!(res2.len(), 4); //TODO check instances not just counts
        assert_eq!(res3.len(), 5); //TODO check instances not just counts
        Ok(())
    }

    #[test]
    fn create_and_delete_new_dataset() -> Result<(), LigatureError> {
        let instance = instance();
        let test_dataset = dataset("test/test");
        let test_dataset2 = dataset("test/test2");
        instance.create_dataset(&test_dataset)?;
        instance.delete_dataset(&test_dataset)?;
        instance.delete_dataset(&test_dataset2)?;
        let res: Vec<Result<Dataset, LigatureError>> = instance.all_datasets().collect();
        assert!(res.is_empty());
        Ok(())
    }

    #[test]
    fn new_datasets_should_be_empty() -> Result<(), LigatureError> {
        let instance = instance();
        let test_dataset = dataset("test/test");
        instance.create_dataset(&test_dataset)?;
        let res: Vec<PersistedStatement> =
            instance.query(&test_dataset, Box::new(|tx| tx.all_statements().collect()))?;
        assert!(res.is_empty());
        Ok(())
    }

    #[test]
    fn create_new_entity() -> Result<(), LigatureError> {
        let instance = instance();
        let test_dataset = dataset("test/test");
        instance.create_dataset(&test_dataset)?;
        let (entity1, entity2) = instance.write(
            &test_dataset,
            Box::new(|tx| {
                let entity1 = tx.new_entity()?;
                let entity2 = tx.new_entity()?;
                Ok((entity1, entity2))
            }),
        )?;
        assert_eq!(entity1.0, 1);
        assert_eq!(entity2.0, 2);
        assert!(entity1 != entity2);
        Ok(())
    }

    #[test]
    fn add_a_basic_statement() -> Result<(), LigatureError> {
        let instance = instance();
        let test_dataset = dataset("test/test");
        instance.create_dataset(&test_dataset)?;
        instance.write(
            &test_dataset,
            Box::new(|tx| {
                let entity = tx.new_entity()?;
                let attribute = Attribute::new("name")?;
                let value = Value::StringLiteral("Juniper".to_string());
                let string_statement = Statement {
                    entity: entity.clone(),
                    attribute: attribute.clone(),
                    value: value.clone(),
                };

                let entity2 = tx.new_entity()?;
                let attribute2 = Attribute::new("connection")?;
                let entity3 = tx.new_entity()?;
                let entity_statement = Statement {
                    entity: entity2.clone(),
                    attribute: attribute2.clone(),
                    value: Value::Entity(entity3.clone()),
                };

                let integer = Value::IntegerLiteral(4200);
                let integer_statement = Statement {
                    entity: entity2.clone(),
                    attribute: attribute2.clone(),
                    value: integer.clone(),
                };

                let float = Value::FloatLiteral(42.2);
                let float_statement = Statement {
                    entity: entity3.clone(),
                    attribute: attribute2.clone(),
                    value: float.clone(),
                };

                tx.add_statement(&string_statement)?;
                tx.add_statement(&entity_statement)?;
                tx.add_statement(&integer_statement)?;
                tx.add_statement(&float_statement)?;

                Ok(())
            }),
        )?;
        let res: Vec<PersistedStatement> =
            instance.query(&test_dataset, Box::new(|tx| tx.all_statements().collect()))?;
        assert_eq!(res.len(), 4); //TODO check instance not just number
                                  //TODO check context on persisted statements
        Ok(())
    }

    // #[test]
    // fn removing_statements_from_datasets() -> Result<(), LigatureError> {
    //     let instance = instance();
    //     let write_tx = instance.write();
    //     let nn1 = tx.newNode(test_dataset);
    //     let nn2 = tx.newNode(test_dataset);
    //     let nn3 = tx.newNode(test_dataset);
    //     tx.addStatement(test_dataset, Statement(nn1, a, nn2));
    //     tx.addStatement(test_dataset, Statement(nn3, a, nn2));
    //     tx.removeStatement(test_dataset, Statement(nn1, a, nn2));
    //     tx.removeStatement(test_dataset, Statement(nn1, a, nn2));
    //     tx.removeStatement(test_dataset, Statement(nn2, a, nn1));
    //     let read_tx = instance.query();
    //     let res = tx
    //         .allStatements(test_dataset)
    //         .map /*{
    //           _.statement
    //         }*/
    //         .toListL;
    //     assert_equals!(res, Set(Statement(BlankNode(3), a, BlankNode(2))));
    // }

    //   #[test]
    //   fn matching_statements_in_datasets() {
    //        let instance = LigatureMock::new();
    //     let res = createLigature.instance.use { instance  =>
    //     lateinit var valjean: Node
    //     lateinit var javert: Node
    //     instance.write.use { tx =>
    //       valjean = tx.newNode(test_dataset)
    //       javert = tx.newNode(test_dataset)
    //       tx.addStatement(test_dataset, Statement(valjean, Predicate("nationality"), StringLiteral("French")))
    //       tx.addStatement(test_dataset, Statement(valjean, Predicate("prisonNumber"), IntegerLiteral(24601)))
    //       tx.addStatement(test_dataset, Statement(javert, Predicate("nationality"), StringLiteral("French")))
    //     }
    //     instance.read.use { tx =>
    //       tx.matchStatements(test_dataset, null, null, StringLiteral("French"))
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("nationality"), StringLiteral("French")),
    //                   Statement(javert, Predicate("nationality"), StringLiteral("French"))
    //       )
    //       tx.matchStatements(test_dataset, null, null, IntegerLiteral(24601))
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("prisonNumber"), IntegerLiteral(24601))
    //       )
    //       tx.matchStatements(test_dataset, valjean)
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("nationality"), StringLiteral("French")),
    //                   Statement(valjean, Predicate("prisonNumber"), IntegerLiteral(24601))
    //       )
    //       tx.matchStatements(test_dataset, javert, Predicate("nationality"), StringLiteral("French"))
    //               .toSet() shouldBe setOf(
    //                   Statement(javert, Predicate("nationality"), StringLiteral("French"))
    //       )
    //       tx.matchStatements(test_dataset, null, null, null)
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("nationality"), StringLiteral("French")),
    //                   Statement(valjean, Predicate("prisonNumber"), IntegerLiteral(24601)),
    //                   Statement(javert, Predicate("nationality"), StringLiteral("French"))
    //       )
    //     }
    //   }

    //   #[test]
    //   fn matching_statements_with_literals_and_ranges_in_datasets() {
    //        let instance = LigatureMock::new();

    //     let res = createLigature.instance.use { instance  =>
    //     lateinit var valjean: Node
    //     lateinit var javert: Node
    //     lateinit var trout: Node
    //     instance.write.use { tx =>
    //       valjean = tx.newNode(test_dataset)
    //       javert = tx.newNode(test_dataset)
    //       trout = tx.newNode(test_dataset)
    //       tx.addStatement(test_dataset, Statement(valjean, Predicate("nationality"), StringLiteral("French")))
    //       tx.addStatement(test_dataset, Statement(valjean, Predicate("prisonNumber"), IntegerLiteral(24601)))
    //       tx.addStatement(test_dataset, Statement(javert, Predicate("nationality"), StringLiteral("French")))
    //       tx.addStatement(test_dataset, Statement(javert, Predicate("prisonNumber"), IntegerLiteral(24602)))
    //       tx.addStatement(test_dataset, Statement(trout, Predicate("nationality"), StringLiteral("American")))
    //       tx.addStatement(test_dataset, Statement(trout, Predicate("prisonNumber"), IntegerLiteral(24603)))
    //     }
    //     instance.read.use { tx =>
    //       tx.matchStatements(test_dataset, null, null, StringLiteralRange("French", "German"))
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("nationality"), StringLiteral("French")),
    //                   Statement(javert, Predicate("nationality"), StringLiteral("French"))
    //       )
    //       tx.matchStatements(test_dataset, null, null, IntegerLiteralRange(24601, 24603))
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("prisonNumber"), IntegerLiteral(24601)),
    //                   Statement(javert, Predicate("prisonNumber"), IntegerLiteral(24602))
    //       )
    //       tx.matchStatements(test_dataset, valjean, null, IntegerLiteralRange(24601, 24603))
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("prisonNumber"), IntegerLiteral(24601))
    //       )
    //     }
    //   }
}
