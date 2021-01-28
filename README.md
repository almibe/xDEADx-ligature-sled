# ligature-sled
An implementation of Ligature that uses Sled for storage.

## How Storage is Handled
The default tree stores only the names of all the datasets stored.
Each dataset then has its own tree where the records for a given dataset is stored.

Inside of a dataset's tree statements are stored in a way that is inspired by the hexastore architecture.
Each part of statement is stored and given a unique id.
Those ids are then used to store the following permutations.
* SPOG
* SOPG
* PSOG
* POSG
* POSG
* OPSG
* GSPO
