# ligature-sled
An implementation of Ligature that uses Sled for storage.

## How Storage is Handled
The default tree stores only the names of all the datasets stored.
Each dataset then has its own tree where the records for a given dataset are stored.

ligature-sled uses a set of `u8` prefixes before any entry into the database.
Below is a list of what is currently being used.

| Prefix | Description          | Stored In    | Key       | Value  |
| ------ | -------------------- | ------------ | --------- | ------ |
| 0      | Dataset Name         | Root tree    | u8 string | _      |
| 1      | Dataset Counter      | Dataset tree | u8        | u64    |
| 2      | Attribute Name to ID | Dataset tree | u8 string | u64    |
| 3      | Attribute ID to Name | Dataset tree | u8 u64    | string |
| 4      | EAVC                 | Dataset tree | see note  | _      |
| 5      | EVAC                 | Dataset tree | see note  | _      |
| 6      | AEVC                 | Dataset tree | see note  | _      |
| 7      | AVEC                 | Dataset tree | see note  | _      |
| 8      | VEAC                 | Dataset tree | see note  | _      |
| 9      | VAEC                 | Dataset tree | see note  | _      |
| 10     | CEAV                 | Dataset tree | see note  | _      |
| 11     | String Literal to ID | Dataset tree | u8 string | u64    |
| 12     | ID to String Literal | Dataset tree | u8 u64    | string |

Vertices also have prefixes to say what kind of vertex they are.

| Prefix | Type   |
| ------ | ------ |
| 0      | Entity |
| 1      | String |
| 2      | Long   |
| 3      | Float  |

NOTE:

Inside of a Dataset's tree, Statements are stored in a way that is inspired by the hexastore (I guess in Ligature's case it's a heptastore?) architecture.
This involves storing many permutations of the Statement.
Entities are referneced by their IDs.
Attributes are referenced by their IDs.
Values are referenced by the above prefix and their value (if a long or float) or ID (if a String or Entity).
Contexts are referenced by their IDs (they are just Entities).

`E = Entity; A = Attribute; V = Value; C = Context;`
* `EAVC`
* `EVAC`
* `AEVC`
* `AVEC`
* `VEAC`
* `VAEC`
* `CEAV`

## Building
This project uses cargo for building.
See https://rustup.rs/ for instructions on installing the Rust toolchain.
See https://doc.rust-lang.org/cargo/ for documentation on cargo in general.
