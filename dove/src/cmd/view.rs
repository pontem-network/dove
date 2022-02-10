use std::path::{Path, PathBuf};
use anyhow::Error;
use structopt::StructOpt;
use log::{error, info};
use reqwest::Url;

use move_core_types::language_storage::TypeTag;
use move_package::source_package::parsed_manifest::{AddressDeclarations, NamedAddress};

use lang::ss58::ss58_to_address;
use resource_viewer::ser;
use net::{make_net, NetView};

use crate::context::Context;
use crate::call::parser::parse_type_param;

const STDOUT_PATH: &str = "-";

/// Move Resource Viewer
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct View {
    /// Query in `TypeTag` format,
    /// one-line address+type description.
    /// Mainly, in most cases should be StructTag.
    /// Additionaly can contain index at the end.
    /// Query examples:
    /// "0x1::Account::Balance<0x1::XFI::T>",
    /// "0x1::Account::Balance<0x1::Coins::ETH>"
    #[structopt(long = "query", short)]
    query_string: String,

    /// Time: maximum block number
    #[structopt(long, short)]
    height: Option<String>,

    /// Output file path.
    /// Special value for write to stdout: "-"
    #[structopt(long, short)]
    #[structopt(default_value = STDOUT_PATH)]
    output: PathBuf,

    /// Sets output format to JSON.
    /// Optional, `true` if output file extension is .json
    #[structopt(long, short)]
    json: bool,

    /// Node REST API address
    #[structopt(long)]
    api: Url,

    /// Export JSON schema for output format.
    /// Special value for write to stdout: "-"
    #[structopt(long = "json-schema")]
    json_schema: Option<PathBuf>,
}

impl View {
    pub fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        if let Some(path) = self.json_schema.as_ref() {
            produce_json_schema(path);
        }

        let output = self.output.clone();
        let height = self.height.clone();
        let net = make_net(self.api.clone())?;
        let address_map = ctx.manifest.addresses.clone().unwrap_or_default();

        if !self.query_string.starts_with("0x") {
            if let Some(pos) = self.query_string.find("::") {
                let name_address = &self.query_string[..pos];
                let address = address_map
                    .get(&NamedAddress::from(name_address))
                    .map(|v| {
                        v.ok_or(anyhow!(
                            "In Move.toml address not assigned to alias {}",
                            name_address
                        ))
                    })
                    .unwrap_or_else(|| ss58_to_address(name_address))?;

                self.query_string =
                    format!("{}{}", address.to_hex_literal(), &self.query_string[pos..]);
            }
        }
        let query = parse_query(&address_map, &self.query_string)?;

        match query {
            TypeTag::Struct(st) => {
                let addr = st.address;

                net.get_resource(&addr, &st, &height)
                    .map(|resp| {
                        let view = NetView::new(net, height);
                        if let Some(bytes_for_block) = resp {
                            // Internally produce FatStructType (with layout) for StructTag by
                            // resolving & de-.. entire deps-chain.
                            let annotator = move_resource_viewer::MoveValueAnnotator::new(&view);

                            annotator
                                .view_resource(&st, &bytes_for_block.0)
                                .and_then(|result| {
                                    let height = bytes_for_block.1;

                                    if self.json {
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
                            bail!("Resource not found, result is empty")
                        }
                    })
                    .and_then(|result| result)
            }
            TypeTag::Vector(list_types) => bail!("Unsupported root type Vec {:?}", list_types),
            _ => bail!("Unsupported type {}", query),
        }
    }
}

fn produce_json_schema(path: &Path) {
    let schema = ser::produce_json_schema();
    let render = serde_json::to_string_pretty(&schema).unwrap();
    write_output(path, &render, "schema");
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

/// Query parsing
///     addr_map:&AddressDeclarations - To check alias addresses and replace with a hexadecimal address
///     qyery - Query string for parsing
fn parse_query(addr_map: &AddressDeclarations, query: &str) -> Result<TypeTag, Error> {
    use move_command_line_common::files::FileHash;
    use move_compiler::shared::CompilationEnv;
    use move_compiler::parser::syntax::Context;
    use move_compiler::parser::lexer::Lexer;
    use move_compiler::Flags;

    let mut lexer = Lexer::new(query, FileHash::new(query));
    let mut env = CompilationEnv::new(Flags::empty(), Default::default());
    let mut ctx = Context::new(&mut env, &mut lexer);

    ctx.tokens.advance().map_err(|err| anyhow!("{:?}", &err))?;

    parse_type_param(addr_map, &mut ctx)
}
