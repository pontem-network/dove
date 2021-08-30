# Move Resource Viewer

**Resource viewer is currently out of date and pending migration inside dove in future versions.**

Move Resource Viewer is a tool to query [BCS](https://github.com/diem/bcs) resources data from blockchain nodes storage and represent them in JSON or human readable format.

Supported nodes:
* [pontem](https://github.com/pontem-network/pontem)
* [dnode](http://github.com/dfinance/dnode)
* [diem](https://github.com/diem/diem)

## How does it works?

1. The viewer makes a request to the blockchain node by a sending specific query (address + resource type).
2. The viewer send another request to node and query resource type layout.
3. The viewer restores resources using response data and type layout.

## Installation

Requirements:
- [Rust][] toolchain, the easiest way to get it is to use [Rustup][].

Using cargo:

```bash
cargo install --git https://github.com/pontem-network/move-tools.git move-resource-viewer
```

[Rust]: https://www.rust-lang.org
[Rustup]: https://rustup.rs


[DFinance]: https://github.com/dfinance
[diem/Diem]: https://github.com/diem
[SS58]: "https://github.com/paritytech/substrate/wiki/External-Address-Format-(SS58)"


## Usage example

Query the user's store contract balance:

```bash
move-resource-viewer --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY  --query "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY::Store::Store<u64>" --api="ws://127.0.0.1:9946"
```

### Input parameters

- `-a` / `--account` can be in Pontem [ss58][], Dfinance [bech32][] or hex `0x…{16-20 bytes}`.
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

[dnode]: https://github.com/dfinance/dnode
[bech32]: https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki

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
