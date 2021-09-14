import init, * as wasm from '../pkg/client.js';
import * as sidebar from './sidebar.js';

export default async function run() {
    await init();
    sidebar.init();
}
