import init, { build, make_abi, module_abi, tx } from '../../pkg/dovelight.js';
import * as sidebar from './sidebar.js';

async function run() {
    sidebar.init();

    await init();


    let sources = new Map();
    let source_map = { source_map: sources };
    sources["version.module.move"] = "module 0x1::Version { public fun get():u8 { 8 } }";
    sources["empty.module.move"] = "module 0x1::Empty { public fun l() { } }";
    sources["version.script.move"] = "script { use 0x1::Version; fun version(){ let _v = Version::get(); }}";

    console.log(build("http://localhost:9933/", source_map, "pont", "0x1"));
    console.log(tx("http://localhost:9933/", source_map, "pont", "version()"));


    // sources = new Map();
    // source_map = { source_map: sources };
    // sources["script.move"] = "script {fun main(){}}";
    // sources["module.move"] = "module 0x1::Foo {" +
    //     "use 0x1::DiemTimestamp;" +
    //     "public fun test() { " +
    //     "   DiemTimestamp::now_microseconds(); " +
    //     "}" +
    //     "}";

    // console.log("build");
    // console.log(build("http://localhost:9933/", source_map, "pont", "0x1"));

    // sources = new Map();
    // source_map = { source_map: sources };
    // sources["module.move"] = "module 0x1::Foo {" +
    //     "use 0x1::DiemTimestamp;" +
    //     "public fun test(){" +
    //     "   DiemT imestamp::now_microseconds();" +
    //     "}" +
    //     "}";


    // console.log("make_abi");
    // console.log(make_abi("http://localhost:9933/", source_map, "pont", "0x1"));
    // console.log("module_abi");
    // console.log(module_abi("http://localhost:9933/", "pont", "0x1", "DiemTimestamp"));
}

run();