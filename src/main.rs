use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    description: Option<String>,
}

fn main() {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").unwrap();
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml).unwrap();
    println!("{:?}", cargo_toml);
}
