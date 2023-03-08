import simpleGit from 'simple-git';
import path from 'path';
import fs from 'fs-extra';
import * as utils from './utils.js';

const git = simpleGit({
    baseDir: utils.get_project_root_abs_path(),
    binary: 'git'
});

export async function upload_target(target) {
    try {
        let result = await git.pull();
        console.log(`pull result: ${JSON.stringify(result)}`);

        let committed_count = await commit_target(target);
        if (committed_count > 0) {
            let push_result = await git.push();
            console.log(`push result: ${JSON.stringify(push_result)}`);
        } else {
            console.log('nothing should be committed.');
        }
    } catch (e) {
        console.error(`upload failed! ${e}`);
        return 1;
    }

    return 0;
}

async function commit_target(target) {
    // add
    let file_list = get_file_list(target);

    for (let i in file_list) {
        let file_path = path.resolve(utils.get_kernel_abs_path(target), file_list[i]);
        await git.add(file_path);
    }

    // commit
    let result = await git.status();
    if (result.staged.length > 0) {
        let commit_result = await git.commit(`automatic build for ${target}`);
        console.log(`commit result: ${JSON.stringify(commit_result)}`);
        return result.staged.length;
    }

    return 0;
}

function get_file_list(target) {
    let file_list = [];

    if (target.indexOf('windows') >= 0) {
        file_list = utils.get_windows_vcpkg_dlls().concat(utils.get_windows_kernel_libs());
    } else if (target.indexOf('apple') >= 0) {
        file_list = utils.get_mac_vcpkg_dylibs().concat(utils.get_mac_kernel_libs());
    } else if (target.indexOf('wasm') >= 0) {
        file_list = utils.get_web_kernel_libs();
    }

    return file_list;
}