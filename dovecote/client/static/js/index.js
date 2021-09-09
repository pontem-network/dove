import init, * as wasm from '../pkg/client.js';
import * as sidebar from './sidebar.js';

let project_id;

export default async function run() {
    await init();
    sidebar.init();
    
    // TODO Remove id. This is bug demo.
    console.log("3000490687877993158 is expected. Actual is " + wasm.there_be_a_bug().id);
}
