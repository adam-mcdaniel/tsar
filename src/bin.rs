use clap::{clap_app, crate_authors, crate_version, AppSettings::ArgRequiredElseHelp};
use tsar::cmd::{build, create};
use xassembler::{Golang, Rust};

fn main() {
    let matches = clap_app!(tsar =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Compiler for the Tsar programming langauge")
        (@subcommand new =>
            (about: "Create new Tsar package")
            (version: crate_version!())
            (author: crate_authors!())
            (@arg name: +required "The package name")
        )
        (@subcommand build =>
            (about: "Build a Tsar package")
            (version: crate_version!())
            (author: crate_authors!())
        )
        (@subcommand run =>
            (about: "Run a Tsar package")
            (version: crate_version!())
            (author: crate_authors!())
        )
    )
    .setting(ArgRequiredElseHelp)
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let name = matches.value_of("name").unwrap();
        match create(name) {
            Ok(_) => println!("Successfully created package `{}`", name),
            Err(e) => eprintln!("Failed to create package `{}`: {}", name, e),
        };
    }

    if let Some(_) = matches.subcommand_matches("build") {
        match build::<Golang>(false) {
            Ok(()) => println!("Successfully built package"),
            Err(e) => {
                eprintln!("{}", e);
                eprintln!("Attempting to use Rust backend instead");
                if let Err(e) = build::<Rust>(true) {
                    eprintln!("{}", e);
                    eprintln!("Aborting");
                } else {
                    println!("Successfully built package");
                }
            }
        }
    }

    if let Some(_) = matches.subcommand_matches("run") {
        if let Err(e) = build::<Golang>(true) {
            eprintln!("{}", e);
            eprintln!("Attempting to use Rust backend instead");
            if let Err(e) = build::<Rust>(true) {
                eprintln!("{}", e);
                eprintln!("Aborting");
            }
        }
    }
}
