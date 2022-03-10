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

/// Move Resource Viewer
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(usage = "dove view [QUERY] [OPTIONS]
    Examples:
    $ dove view Account::Store::U64
    $ dove view Account::Store::U64 --api http://127.0.0.1:9933
    $ dove view Account::Store::U64 --api http://127.0.0.1:9933 --json
    $ dove view 0x1::Account::Balance<0x1::Coins::ETH> --api http://127.0.0.1:9933 --json --output PATH/SAVE.json
")]
pub struct View {
    #[structopt(
        display_order = 1,
        help = "Fully qualified type description in a form of ADDRESS::MODULE::TYPE_NAME<GENERIC_PARAMS> \n\
            Examples: \n\
            Account::Store::U64 \n\
            0x1::Account::Balance<0x1::Coins::ETH>"
    )]
    query: String,

    #[structopt(
        long,
        default_value = "http://127.0.0.1:9933",
        display_order = 2,
        help = "The url of the substrate node to query. HTTP or HTTPS only"
    )]
    api: Url,

    #[structopt(long, short, display_order = 3, help = "Sets output format to JSON")]
    json: bool,

    #[structopt(
        long = "json-schema",
        display_order = 4,
        help = "Export JSON schema for output format"
    )]
    json_schema: Option<PathBuf>,

    #[structopt(long, short, display_order = 5, help = "Path to output file")]
    output: Option<PathBuf>,

    #[structopt(long, short, display_order = 6, help = "Block number")]
    height: Option<String>,
}

impl View {
    pub fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        if let Some(path) = self.json_schema.as_ref() {
            produce_json_schema(path);
        }

        let height = self.height.clone();
        let net = make_net(self.api.clone())?;
        let address_map = ctx.manifest.addresses.clone().unwrap_or_default();

        if !self.query.starts_with("0x") {
            if let Some(pos) = self.query.find("::") {
                let name_address = &self.query[..pos];
                let address = address_map
                    .get(&NamedAddress::from(name_address))
                    .map(|acc| {
                        acc.ok_or(anyhow!(
                            "In Move.toml address not assigned to alias {}",
                            name_address
                        ))
                    })
                    .unwrap_or_else(|| ss58_to_address(name_address))?;

                self.query = format!("{}{}", address.to_hex_literal(), &self.query[pos..]);
            }
        }
        let query = parse_query(&address_map, &self.query)?;

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
                                .map(|result| {
                                    write_output(self.output.as_deref(), &result, "result")
                                })
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
    write_output(Some(path), &render, "schema");
}

fn write_output(path: Option<&Path>, result: &str, name: &str) {
    use std::io::prelude::*;

    if let Some(path) = path {
        std::fs::File::create(path)
            .and_then(|mut f| f.write_all(result.as_bytes()))
            .map_err(|err| error!("Cannot write output: {}", err))
            .map(|_| info!("File with {} was written successfully", name))
            .ok();
    } else {
        println!("{}", &result);
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
    let mut env = CompilationEnv::new(Flags::empty());
    let mut ctx = Context::new(&mut env, &mut lexer);

    ctx.tokens.advance().map_err(|err| anyhow!("{:?}", &err))?;

    parse_type_param(addr_map, &mut ctx)
}
