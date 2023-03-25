use std::{fs, path::Path, process::Command};

fn main() {
    let input = "/Users/bytedance/";
    let mut dirs = vec![input.to_string()];
    traverse_directory(Path::new(input), &mut dirs);
    dirs.iter()
        .filter(|x| ignore(x.to_string()))
        .filter(|x| check_cargo(Path::new(x)))
        .for_each(|x| {
            println!("{}", x);
            let clean_output = Command::new("cargo")
                .arg("clean")
                .arg("-vv")
                .output()
                .unwrap();
            println!("{}", String::from_utf8_lossy(&clean_output.stdout));
            println!("{}", String::from_utf8_lossy(&clean_output.stderr));
        });
}

fn traverse_directory(path: &Path, dirs: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    if !ignore(entry_path.to_string_lossy().to_string()) {
                        continue;
                    }
                    print!("\rDirectory: {}", entry_path.display());
                    dirs.push(entry_path.clone().to_string_lossy().to_string());
                    traverse_directory(&entry_path, dirs);
                } else {
                    // println!("File: {}", entry_path.display());
                }
            }
        }
    }
}

fn ignore(x: String) -> bool {
    let exclude_dirs = vec![
        "Library",
        "node_modules",
        "target",
        "dist",
        ".git",
        ".vscode",
        ".idea",
        ".cargo",
        ".bun",
        ".colima",
        ".gradle",
        ".codeium",
        "/pkg/mod/",
        ".rustup",
        ".npm",
        ".yarn",
        ".pub-cache",
        ".mbox",
        ".cache",
        ".oh-my-zsh",
    ];
    for s in exclude_dirs {
        if x.contains(s) {
            return false;
        }
    }
    true
}

fn check_cargo(path: &Path) -> bool {
    let mut target_flag = false;
    let mut cargo_flag = false;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    if entry_path.file_name().unwrap() == "target" {
                        target_flag = true;
                    }
                    print!("\rDirectory: {}", entry_path.display());
                } else {
                    // println!("File: {}", entry_path.display());
                    if entry_path.file_name().unwrap() == "Cargo.toml" {
                        cargo_flag = true;
                    }
                }
            }
        }
    }

    cargo_flag && target_flag
}
