use std::{env, fs, path::Path, process::Command};

use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let cwd = env::current_dir()?;

    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        return Ok(());
    }

    let input = args[1].to_owned();
    let out = cwd.join(input);
    println!("-----> out: {}", out.display());

    let mut dirs = vec![];
    traverse_directory(Path::new(out.as_path()), &mut dirs)?;
    if dirs.len() == 0{
        println!("There are no target file should be clean in {}", out.display());
        return Ok(());
    }

    dirs.iter().for_each(|x| {
        let clean_output = Command::new("cargo")
            .arg("clean")
            .arg("-vv")
            .current_dir(x)
            .output()
            .unwrap();
        println!("{}", String::from_utf8_lossy(&clean_output.stdout));
        println!("{}", String::from_utf8_lossy(&clean_output.stderr));
    });

    Ok(())
}

fn traverse_directory(path: &Path, dirs: &mut Vec<String>) -> Result<(), anyhow::Error> {
    if check_cargo(&path)? {
        println!("{}", path.to_string_lossy());
        dirs.push(path.to_string_lossy().to_string());
    }

    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry_path = entry?.path();
        println!("-----> {}", entry_path.display());
        if entry_path.is_dir() {
            if !ignore(entry_path.to_string_lossy().to_string()) {
                continue;
            }
            if check_cargo(&entry_path)? {
                println!("{}", entry_path.to_string_lossy());
                dirs.push(entry_path.to_string_lossy().to_string());
            }
            traverse_directory(&entry_path, dirs)?;
        }
    }
    Ok(())
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
        ".Trash",
        ".flutter-tools",
    ];
    for s in exclude_dirs {
        if x.contains(s) {
            return false;
        }
    }
    true
}

fn check_cargo(path: &Path) -> Result<bool, anyhow::Error> {
    let mut target_flag = false;
    let mut cargo_flag = false;

    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry_path = entry?.path();
        if entry_path.is_dir() {
            if entry_path.file_name().unwrap() == "target" {
                target_flag = true;
            }
        } else if entry_path.file_name().unwrap() == "Cargo.toml" {
            cargo_flag = true;
        }
    }

    Ok(cargo_flag && target_flag)
}
