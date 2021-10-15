import init from '../../pkg/dovelight.js';
import { init as sidebar_init } from './sidebar.js';

export async function run() {
    sidebar_init();
    await init();


    // @todo remove dev
    // setTimeout(() => {
    //     document.querySelector(".project").click();
    //     setTimeout(() => {
    //         document.querySelector(".tx_script button").click();
    //         document.querySelector("#tx_list .item button").click();
    //     }, 50)
    // }, 100);

    // @todo remove
    // let sources = new Map();
    // let source_map = { source_map: sources };
    // sources["version.module.move"] = "module 0x1::Version { " +
    //     "public fun get():u8 { 8 } " +
    //     "public(script) fun version_run(){ fsadf let _r = 0x1::Version::get(); } " +
    //     "}";
    // sources["empty.module.move"] = "module 0x1::Empty { " +
    //     " public fun empty() { } " +
    //     "}";
    // sources["version.script.move"] = "script { " +
    //     "use 0x1::Version; " +
    //     "fun version(){ let _v = Version::get(); }" +
    //     "}";

    // console.log(build("http://localhost:9933/", source_map, "pont", "0x1"));
    // console.log(tx("http://localhost:9933/", source_map, "pont", "version()"));
    // let r = tx("http://localhost:9933/", source_map, "pont", "0x1::Version::version_run()");
    // console.log(r);


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