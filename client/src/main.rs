mod dove;
mod env;

use anyhow::Error;
use wasmer::{Function, imports, Instance, Module, Store, ChainableNamedResolver};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;
use wasmer_wasi::WasiState;
use crate::dove::{Dove, SourceMap};
use crate::env::init;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = init_wasm(include_bytes!(
        "../../dovelight/target/wasm32-wasi/release/dovelight.wasm"
    ))?;
    let dove = Dove::new(&instance)?;
    let tx = dove.tx(
        "http://localhost:9933/".to_string(),
        source_map(),
        "pont".to_string(),
        "main()".to_string(),
    )?;
    println!("tx: {:?}", tx);
    Ok(())
}

fn source_map() -> SourceMap {
    let mut map = SourceMap {
        source_map: Default::default(),
    };
    map.source_map.insert(
        "script.move".to_string(),
        "\
    script {
       use 0x1::DiemTimestamp;
       use 0x1::Foo;
       fun main() {
            DiemTimestamp::now_microseconds();
            Foo::get_num();
       }
    }
    "
        .to_string(),
    );

    map.source_map.insert(
        "module.move".to_string(),
        "\
        module 0x1::Foo {
            public fun get_num(): u64 {
                10
            }
        }
    "
        .to_string(),
    );

    map
}

fn init_wasm(module: &[u8]) -> Result<Instance, Error> {
    let store = Store::new(&Universal::new(Cranelift::default()).engine());
    let module = Module::new(&store, module)?;
    let mut wasi_env = WasiState::new("dove").finalize()?;
    let import_object = imports! {
        "env" => {
            "_log" => Function::new_native(&store, env::log),
            "send_http_request" => Function::new_native(&store, env::send_http_request),
            "store" => Function::new_native(&store, env::store),
            "drop" => Function::new_native(&store, env::drop),
            "load" => Function::new_native(&store, env::load),
        }
    };

    let rt_import_object = wasi_env.import_object(&module)?;
    let io = rt_import_object.chain_front(import_object);
    let instance = Instance::new(&module, &io)?;
    init(&instance)?;
    Ok(instance)
}
