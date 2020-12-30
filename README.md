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
