import childProcess from 'child_process';
import path from 'path';
import fs from 'fs-extra';
import * as utils from './utils.js';

export async function build_target(target) {
    if (target.indexOf('wasm') >= 0) {
        return await build_web(target);
    }

    if (target.indexOf('windows') >= 0 && process.platform !== "win32") {
        console.error(`build target ${target} need on windows host.`);
        return 1;
    }
    
    if (target.indexOf('apple') >= 0 && process.platform !== "darwin") {
        console.error(`build target ${target} need on macos host.`);
        return 1;
    }

    const cmd_args = `build --release -p shell --features=glow,skia,skia_force_cpu,use_kernel_source --target=${target}`;
    let cmd = `cargo ${cmd_args}`;

    if (target.indexOf('windows') >= 0) {
        const is_i686 = target == 'i686-pc-windows-msvc';
        if (is_i686) {
            cmd = `cargo +nightly-x86_64-pc-windows-msvc ${cmd_args}`;
        }
        const msvc_env_bat = `"C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Auxiliary/Build/${is_i686 ? 'vcvarsamd64_x86.bat' : 'vcvars64.bat'}"`;
        cmd = `${msvc_env_bat} && ${cmd}`;
    }

    let options = {
        'cwd': utils.get_project_root_abs_path(),
        'env': {
            'PKG_CONFIG_ALLOW_CROSS': '1',
            'PKG_CONFIG_PATH': utils.get_vcpkg_pkgconfig_abs_path(target),
        },
        'shell': true,
    };

    if (target == 'i686-pc-windows-msvc') {
        options['env']['SKIA_BINARIES_URL'] =
            'https://github.com/rustytsuki/skia-binaries/releases/download/0.58.0/skia-binaries-rustytsuki-i686-pc-windows-msvc-gl.tar.gz';
    }

    if (target.indexOf('apple') >= 0) {
        options['shell'] = '/bin/zsh';
    }

    return new Promise((resolve) => {
        console.log(`start building target ${target}`);

        let ps = childProcess.spawn(cmd, options);

        ps.stdout.on('data', (data) => {
            console.log(data.toString());
        });

        ps.stderr.on('data', (data) => {
            console.error(data.toString());
        });

        ps.on('close', (code) => {
            console.log(`build exited with code ${code}`);
            resolve(code);
        });
    });
}

async function build_web(target) {
    await build_web_cmd(target);

    const kernel_dir = utils.get_kernel_abs_path(target);
    await fs.remove(path.resolve(kernel_dir, '.gitignore'));
}

async function build_web_cmd(target) {
    const root_dir = utils.get_project_root_abs_path();
    const kernel_dir = utils.get_kernel_abs_path(target);
    const cmd = `wasm-pack build ${path.resolve(root_dir, 'shell')} --out-name rust_office_engine --out-dir ${kernel_dir} --no-typescript --release --target web --features=glow,use_kernel_source`;
    let options = {
        'cwd': root_dir,
        'shell': true,
    };
    if (target.indexOf('apple') >= 0) {
        options['shell'] = '/bin/zsh';
    }

    return new Promise((resolve) => {
        let ps = childProcess.spawn(cmd, options);

        ps.stdout.on('data', (data) => {
            console.log(data.toString());
        });

        ps.stderr.on('data', (data) => {
            console.error(data.toString());
        });

        ps.on('close', (code) => {
            console.log(`build wasm exited with code ${code}`);
            resolve(code);
        });
    });
}
