use std::{env, path::Path, process::Command};

pub fn edit(path: &Path) {
    let editor = env::var("EDITOR").unwrap_or("vi".into());
    let editor = shell_words::split(&editor).expect("failed to parse $EDITOR");

    Command::new(&editor[0])
        .args(&editor[1..])
        .arg(path)
        .status()
        .expect("Something went wrong");
}
