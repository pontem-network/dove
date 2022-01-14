# Move tools

Toolset for work with Move language based projects:

* [dove](#dove) - package manager and compiler.

Supported projects and dialects:

* [Diem](https://www.diem.com/en-us/)
* [Pontem](https://pontem.network/)
* [Dfinance](https://dfinance.co/)

## Installation

**Using pre-compiled binaries:**

* If you are using **Mac OS** see [how to install OpenSSL](https://github.com/pontem-network/move-tools/issues/147#issuecomment-946744809).

Just visit [releases page](https://github.com/pontem-network/move-tools/releases) and download binaries you are going to use.

**Using source code:**

Clone this repository and follow documentation:

```shell script
git clone git@github.com:pontem-network/move-tools.git
cd move-tools
```

## Dove

* If you are using **Mac OS** see [how to install OpenSSL](https://github.com/pontem-network/move-tools/issues/147#issuecomment-946744809).

Regular installation:

```shell script
cargo install --path dove
```

##### See help:

```shell script
dove -h
```

##### Create new project:

```shell script
dove new first_project 
```
By default, the Pont dialect is used. You can change the dialect by adding an entry in `Move.toml`.
```
[package]
dialect = "Pont"
```
* `dialect` - dialect of the Move language. Default is `pont`. Supported dialects:
* `diem` - for original diem version.
* `pont` - Polkadot SS58 addresses.
* `dfinance` - bech32 addresses.

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

Command `tx` allows you to create transactions for Polkadot chain with [Move Pallete](https://github.com/pontem-network/sp-move) on board.

`tx` takes script identifier, type parameters, and arguments and creates a transaction file as an artifact of work.

Example:
```shell script
dove tx 'store_u64(60)'
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
dove tx 'create_account<0x01::PONT::PONT>()'
```

You allow can use SS58 address format:

```shell script
dove tx 'create_account<5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY::MyToken::Token>()'
dove tx 'create_account(5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, 10, true, [10, 20, 30, 40])'
```

Supported types:

* Numbers (u8, u64, u128)
* Boolean
* Addresses
* Vectors
* Type parameters (generics).

For more commands and parameters look at help:

```shell script
dove tx --help
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

## LICENSE

[LICENSE](/LICENSE)
