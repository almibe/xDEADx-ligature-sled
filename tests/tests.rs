// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use ligature::{Dataset, Ligature, Link, PersistedLink, LigatureError};
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
    fn creating_a_new_dataset() {
        let mut instance = instance();
        instance.create_dataset(dataset("test/test"));
        let res: Vec<Result<Dataset, LigatureError>> = instance.all_datasets().collect();
        let expected: Vec<Result<Dataset, LigatureError>> = vec![Ok(dataset("test/test"))];
        assert_eq!(res, expected);
    }

    #[test]
    fn access_and_delete_new_dataset() {
        let mut instance = instance();
        instance.create_dataset(dataset("test/test"));
        instance.delete_dataset(dataset("test/test"));
        instance.delete_dataset(dataset("test/test2"));
        let res: Vec<Result<Dataset, LigatureError>> = instance.all_datasets().collect();
        assert!(res.is_empty());
    }

//    #[test]
    fn new_datasets_should_be_empty() {
        let instance = instance();
        instance.create_dataset(dataset("test/test"));
        let read_tx = instance.query(dataset("test/test")).unwrap();
        let res: Vec<Result<PersistedLink, LigatureError>> = read_tx.all_links().collect();
        assert!(res.is_empty());
    }

    // #[test]
    // fn new_blank_node() {
    //     let instance = LigatureMock::new();
    //     let write_tx = instance.write();
    //     let nn1 = write_tx.newNode(testDataset);
    //     let nn2 = write_tx.newNode(testDataset);
    //     write_tx.addStatement(testDataset, Statement(nn1, a, nn2));
    //     let nn3 = write_tx.newNode(testDataset);
    //     let nn4 = write_tx.newNode(testDataset);
    //     write_tx.addStatement(testDataset, Statement(nn3, a, nn4));
    //     write_tx.commit();
    //     let read_tx = instance.query();
    //     let res = read_tx.allStatements(testDataset).toListL;
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
    //     let ent1 = tx.newNode(testDataset);
    //     let ent2 = tx.newNode(testDataset);
    //     tx.addStatement(testDataset, Statement(ent1, a, ent2));
    //     tx.addStatement(testDataset, Statement(ent1, a, ent2));
    //     write_tx.commit();
    //     let read_tx = instance.query();
    //     let res = read_tx
    //         .allStatements(testDataset)
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
    //     let nn1 = tx.newNode(testDataset);
    //     let nn2 = tx.newNode(testDataset);
    //     let nn3 = tx.newNode(testDataset);
    //     tx.addStatement(testDataset, Statement(nn1, a, nn2));
    //     tx.addStatement(testDataset, Statement(nn3, a, nn2));
    //     tx.removeStatement(testDataset, Statement(nn1, a, nn2));
    //     tx.removeStatement(testDataset, Statement(nn1, a, nn2));
    //     tx.removeStatement(testDataset, Statement(nn2, a, nn1));
    //     let read_tx = instance.query();
    //     let res = tx
    //         .allStatements(testDataset)
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
    //         r1 = tx.matchStatements(testDataset, null, null, StringLiteral("French")).toListL
    //         r2 = tx.matchStatements(testDataset, null, a, null).toListL
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
    //       valjean = tx.newNode(testDataset)
    //       javert = tx.newNode(testDataset)
    //       tx.addStatement(testDataset, Statement(valjean, Predicate("nationality"), StringLiteral("French")))
    //       tx.addStatement(testDataset, Statement(valjean, Predicate("prisonNumber"), LongLiteral(24601)))
    //       tx.addStatement(testDataset, Statement(javert, Predicate("nationality"), StringLiteral("French")))
    //     }
    //     instance.read.use { tx =>
    //       tx.matchStatements(testDataset, null, null, StringLiteral("French"))
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("nationality"), StringLiteral("French")),
    //                   Statement(javert, Predicate("nationality"), StringLiteral("French"))
    //       )
    //       tx.matchStatements(testDataset, null, null, LongLiteral(24601))
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("prisonNumber"), LongLiteral(24601))
    //       )
    //       tx.matchStatements(testDataset, valjean)
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("nationality"), StringLiteral("French")),
    //                   Statement(valjean, Predicate("prisonNumber"), LongLiteral(24601))
    //       )
    //       tx.matchStatements(testDataset, javert, Predicate("nationality"), StringLiteral("French"))
    //               .toSet() shouldBe setOf(
    //                   Statement(javert, Predicate("nationality"), StringLiteral("French"))
    //       )
    //       tx.matchStatements(testDataset, null, null, null)
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("nationality"), StringLiteral("French")),
    //                   Statement(valjean, Predicate("prisonNumber"), LongLiteral(24601)),
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
    //       valjean = tx.newNode(testDataset)
    //       javert = tx.newNode(testDataset)
    //       trout = tx.newNode(testDataset)
    //       tx.addStatement(testDataset, Statement(valjean, Predicate("nationality"), StringLiteral("French")))
    //       tx.addStatement(testDataset, Statement(valjean, Predicate("prisonNumber"), LongLiteral(24601)))
    //       tx.addStatement(testDataset, Statement(javert, Predicate("nationality"), StringLiteral("French")))
    //       tx.addStatement(testDataset, Statement(javert, Predicate("prisonNumber"), LongLiteral(24602)))
    //       tx.addStatement(testDataset, Statement(trout, Predicate("nationality"), StringLiteral("American")))
    //       tx.addStatement(testDataset, Statement(trout, Predicate("prisonNumber"), LongLiteral(24603)))
    //     }
    //     instance.read.use { tx =>
    //       tx.matchStatements(testDataset, null, null, StringLiteralRange("French", "German"))
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("nationality"), StringLiteral("French")),
    //                   Statement(javert, Predicate("nationality"), StringLiteral("French"))
    //       )
    //       tx.matchStatements(testDataset, null, null, LongLiteralRange(24601, 24603))
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("prisonNumber"), LongLiteral(24601)),
    //                   Statement(javert, Predicate("prisonNumber"), LongLiteral(24602))
    //       )
    //       tx.matchStatements(testDataset, valjean, null, LongLiteralRange(24601, 24603))
    //               .toSet() shouldBe setOf(
    //                   Statement(valjean, Predicate("prisonNumber"), LongLiteral(24601))
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
