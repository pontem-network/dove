use std::path::{Path, PathBuf};
use anyhow::Error;
use dialect::Dialect;
use structopt::StructOpt;
use http::Uri;
use log::{error, info};
use move_cli::Move;
use move_core_types::language_storage::TypeTag;
use move_lang::Flags;
use move_lang::parser::lexer::Lexer;
use move_lang::shared::CompilationEnv;
use move_symbol_pool::Symbol;
use move_resource_viewer::ser;
use net::{make_net, NetView};

use crate::cmd::{Cmd, default_sourcemanifest};
use crate::context::Context;
use crate::tx::parser::parse_type_param;

const STDOUT_PATH: &str = "-";

/// Move Resource Viewer
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(name = "Move resource viewer")]
pub struct View {
    /// Owner's address
    #[structopt(long)]
    address: String,

    /// Query in `TypeTag` format,
    /// one-line address+type description.
    /// Mainly, in most cases should be StructTag.
    /// Additionaly can contain index at the end.
    /// Query examples:
    /// "0x1::Account::Balance<0x1::XFI::T>",
    /// "0x1::Account::Balance<0x1::Coins::ETH>"
    #[structopt(long, short)]
    query: String,
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
    api: Uri,

    /// Export JSON schema for output format.
    /// Special value for write to stdout: "-"
    #[structopt(long = "json-schema")]
    json_schema: Option<PathBuf>,
}

impl Cmd for View {
    fn context(&mut self, project_dir: PathBuf, move_args: Move) -> anyhow::Result<Context> {
        Ok(Context {
            project_dir,
            move_args,
            manifest: default_sourcemanifest(),
            manifest_hash: 0,
        })
    }

    fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        let dialect = ctx.move_args.dialect.unwrap_or(Dialect::Pont);
        if let Some(path) = self.json_schema.as_ref() {
            produce_json_schema(path);
        }
        let net = make_net(self.api.clone(), dialect)?;

        let output = self.output.clone();
        let height = self.height.clone();

        let mut query_string = self.query.clone();
        if !query_string.starts_with("0x") {
            if let Some(pos) = query_string.find("::") {
                let address = dialect.parse_address(&query_string[..pos])?;
                query_string = format!("{}{}", address.to_hex_literal(), &query_string[pos..]);
            }
        }

        let query = parse_query(&query_string)?;
        let addr = dialect.parse_address(&self.address)?;

        match query {
            TypeTag::Struct(st) => {
                net.get_resource(&addr, &st, &height)
                    .map(|resp| {
                        let view = NetView::new(net, height);
                        if let Some(bytes_for_block) = resp {
                            // Internally produce FatStructType (with layout) for StructTag by
                            // resolving & de-.. entire deps-chain.
                            let annotator = resource_viewer::MoveValueAnnotator::new(&view);

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

fn parse_query(query: &str) -> Result<TypeTag, Error> {
    use move_lang::parser::syntax::Context;

    let mut lexer = Lexer::new(query, Symbol::from("query"));
    let mut env = CompilationEnv::new(Flags::empty(), Default::default());
    let mut ctx = Context::new(&mut env, &mut lexer);

    ctx.tokens.advance().map_err(|err| anyhow!("{:?}", &err))?;

    parse_type_param(&mut ctx)
}
