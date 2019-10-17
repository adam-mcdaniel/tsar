use clap::{clap_app, crate_authors, crate_version, AppSettings::ArgRequiredElseHelp};
use tsar::{
    cmd::{build, create},
    log::build::{error, finished, version},
};

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
            Ok(_) => finished(format!("creating package \"{}\"", name)),
            Err(e) => error(format!("Failed to create package \"{}\": {}", name, e)),
        };
    } else {
        version();

        if let Some(_) = matches.subcommand_matches("build") {
            match build(false) {
                Ok(_) => finished("building package"),
                Err(e) => error(e),
            }
        }

        if let Some(_) = matches.subcommand_matches("run") {
            match build(true) {
                Ok(_) => finished("running package"),
                Err(e) => error(e),
            }
        }
    }
}
