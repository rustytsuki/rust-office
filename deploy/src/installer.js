import path from 'path';
import fs from 'fs-extra';
import childProcess from 'child_process';
import * as utils from './utils.js'

export async function install_target(target) {
    if (target.indexOf('windows') >= 0) {
        return await install_windows(target);
    } else if (target.indexOf('apple') >= 0) {
        return await install_macos(target);
    }
}

async function install_windows(target) {
    // copy vcpkg dll
    const vcpkg_dlls = utils.get_windows_vcpkg_dlls();
    const vcpkg_bin_dir = utils.get_vcpkg_bin_abs_path(target);
    const kernel_dir = utils.get_kernel_abs_path(target);
    await fs.ensureDir(kernel_dir);
    for (let i in vcpkg_dlls) {
        const src = path.resolve(vcpkg_bin_dir, vcpkg_dlls[i]);
        const dst = path.resolve(kernel_dir, vcpkg_dlls[i]);
        await fs.copy(src, dst);
    }

    // // copy vcpkg fonts folder
    // const fonts_src = path.resolve(utils.get_vcpkg_abs_path(target), 'etc/fonts');
    // console.log('----------', fonts_src);
    // const fonts_dst = path.resolve(kernel_dir, 'fonts');
    // await fs.ensureDir(fonts_dst);
    // await fs.copy(fonts_src, fonts_dst);
    
    // copy engine lib
    const engine_lib = utils.get_windows_kernel_libs();
    const release_deps_dir = utils.get_release_deps_abs_path(target);
    for (let i in engine_lib) {
        const src = path.resolve(release_deps_dir, engine_lib[i]);
        const dst = path.resolve(kernel_dir, engine_lib[i]);
        await fs.copy(src, dst);
    }
}

async function install_macos(target) {
    // copy vcpkg dylibs
    const vcpkg_dylibs = utils.get_mac_vcpkg_dylibs();
    const vcpkg_bin_dir = utils.get_vcpkg_bin_abs_path(target);
    const kernel_dir = utils.get_kernel_abs_path(target);
    await fs.ensureDir(kernel_dir);
    for (let i in vcpkg_dylibs) {
        const src = path.resolve(vcpkg_bin_dir, vcpkg_dylibs[i]);
        const dst = path.resolve(kernel_dir, vcpkg_dylibs[i]);
        await fs.copy(src, dst, { "dereference": true });
        // await modify_dylib_LC_ID_DYLIB(src, `@executable_path/${vcpkg_dylibs[i]}`); // used for new vcpkg installed
        await modify_dylib_LC_ID_DYLIB(dst, `@executable_path/${vcpkg_dylibs[i]}`);
    }

    // copy engine lib
    const engine_lib = utils.get_mac_kernel_libs();
    const release_deps_dir = utils.get_release_deps_abs_path(target);
    for (let i in engine_lib) {
        const src = path.resolve(release_deps_dir, engine_lib[i]);
        const dst = path.resolve(kernel_dir, engine_lib[i]);
        await fs.copy(src, dst);
        await modify_dylib_LC_ID_DYLIB(dst, `@executable_path/${engine_lib[i]}`);
    }
}

async function modify_dylib_LC_ID_DYLIB(file, lc_id_dylib) {
    let cmd = `install_name_tool -id ${lc_id_dylib} ${file}`;

    let options = {
        'cwd': utils.get_project_root_abs_path(),
        'shell': '/bin/zsh'
    };

    return new Promise((resolve) => {
        let ps = childProcess.spawn(cmd, options);

        ps.stdout.on('data', (data) => {
            console.log(data.toString());
        });

        ps.stderr.on('data', (data) => {
            console.error(data.toString());
        });

        ps.on('close', (code) => {
            console.log(`modify dylib LC_ID_DYLIB exit code ${code}`);
            resolve(code);
        });
    });
}