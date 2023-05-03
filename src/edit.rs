use std::{env, path::Path, process::Command};

pub fn edit(path: &Path) {
    let editor = env::var("EDITOR").unwrap_or("vi".into());

    Command::new(editor)
        .arg(path)
        .status()
        .expect("Something went wrong");
}
