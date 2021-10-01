// @todo remove

import init from '../pkg/client.js';
import * as sidebar from './sidebar.js';

export default async function run() {
    await init();
    await sidebar.init();
}