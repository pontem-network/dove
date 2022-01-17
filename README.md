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

## Executor

Migrated inside Dove, see help:

```shell script
dove run --help
```

## Resource Viewer
Move Resource Viewer is a tool to query [BCS](https://github.com/diem/bcs) resources data from blockchain nodes storage and represent them in JSON or human readable format.

Supported nodes:
* [pontem](https://github.com/pontem-network/pontem)
* [dnode](http://github.com/dfinance/dnode)
* [diem](https://github.com/diem/diem)

1. The viewer makes a request to the blockchain node by a sending specific query (address + resource type).
2. The viewer send another request to node and query resource type layout.
3. The viewer restores resources using response data and type layout.

## Usage example

Query the user's store contract balance:

```bash
dove view --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY  --query "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY::Store::Store<u64>" --api "ws://127.0.0.1:9946"
```

### Input parameters

- `--address` can be in Pontem [ss58][], Dfinance [bech32][] or hex `0x…{16-20 bytes}`.
- `-q` / `--query` resource type-path, e.g.:
    - `0x1::Account::Balance<0x1::PONT::PONT>`
    - `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY::Store::Store<u64>`
    - In general: `0xDEADBEEF::Module::Struct< 0xBADBEEF::Mod::Struct<...>, ... >`
    - Inner address can be omitted, it's inherited by parent:
      `0xDEADBEEF::Module::Struct<Mod::Struct>` expands to `0xDEADBEEF::Module::Struct<0xDEADBEEF::Mod::Struct>`
    - Query can ends with index `[42]` for `vec`-resources
- Output options:
    - `-o` / `--output` fs-path to output file
    - `-j` / `--json` sets output format to json. Can be omitted if output file extension is `.json`, so then json format will be chosen automatically.
    - `--json-schema` additional json-schema export, fs-path to output schema file.

For more info check out `--help`.

### Output

Two output formats supported:

- Move-like text
- JSON

_The structure of the output in JSON is described in the scheme, which can be obtained by calling with the `--json-schema` parameter._

#### Move-like example:

```rust
resource 00000000::Account::Balance<00000000::Coins::BTC> {
    coin: resource 00000000::Dfinance::T<00000000::Coins::BTC> {
        value: 1000000000u128
    }
}
```

#### JSON example:

```json
{
  "is_resource": true,
  "type": {
    "address": "0000000000000000000000000000000000000001",
    "module": "Account",
    "name": "Balance",
    "type_params": [
      {
        "Struct": {
          "address": "0000000000000000000000000000000000000001",
          "module": "Coins",
          "name": "BTC",
          "type_params": []
        }
      }
    ]
  },
  "value": [
    {
      "id": "coin",
      "value": {
        "Struct": {
          "is_resource": true,
          "type": {
            "address": "0000000000000000000000000000000000000001",
            "module": "Dfinance",
            "name": "T",
            "type_params": [
              {
                "Struct": {
                  "address": "0000000000000000000000000000000000000001",
                  "module": "Coins",
                  "name": "BTC",
                  "type_params": []
                }
              }
            ]
          },
          "value": [
            {
              "id": "value",
              "value": {
                "U128": 1000000000
              }
            }
          ]
        }
      }
    }
  ]
}
```

## Publishing a module or package

```bash
$ dove publish [TYPE] --file [FILE_NAME] --gas [GAS]  --secret [KEY PHRASE] --account [ADDRESS] --url [URL]
```
### Input parameters
- [TYPE] file type
  - module
  - package
- `-f` / `--file` Path to the transaction
- `-g` / `--gas` Limitation of gas consumption per operation. A positive integer is expected
-  `-u` / `--url` The url of the substrate node to query [default: ws://localhost:9944]. HTTP, HTTPS, WS protocols are supported. It is recommended to use WS. When using HTTP or HTTPS, you cannot get the publication status.  
- `-t` / `--account` Test account from who to publish. Example: //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY. Only for text publications. When used in combination with `--secret` is ignored. 
- `-s` / `--secret` Secret phrase. If a secret phrase is specified, you do not need to specify.

### Examples:
```bash
$ dove publish module --file PATH/TO/MODULE.mv --gas 100
$ dove publish package --file ./PATH/TO/PACKAGE.pac --gas 300 --account 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
$ dove publish module --file /PATH/TO/MODULE.mv --gas 200 --account alice --url ws://127.0.0.1:9944
$ dove publish module --file /PATH/TO/MODULE.mv --gas 200 --secret "net exotic exchange stadium..."
```

## Execute a transaction

```bash
$ dove execute --file [FILE_NAME] --gas [GAS] --secret [KEY PHRASE] --account [ADDRESS] --url [URL]
```
### Input parameters
- `-f` / `--file` Path to the transaction
- `-g` / `--gas` Limitation of gas consumption per operation. A positive integer is expected
-  `-u` / `--url` The url of the substrate node to query [default: ws://localhost:9944]. HTTP, HTTPS, WS protocols are supported. It is recommended to use WS. When using HTTP or HTTPS, you cannot get the publication status.
- `-t` / `--account` Test account from who to publish. Example: //Alice, alice, bob... or 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY. Only for text publications. When used in combination with `--secret` is ignored.
- `-s` / `--secret` Secret phrase. If a secret phrase is specified, you do not need to specify.

### Examples:
```bash
$ dove execute --file PATH/TO/TRANSACTION.mvt  --gas 120 
$ dove execute --file ./PATH/TO/TRANSACTION.mvt --gas 220 --account 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 
$ dove execute --file /PATH/TO/TRANSACTION.mvt --gas 110 --account alice --url ws://127.0.0.1:9944
$ dove execute --file /PATH/TO/TRANSACTION.mvt --gas 140 --secret "net exotic exchange stadium..."
```

## LICENSE

[LICENSE](/LICENSE)
