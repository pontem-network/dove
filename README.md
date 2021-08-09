# Move tools

Toolset for work with Move language based projects:

* [dove](/dove/) - package manager and compiler.
* [language server](/language_server/) - Move language server.
* [resource viewer](/resource-viewer/) - [LCS](https://github.com/diemstartup/diem-canonical-serialization) resource viewer.
* [executor](/executor/) - launch and test Move code without sending transactions.

Supported projects and dialects:

* [Diem](https://www.diem.com/en-us/)
* [Pontem](https://pontem.network/)
* [Dfinance](https://dfinance.co/)

Clone this repository and follow documentation:

```shell script
git clone git@github.com:pontem-network/move-tools.git
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

`dialect` - dialect of the Move language. Either `move` (for original diem version)  or `polkadot`, or `dfinance` (bech32 addresses and some other stuff). Default is `move`.

Build project:

```shell script
dove build
```
See `./target/` folder to get scripts/modules binaries.

### Arguments

Command `tx` allows you to create transactions for `polkadot` chain with [Move Pallete](https://github.com/pontem-network/sp-move).

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

You can use type parameters like in the move language.

Example:
```shell script
dove tx 'create_account<0x01::PONT::T, 0x01::Coins::USDT>()'
```

You allow can use ss58 address format:
```shell script
dove tx 'create_account<1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE::MyToken::Token>()'
```

Supported types:

**Numbers (u8, u64, u128):**

```shell script
dove tx 'my_script(10, 1024)'
```

**Boolean:**

```shell script
dove tx 'my_script(true, false)'
```

**Addresses:**

```shell script
dove tx 'my_script(1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE, 0x1CF326C5AAA5AF9F0E2791E66310FE8F044FAADAF12567EAA0976959D1F7731F)'
```

**Vectors:**

```shell script
dove tx 'my_script([10, 20, 1024])' // Vector u64
dove tx 'my_script([1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE, 0x1CF326C5AAA5AF9F0E2791E66310FE8F044FAADAF12567EAA0976959D1F7731F, 0x01])' // Vector addresses.
```

You can define or override script arguments by using '--args' or '-a' parameter.

Example:

```shell script
dove tx 'store_u64()' -a [10, 1024] 10 0x01
```
```shell script
dove tx -n store_u64 -a [10, 1024] 10 0x01
```

**Script name**

You can define or override script names by using '--name' or '-n' parameter.

Example:

```shell script
dove tx 'store_u64(60)' -n store_u126
```

Define script name:

```shell script
dove tx -n store_u126
```

**Script file**

You can define the file name by using '--file' or '-f' parameter.

With this option 'tx' searches in a specified file. It may be useful when there is more than one script with the same name in different files.
Or the specified file has one script.

Example:

```shell script
dove tx 'store_u64(60)' -n store_u126 -f script.move
```
```shell script
dove tx -n store_u126 -f script
```

**Types**

You can define or override script type parameters by using '--type' or '-t' parameter.

Example:

```shell script
dove tx 'store_u64()' -t 0x01::Coins::USDT u8
```
```shell script
dove tx -n store_u64 -t 0x01::Coin::USDT u8
```

## Resource Viewer

See [documentation](/resource-viewer/README.md).

## Language Server

Implementation of Language Server Protocol for [Move language](https://developers.diem.org/docs/crates/move-language).

```shell script
RUST_LOG=info cargo run --bin move-language-server
```

Features:
* check source code files with the official compiler on-the-fly

For the corresponding VSCode extension, see https://marketplace.visualstudio.com/items?itemName=damirka.move-ide

#### Configuration

`dialect` - dialect of the Move language. Either `move` (for original diem version) or `polkadot` (ss58), or ` (bech32 addresses and some other stuff). Default is `move`.

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
