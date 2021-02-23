# Move tools

Toolset for work with Move language based projects:

* [dove](/dove/) - package manager and compiler.
* [language server](/language_server/) - Move language server.
* [resource viewer](/resource-viewer/) - [LCS](https://github.com/librastartup/libra-canonical-serialization) resource viewer.
* [executor](/executor/) - launch and test Move code without sending transactions.

Supported projects and dialects:

* [Diem](https://www.diem.com/en-us/)
* [Dfinance](https://dfinance.co/)
* [Pontem](https://pontem.network/)

Clone this repository and follow documentation:

```shell script
git clone git@github.com:dfinance/move-tools.git
cd move-tools
```

## Dove

Installation with **Polkadot** support:

```shell script
cargo install --bin=dove --path dove --features="ps_address" --no-default-features
```

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
dove new first_project --dialect polkadot
```

`dialect` - dialect of the Move language. Either `move` (for original Libra version) or `dfinance` (bech32 addresses and some other stuff), or `polkadot`. Default is `move`.

Build project:

```shell script
dove build
```
See `./target/` folder to get scripts/modules binaries.


Create transactions:

Command `ct` allows you to create transactions for `polkadot` chain with move vm palette. 

`ct` takes script identifier, type parameters, and arguments and creates a transaction file as an artifact of work.

Example:
```shell script
dove ct 'store_u64(60)'
```
This command searches for the script by name 'store_u64' in the script directory. Then it compiles it and creates a transaction file.

This command will fail if:

- There is no script with the name given name 'store_u64'.
- There is more than one script with the name 'store_64'.
- The passed parameters or type parameters do not match the script parameters.
- There are syntax errors in the script.

Type parameters:

You can use type parameters like in the move language.
Example:
```shell script
dove ct 'create_account<0x01::Dfinance::USD, 0x01::Dfinance::BTC>()'
```
You allow can use ss58 address format:
```shell script
dove ct 'create_account<1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE::Dfinance::USD>()'
```

Types:

numbers (u8, u64, u128): 10, 1024. 

bool: true, false.

address: 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE, 0x1CF326C5AAA5AF9F0E2791E66310FE8F044FAADAF12567EAA0976959D1F7731F

vector<address>: [1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE, 0x1CF326C5AAA5AF9F0E2791E66310FE8F044FAADAF12567EAA0976959D1F7731F, 0x01]

vector<u8/u64/128>: [10, 30, 1024]

vector<bool>: [true, false]

You can define or override script names by using '--name' or '-n' parameter.

Example:
Override script name:
```shell script
dove ct 'store_u64(60)' -n store_u126
```
Define script name:
```shell script
dove ct -n store_u126
```

File name:
You can define the file name by using '--file' or '-f' parameter.
With this option 'ct' searches in a specified file. It may be useful when there is more than one script with the same name in different files. 
Or the specified file has one script.
```shell script
dove ct 'store_u64(60)' -n store_u126 -f script.move
```
```shell script
dove ct -n store_u126 -f script
```

Type parameters:

You can define or override script type parameters by using '--type' or '-t' parameter.
```shell script
dove ct 'store_u64()' -t 0x01::Dfinance::USD u8
```
```shell script
dove ct -n store_u64 -t 0x01::Dfinance::USD u8
```


arguments:

You can define or override script arguments by using '--args' or '-a' parameter.
```shell script
dove ct 'store_u64()' -a [10, 1024] 10 0x01
```
```shell script
dove ct -n store_u64 -a [10, 1024] 10 0x01
```

## Resource Viewer

See [documentation](/resource-viewer/README.md).

## Language Server

Implementation of Language Server Protocol for [Move language](https://developers.libra.org/docs/crates/move-language).

```shell script
RUST_LOG=info cargo run --bin move-language-server
```

Features:
* check source code files with the official compiler on-the-fly

For the corresponding VSCode extension, see https://marketplace.visualstudio.com/items?itemName=damirka.move-ide

#### Configuration

`dialect` - dialect of the Move language. Either `move` (for original Libra version) or `dfinance` (bech32 addresses and some other stuff). Default is `move`.

`sender_address` - address of the user, used for module imports. Default is `0x0`.

`stdlib_folder` - stdlib folder path. Default is `null`, no stdlib is loaded.

`modules_folders` - array of folder paths for module lookup. Default is empty array.

## Executor

Install **executor**:

```shell script
cargo install --path executor
```
See help:
```
executor -h
```

## LICENSE

[LICENSE](/LICENSE)
