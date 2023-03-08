import path from 'path';
import url from 'url';

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
const project_root_abs_path = path.resolve(__dirname, '../../');

export function get_project_root_abs_path() {
    return project_root_abs_path;
}

export function get_vcpkg_abs_path(target) {
    if (target == 'x86_64-pc-windows-msvc') {
        return path.resolve(project_root_abs_path, '../rust-office-vcpkg/x86_64-pc-windows-msvc/installed/x64-windows');
    } else if (target == 'i686-pc-windows-msvc') {
        return path.resolve(project_root_abs_path, '../rust-office-vcpkg/i686-pc-windows-msvc/installed/x86-windows');
    } else if (target == 'x86_64-apple-darwin') {
        return path.resolve(project_root_abs_path, '../rust-office-vcpkg/x86_64-apple-darwin/installed/x64-osx-dynamic');
    } else if (target == 'aarch64-apple-darwin') {
        return path.resolve(project_root_abs_path, '../rust-office-vcpkg/aarch64-apple-darwin/installed/arm64-osx-dynamic');
    }
}

export function get_vcpkg_pkgconfig_abs_path(target) {
    return path.resolve(get_vcpkg_abs_path(target), 'lib/pkgconfig');
}

export function get_vcpkg_bin_abs_path(target) {
    if (target.indexOf('apple') >= 0) {
        return path.resolve(get_vcpkg_abs_path(target), 'lib');
    } else {
        return path.resolve(get_vcpkg_abs_path(target), 'bin');
    }
}

export function get_release_abs_path(target) {
    return path.resolve(project_root_abs_path, `target/${target}/release`);
}

export function get_release_deps_abs_path(target) {
    return path.resolve(get_release_abs_path(target), 'deps');
}

export function get_kernel_abs_path(target) {
    return path.resolve(project_root_abs_path, `kernel/${target}`);
}

export function get_windows_vcpkg_dlls() {
    return [
        'fontconfig-1.dll',
        'libexpat.dll',
        'freetype.dll',
        'bz2.dll',
        'zlib1.dll',
        'libpng16.dll',
        'brotlidec.dll',
        'brotlicommon.dll'
    ];
}

export function get_windows_kernel_libs() {
    return [
        'rust_office_engine.dll',
        'rust_office_engine.dll.lib'
    ];
}

export function get_mac_vcpkg_dylibs() {
    return [
        'libfontconfig.1.dylib',
        'libexpat.1.dylib',
        'libfreetype.6.dylib',
        'libbz2.1.0.dylib',
        'libz.1.dylib',
        'libpng16.16.dylib',
        'libbrotlidec.1.dylib',
        'libbrotlicommon.1.dylib'
    ];
}

export function get_mac_kernel_libs() {
    return ['librust_office_engine.dylib'];
}

export function get_web_kernel_libs() {
    return [
        'package.json',
        'rust_office_engine_bg.wasm',
        'rust_office_engine.js'
    ];
}