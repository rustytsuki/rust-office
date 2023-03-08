use std::{path::{PathBuf, Path}, env, fs};

fn get_binary_output_path() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir);
    let output = path.parent().unwrap().parent().unwrap().parent().unwrap();
    return PathBuf::from(output);
}

fn get_vcpkg_path() -> Option<PathBuf> {
    let engine_root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&engine_root);
    let root = path.parent().unwrap();
    let mut ret = PathBuf::from(root.parent().unwrap());

    let target = env::var("TARGET").unwrap();
    if target.find("x86_64-pc-windows-msvc").is_some() {
        ret.push("rust-office-vcpkg/x86_64-pc-windows-msvc/installed/x64-windows");
        return Some(ret);
    } else if target.find("i686-pc-windows-msvc").is_some() {
        ret.push("rust-office-vcpkg/i686-pc-windows-msvc/installed/x86-windows");
        return Some(ret);
    } else if target.find("x86_64-apple-darwin").is_some() {
        ret.push("rust-office-vcpkg/x86_64-apple-darwin/installed/x64-osx-dynamic");
        return Some(ret);
    } else if target.find("aarch64-apple-darwin").is_some() {
        ret.push("rust-office-vcpkg/aarch64-apple-darwin/installed/arm64-osx-dynamic");
        return Some(ret);
    }
    None
}

fn get_vcpkg_lib_path() -> PathBuf {
    let mut vcpkg = get_vcpkg_path().unwrap();
    vcpkg.push("lib");
    vcpkg
}

fn get_kernel_path() -> PathBuf {
    let path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&path);
    let mut kernel = PathBuf::from(path.parent().unwrap());
    kernel.push("kernel");

    let target = env::var("TARGET").unwrap();
    kernel.push(target);

    kernel
}

fn copy_all_kernel_dylib() {
    let target = env::var("TARGET").unwrap();
    let dylib_ext = if target.find("windows").is_some() {
        "dll"
    } else if target.find("apple").is_some() {
        "dylib"
    } else {
        "so"
    };

    let output = get_binary_output_path();
    let kernel = get_kernel_path();
    for entry in fs::read_dir(kernel).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            if let Some(filename) = path.file_name() {
                if let Some(ext) = path.extension() {
                    if ext == dylib_ext {
                        let dest_path = output.clone().join(filename);
                        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
                        println!("cargo:rerun-if-changed={}", dest_path.to_str().unwrap());
                        fs::copy(&path, &dest_path).unwrap();
                    }
                }
            }
        }
    }
}

fn copy_runtime() {
    if !cfg!(feature = "use_kernel_source") {
        copy_all_kernel_dylib();
    }
}

fn set_link_search() {
    if cfg!(feature = "use_kernel_source") {
        // link to engine, rust_office_engine.dll.lib is in target\debug\deps
        // automatically added by cargo
        // only need to add vcpkg lib for static link
        let vcpkg = get_vcpkg_lib_path();
        println!("cargo:rustc-link-search=native={}", vcpkg.to_str().unwrap());
    } else {
        let kernel = get_kernel_path();
        println!("cargo:rustc-link-search=native={}", kernel.to_str().unwrap());
    }
}

fn set_link_lib() {
    let target = env::var("TARGET").unwrap();
    if target.find("windows").is_some() {
        println!("cargo:rustc-link-lib=dylib=rust_office_engine.dll");
    } else {
        println!("cargo:rustc-link-lib=dylib=rust_office_engine");
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();

    // you have to add cargo -vv to show this warnning message!
    println!("cargo:warning=CARGO_MANIFEST_DIR: {}", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:warning=OUT_DIR: {}", env::var("OUT_DIR").unwrap());
    println!("cargo:warning=TARGET: {}", target);
    println!("cargo:warning=PROFILE: {}", env::var("PROFILE").unwrap());
    println!("cargo:warning=HOST: {}", env::var("HOST").unwrap());

    if target.find("wasm32").is_none() {
        copy_runtime();
        set_link_search();
        set_link_lib();
    }
}