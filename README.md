# Move tools

Toolset for work with Move language based projects:

* [dove](#dove) - package manager and compiler.
* [language server](#language-server) - Move language server.
* [resource viewer](#resource-viewer) - [BCS](https://github.com/diem/bcs) resource viewer.

Supported projects and dialects:

* [Diem](https://www.diem.com/en-us/)
* [Pontem](https://pontem.network/)
* [Dfinance](https://dfinance.co/)

## Installation

**Using pre-compiled binaries:**

Just visit [releases page](https://github.com/pontem-network/pontem/releases/tag/v0.3.1) and download binaries you are going to use.

**Using source code:**

Clone this repository and follow documentation:

```shell script
git clone git@github.com:pontem-network/move-tools.git
cd move-tools
```

## Dove

Regular installation:

```shell script
cargo install --path dove
```

See help:

```shell script
dove -h
```

Create new project:

```shell script
dove new first_project --dialect pont --address <your address> # Replace <your address> with your SS58 address.
```

* `dialect` - dialect of the Move language. Default is `pont`. Supported dialects:
  * `diem` - for original diem version.
  * `pont` - Polkadot SS58 addresses.
  * `dfinance` - bech32 addresses.

Build project:

```shell script
dove build
```
See `./artifacts/` folder to get scripts/modules binaries.

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

## Language Server
 
**Language server is currently out of date and currently in process of migration in a separate project.**

Implementation of Language Server Protocol for [Move language](https://developers.diem.org/docs/crates/move-language).

```shell script
RUST_LOG=info cargo run --bin move-language-server
```

Features:
* check source code files with the official compiler on-the-fly

For the corresponding VSCode extension, see https://marketplace.visualstudio.com/items?itemName=PontemNetwork.move-language

#### Configuration

`dialect` - dialect of the Move language. Either `diem` (for original diem version) or `pont` (ss58), or `dfinance` (bech32 addresses and some other stuff). Default is `pont`.

`account_address` - address of the user, used for module imports. Default is `0x0`.

`stdlib_folder` - stdlib folder path. Default is `null`, no stdlib is loaded.

`modules_folders` - array of folder paths for module lookup. Default is empty array.

## Executor

Migrated inside Dove, see help:

```shell script
dove run --help
```

## Decompiler:

Install **decompiler**:

```shell script
cargo install --path=lang/decompiler --bin decompiler
```

See help:

```
decompiler --help
```

Try to decompile `.mv` file:

```
decompiler --input <path to compiled module or script>
```

## LICENSE

[LICENSE](/LICENSE)
