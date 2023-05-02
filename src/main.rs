use serde::Deserialize;

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
    let cargo_toml = std::fs::read_to_string("Cargo.toml").unwrap();
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml).unwrap();

    let mut exec_path = home::home_dir().unwrap();
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
    println!("{}", desktop_file);
}
