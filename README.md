# ligature-sled
An implementation of Ligature that uses Sled for storage.

## How Storage is Handled
The default tree stores only the names of all the datasets stored.
Each dataset then has its own tree where the records for a given dataset is stored.

Inside of a dataset's tree statements are stored in a way that is inspired by the hexastore architecture.
Each part of statement is stored and given a unique id.
Those ids are then used to store the following permutations.
`S = Source; A = Arrow; T = Target; C = Context;`
* `SATC`
* `STAC`
* `ASTC`
* `ATSC`
* `ATSC`
* `TASC`
* `CSPT`

## Building
This project uses cargo for building.
See https://rustup.rs/ for instructions on installing the Rust toolchain.
See https://doc.rust-lang.org/cargo/ for documentation on cargo in general.
