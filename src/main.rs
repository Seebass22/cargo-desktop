use serde::Deserialize;
use std::fs;
use std::fs::File;
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
    let cargo_toml = fs::read_to_string("Cargo.toml").unwrap();
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml).unwrap();

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

    println!("{}", desktop_file);
    println!("{}", desktop_file_path.display());

    let mut file = File::create(desktop_file_path).unwrap();
    file.write_all(desktop_file.as_bytes()).unwrap();
}
