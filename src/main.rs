use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Deserialize, Debug)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    _description: Option<String>,
}

fn main() {
    let cargo_toml = match fs::read_to_string("Cargo.toml") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading Cargo.toml: {}", e);
            std::process::exit(-1);
        }
    };
    let cargo_toml: CargoToml = match toml::from_str(&cargo_toml) {
        Ok(toml) => toml,
        Err(e) => {
            eprintln!("Error parsing Cargo.toml: {}", e.message());
            std::process::exit(-1);
        }
    };

    let home_dir = home::home_dir().unwrap();

    let mut exec_path = home_dir.clone();
    exec_path.push(".cargo");
    exec_path.push("bin");
    exec_path.push(&cargo_toml.package.name);

    let desktop_file = format!(
        "[Desktop Entry]
Encoding=UTF-8
Type=Application
NoDisplay=false
Terminal=false
Name={}
Exec={}
comment=
",
        cargo_toml.package.name,
        exec_path.display(),
    );

    let mut desktop_file_path = home_dir.clone();
    desktop_file_path.push(".local");
    desktop_file_path.push("share");
    desktop_file_path.push("applications");
    desktop_file_path.push(format!("{}.desktop", cargo_toml.package.name));

    if desktop_file_path.exists() {
        eprint!(
            "{} already exists. Write anyways? [y/N]: ",
            desktop_file_path.display()
        );
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line != "y\n" {
            eprintln!("aborted");
            std::process::exit(0);
        }
    }

    let mut file = File::create(&desktop_file_path).unwrap();
    file.write_all(desktop_file.as_bytes()).unwrap();

    println!("{}", desktop_file_path.display());
}
