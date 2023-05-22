use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

mod edit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, name = "cargo", bin_name = "cargo")]
struct Cli {
    #[command(subcommand)]
    desktop: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a .desktop file
    Desktop {
        /// Automatically answer yes to confirmation prompt
        #[arg(short = 'y', long, default_value_t = false)]
        assume_yes: bool,

        /// Open the desktop file in an editor
        #[arg(short = 'e', long, default_value_t = false)]
        edit: bool,
    },
}

#[derive(Deserialize, Debug)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    #[serde(rename = "default-run")]
    default_run: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let Commands::Desktop { assume_yes, edit } = cli.desktop;

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

    let project_name = match cargo_toml.package.default_run {
        Some(name) => name,
        None => cargo_toml.package.name,
    };

    let mut exec_path = home_dir.clone();
    exec_path.push(".cargo");
    exec_path.push("bin");
    exec_path.push(&project_name);

    let desktop_file = format!(
        "[Desktop Entry]
Encoding=UTF-8
Type=Application

# Specific name of the application, for example \"Firefox\"
Name={}

# Program to execute, possibly with arguments
Exec={}

# Generic name of the application, for example \"Web Browser\"
# GenericName=

# Tooltip for the entry, for example \"View sites on the Internet\"
comment=

# Name or path of of the icon that will be used to display this entry
Icon=

# Categories in which the entry should be shown in a menu
Categories=

# Whether to hide the program in menus
NoDisplay=false

# Whether the program runs in a terminal window
Terminal=false
",
        &project_name,
        exec_path.display(),
    );

    let mut desktop_file_path = home_dir.clone();
    desktop_file_path.push(".local");
    desktop_file_path.push("share");
    desktop_file_path.push("applications");
    desktop_file_path.push(format!("{}.desktop", &project_name));

    if desktop_file_path.exists() && !assume_yes {
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

    if edit {
        edit::edit(&desktop_file_path);
    }
}
