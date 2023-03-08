use std::fs;
use std::path::PathBuf;
use tokio::process::Command;

pub fn ensure_dir(dir: &PathBuf) -> bool {
    if dir.is_dir() {
        return true;
    }

    if !dir.exists() {
        if let Ok(_) = fs::create_dir_all(&dir) {
            return true;
        }
    }

    return false;
}

pub fn to_abs_path(path: PathBuf) -> std::io::Result<PathBuf> {
    fs::canonicalize(path)
}

pub async fn open_shell_folder(path: &PathBuf) {
    let mut path = String::from(fs::canonicalize(path).unwrap().to_str().unwrap());
    let mut cmd: Command;
    if cfg!(target_os = "windows") {
        path = path.replace(r"\\?\", "");
        cmd = Command::new("explorer");
    } else if cfg!(target_os = "macos") {
        cmd = Command::new("open");
    } else if cfg!(target_os = "linux") {
        cmd = Command::new("nautilus");
    } else {
        return;
    }
    cmd.arg(&path);

    match cmd.output().await {
        Ok(output) => {
            println!("folder: {}, ofd exited with {}", path, output.status.success());
        }
        Err(why) => {
            println!("folder: {}, ofd exited with err: {}", path, why);
        }
    }
}