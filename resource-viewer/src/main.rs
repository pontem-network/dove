// Simple querie examples:
// "0x1::Account::Balance<0x1::PONT::T>"
// "0x1::Account::Balance<0x1::XFI::T>",
// "0x1::Account::Balance<0x1::Coins::ETH>",
// "0x1::Account::Balance<0x1::Coins::BTC>",
// "0x1::Account::Balance<0x1::Coins::USDT>",
// "0x1::Account::Balance<0x1::Coins::SXFI>",

#[macro_use]
extern crate log;

use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use clap::Clap;
use http::Uri;
use move_core_types::language_storage::TypeTag;
use resource_viewer as rv;

use lang::compiler::dialects::DialectName;
use move_resource_viewer::{ser, tte};
use net::{make_net, NetView};

#[cfg(feature = "json-schema")]
const STDOUT_PATH: &str = "-";
const VERSION: &str = git_hash::crate_version_with_git_hash_short!();

#[derive(Clap, Debug)]
#[clap(name = "Move resource viewer", version = VERSION)]
struct Cfg {
    /// Owner's address
    #[clap(long, short)]
    address: String,
    #[clap(default_value = "pont")]
    dialect: String,

    /// Query in `TypeTag` format,
    /// one-line address+type description.
    /// Mainly, in most cases should be StructTag.
    /// Additionaly can contain index at the end.
    /// Query examples:
    /// "0x1::Account::Balance<0x1::XFI::T>",
    /// "0x1::Account::Balance<0x1::Coins::ETH>"
    #[clap(long, short)]
    query: tte::TypeTagQuery,

    /// Time: maximum block number
    #[clap(long, short)]
    height: Option<String>,

    /// Output file path.
    /// Special value for write to stdout: "-"
    #[clap(long, short)]
    #[clap(default_value = STDOUT_PATH)]
    output: PathBuf,

    /// Sets output format to JSON.
    /// Optional, `true` if output file extension is .json
    #[clap(long, short)]
    json: Option<bool>,

    /// Node REST API address
    #[clap(long)]
    api: Uri,

    /// Enables compatibility mode
    #[clap(long, short)]
    compat: bool,

    /// Export JSON schema for output format.
    /// Special value for write to stdout: "-"
    #[cfg(feature = "json-schema")]
    #[clap(long = "json-schema")]
    json_schema: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    init_logger()
        .map_err(|err| eprintln!("Error: {}", err))
        .ok();
    run().map_err(|err| {
        error!("{}", err);
        err
    })
}

fn init_logger() -> Result<(), impl std::error::Error> {
    use env_logger::*;

    let mut builder = Builder::from_env(Env::default());
    builder.format_timestamp(None);
    builder.try_init()
}

fn run() -> Result<(), Error> {
    let cfg = Cfg::parse();

    let dialect_name = DialectName::from_str(&cfg.dialect)?;

    produce_json_schema(&cfg);
    let net = make_net(cfg.api, dialect_name)?;
    let dialect = dialect_name.get_dialect();

    let output = cfg.output;
    let height = cfg.height;
    let json = cfg.json.unwrap_or_else(|| {
        output
            .extension()
            .map(|ext| ext == "json")
            .unwrap_or_default()
    });
    let (tte, index) = cfg.query.into_inner();
    let addr = dialect.as_ref().parse_address(&cfg.address)?;

    match tte {
        TypeTag::Struct(st) => {
            net.get_resource(&addr, &st, &height)
                .map(|resp| {
                    let view = NetView::new(net, height);
                    if let Some(bytes_for_block) = resp {
                        // Internally produce FatStructType (with layout) for StructTag by
                        // resolving & de-.. entire deps-chain.
                        let annotator = rv::MoveValueAnnotator::new_no_stdlib(&view);

                        annotator
                            .view_resource(&st, &bytes_for_block.0)
                            .and_then(|result| {
                                let height = bytes_for_block.1;

                                if json {
                                    serde_json::ser::to_string_pretty(
                                        &ser::AnnotatedMoveStructWrapper { height, result },
                                    )
                                    .map_err(|err| anyhow!("{}", err))
                                } else {
                                    Ok(format!("{}", result))
                                }
                            })
                            .map(|result| write_output(&output, &result, "result"))
                    } else {
                        Err(anyhow!("Resource not found, result is empty"))
                    }
                })
                .and_then(|result| result)
        }

        TypeTag::Vector(tt) => Err(anyhow!(
            "Unsupported root type Vec<{}>{:?}",
            tt,
            index.map(|v| [v]).unwrap_or_default()
        )),

        _ => Err(anyhow!("Unsupported type {}", tte)),
    }
}

#[allow(unused_variables)]
fn produce_json_schema(cfg: &Cfg) {
    #[cfg(feature = "json-schema")]
    if let Some(path) = cfg.json_schema.as_ref() {
        let schema = ser::produce_json_schema();
        let render = serde_json::to_string_pretty(&schema).unwrap();
        write_output(&path, &render, "schema");
    }
}

fn write_output(path: &Path, result: &str, name: &str) {
    use std::io::prelude::*;
    if path.as_os_str() == STDOUT_PATH {
        println!("{}", &result);
    } else {
        std::fs::File::create(path)
            .and_then(|mut f| f.write_all(result.as_bytes()))
            .map_err(|err| error!("Cannot write output: {}", err))
            .map(|_| info!("File with {} was written successfully", name))
            .ok();
    }
}
