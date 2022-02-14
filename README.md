# Dove

Move language package manager for Diem and Pontem networks.

See documentation at [https://docs.pontem.network/](https://docs.pontem.network/).

## Installation

* If you are using **Mac OS** see [how to install OpenSSL](https://github.com/pontem-network/dove/issues/147#issuecomment-946744809).

**Using pre-compiled binaries:**

Just visit [releases page](https://github.com/pontem-network/move-tools/releases) and download binaries you are going to use.

**Using source code:**

Clone this repository and follow documentation:

```shell script
git clone git@github.com:pontem-network/dove.git
cd dove
cargo install --path ./dove
```

##### See help:

```shell script
dove -h
```

##### Create new project:

```shell script
dove new first_project 
```

This command will create `first_project/` directory with special `Move.toml` manifest file and `sources/` directory for Move source code. 

##### Build project:

```shell script
dove build
```
See `./build/` folder to get scripts/modules binaries.

##### Clean build directory:
```shell script
dove clean
```
The contents of the directories will be deleted:
- `<PROJECT_DIR>/storage`
- `<PROJECT_DIR>/build`

##### Clear build directory and global cache:
```shell script
dove clean --global
```
The contents of the directories will be deleted:
- `<PROJECT_DIR>/storage`
- `<PROJECT_DIR>/build`
- `~/.move/`

### Pallet Transactions

Command `call` allows you to create transactions for Polkadot chain with [Move Pallete](https://github.com/pontem-network/sp-move) on board.

`call` takes script identifier, type parameters, and arguments and creates a transaction file as an artifact of work.

Example:
```shell script
dove call 'store_u64(60)'
```

This command searches for the script by name 'store_u64' in the script directory. Then it compiles it and creates a transaction file.

This command will fail if:

- There is no script with the name given name 'store_u64'.
- There is more than one script with the name 'store_64'.
- The passed parameters or type parameters do not match the script parameters.
- There are syntax errors in the script.

You can use type parameters like in the Move language.

Example:

```shell script
dove call 'create_account<0x01::PONT::PONT>()'
```

You allow can use SS58 address format:

```shell script
dove call 'create_account<0x1::MyToken::Token>()'
dove call 'create_account<ADDRESS_ALIAS::MyToken::Token>()'
dove call 'create_account(ADDRESS_ALIAS, 10, true, [10, 20, 30, 40], 0x1, SS58_ADDRESS)'
```

Supported types:

* Numbers (u8, u64, u128)
* Boolean
* Vectors
* Type parameters (generics).
* SS58 format address
* Addresses in hexadecimal format
* ADDRESS_ALIAS - Address alias. Specified in the "addresses" section of Move.toml

For more commands and parameters look at help:

```shell script
dove call --help
```

### More

If you look for examples, guidelines how to write modules/scripts or tests, visit [Pontem Documentation](https://docs.pontem.network/03.-move-vm/compiler_and_toolset).
 
## Resource Viewer

**Resource viewer is currently out of date and pending migration inside dove in future versions.**

See [documentation](/resource-viewer/README.md).

## Executor

Migrated inside Dove, see help:

```shell script
dove run --help
```

### Manage wallet keys

Command `key` allows you to save the secret keys to the wallet on your computer and access them under an alias.
Saved key can be used when publishing a module `$ dove deploy module --account <NAME_KEY> ...` or package `$ dove deploy package --account <NAME_KEY> ...`, as well as when execute a transaction `$ dove call --account <NAME_KEY> ...`.
Keys are stored on your computer in the `~/.move/` directory. Before saving, they are encrypted with the aes + password.

#### Adding a key:

```shell
dove key add --alias <NAME_KEY>
```
After executing this command, you will be prompted to enter a password and a secret phrase from your wallet.

If you don't want to protect the key with a password, use the `--nopassword` flag(**Not recommended**):

```shell
dove key add --alias <NAME_KEY> --nopassword
```

#### View list of saved keys

```shell
dove key list
```

#### Deleting a key

Deleting a key by name:

```shell
dove key delete --alias <NAME_KEY>
```

Deleting all saved keys:

```shell
dove key delete --all
```

## LICENSE

[LICENSE](/LICENSE)

