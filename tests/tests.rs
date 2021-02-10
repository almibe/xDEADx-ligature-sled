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

    // #[test]
    // fn new_datasets_should_be_empty() -> Result<(), LigatureError> {
    //     let instance = instance();
    //     let test_dataset = dataset("test/test");
    //     instance.create_dataset(&test_dataset)?;
    //     let read_tx = instance.query(&test_dataset)?;
    //     let res: Vec<Result<PersistedStatement, LigatureError>> =
    //         read_tx.all_statements().collect();
    //     assert!(res.is_empty());
    //     Ok(())
    // }

    // #[test]
    // fn create_new_entity() -> Result<(), LigatureError> {
    //     let instance = instance();
    //     let test_dataset = dataset("test/test");
    //     instance.create_dataset(&test_dataset)?;
    //     let write_tx = instance.write(&test_dataset)?;
    //     let entity1 = write_tx.new_entity()?;
    //     let entity2 = write_tx.new_entity()?;
    //     assert_eq!(entity1.0, 1);
    //     assert_eq!(entity2.0, 2);
    //     assert!(entity1 != entity2);
    //     Ok(())
    // }

    // #[test]
    // fn add_a_basic_statement() -> Result<(), LigatureError> {
    //     let instance = instance();
    //     let test_dataset = dataset("test/test");
    //     instance.create_dataset(&test_dataset)?;
    //     let write_tx = instance.write(&test_dataset)?;
    //     let entity = write_tx.new_entity()?;
    //     let attribute = Attribute::new("name")?;
    //     let value = Value::StringLiteral("Juniper".to_string());
    //     let statement = Statement {
    //         entity: entity,
    //         attribute: attribute,
    //         value: value,
    //     };
    //     write_tx.add_statement(&statement)?;
    //     write_tx.commit()?;
    //     let read_tx = instance.query(&test_dataset)?;
    //     let res: Vec<Result<PersistedStatement, LigatureError>> =
    //         read_tx.all_statements().collect();
    //     assert_eq!(res.len(), 1); //TODO check instance not just number
    //                               //TODO check context on persisted statements
    //     Ok(())
    // }

    // #[test]
    // fn new_node() {
    //     let instance = instance();
    //     let test_dataset = dataset("test/test");
    //     instance.create_dataset(&test_dataset);
    //     let write_tx = instance.write(&test_dataset)?;
    //     let n1 = write_tx.new_node();
    //     let arrow = Attribute("arrow");
    //     let n2 = write_tx.new_node();
    //     let statement = Statement {
    //         source: n1,
    //         arrow: arrow,
    //         target: n2,
    //     };
    //     let persisted_statement = write_tx.add_statement(&statement);
    //     let nn3 = write_tx.new_node();
    //     let nn4 = write_tx.new_node();
    //     let statement2 = Statement {
    //         source: n3,
    //         arrow: arrow,
    //         target: n4,
    //     }
    //     let persisted_statement2 = write_tx.add_statement(statement2);
    //     write_tx.commit();
    //     let read_tx = instance.query();
    //     let res = read_tx.allStatements(&test_dataset).toListL;
    //     assert_equals!(
    //         res.map, /*{ _.statement }*/
    //         Set(
    //             Statement(BlankNode(1), a, BlankNode(2)),
    //             Statement(BlankNode(4), a, BlankNode(5)),
    //         ),
    //     );
    // }

    // #[test]
    // fn adding_statements_to_datasets() {
    //     let instance = LigatureMock::new();
    //     let write_tx = instance.write();
    //     let ent1 = tx.newNode(test_dataset);
    //     let ent2 = tx.newNode(test_dataset);
    //     tx.addStatement(test_dataset, Statement(ent1, a, ent2));
    //     tx.addStatement(test_dataset, Statement(ent1, a, ent2));
    //     write_tx.commit();
    //     let read_tx = instance.query();
    //     let res = read_tx
    //         .allStatements(test_dataset)
    //         .map /*{ _.statement }*/
    //         .toListL;
    //     assert_equals!(
    //         res,
    //         Set(
    //             Statement(BlankNode(1), a, BlankNode(2)),
    //             Statement(BlankNode(1), a, BlankNode(2)),
    //         ),
    //     )
    // }

    // #[test]
    // fn removing_statements_from_datasets() {
    //     let instance = LigatureMock::new();
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
    //   fn matching_against_a_non_existent_dataset() {
    //        let instance = LigatureMock::new();

    //     let (r1, r2) = instance.read.use { tx =>
    //       for {
    //         r1 = tx.matchStatements(test_dataset, null, null, StringLiteral("French")).toListL
    //         r2 = tx.matchStatements(test_dataset, null, a, null).toListL
    //       } yield(r1, r2)
    //     }
    //   }

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

    //   #[test]
    //   fn matching_statements_with_dataset_literals_in_datasets() {
    //        let instance = LigatureMock::new();

    //     let res = createLigature.instance.use { instance  =>
    //     let dataset = store.createDataset(NamedNode("test"))
    //     dataset shouldNotBe null
    //     let tx = dataset.writeTx()
    //     TODO("Add values")
    //     tx.commit()
    //     let tx = dataset.tx()
    //     TODO("Add assertions")
    //     tx.cancel() // TODO add test running against a non-existant dataset w/ match-statement calls
    //   }
}
