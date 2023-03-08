import commandLineArgs from 'command-line-args';
import * as builder from './src/builder.js';
import * as installer from './src/installer.js';
import * as uploader from './src/uploader.js';

const optionDefinitions = [
    { name: 'target', alias: 't', type: String },
    { name: 'upload', alias: 'u', type: Boolean},
];

const config = commandLineArgs(optionDefinitions);

async function main() {
    let targets = config['target'].split(',');
    for (let i in targets) {
        await build_kernel(targets[i]);
    }
}

async function build_kernel(target) {
    if (await builder.build_target(target)) {
        return;
    }

    if (await installer.install_target(target)) {
        return;
    }

    if (config['upload']) {
        if (await uploader.upload_target(target)) {

        }
    }
}

main();
