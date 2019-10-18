use crate::{
    builder::{FILE_EXTENSION, FOREIGN_FOLDER, INPUT_TOML, MAIN_FILE_NAME, SOURCE_FOLDER},
    cmd::GITIGNORE,
};
use std::fs::{create_dir_all, write};
use std::path::Path;

pub fn create(name: &str) -> Result<(), String> {
    let toml_path = format!("{}/{}.toml", name, INPUT_TOML);
    if Path::new(&toml_path).exists() {
        return Err(String::from("Package already exists"));
    }

    if let Err(_) = create_dir_all(name) {
        return Err(format!("Could not create folder \"{}\"", name));
    }

    if let Err(_) = write(
        &toml_path,
        format!(
            "[package]
name = \"{}\"
version = \"0.0.1\"
authors = []
include = []
foreign = []

[dependencies]
std = \"https://github.com/adam-mcdaniel/tsar-std\"
",
            name
        ),
    ) {
        return Err(format!("Could not write to file \"{}\"", toml_path));
    }

    if let Err(_) = write(".gitignore", GITIGNORE) {
        return Err(String::from("Could not write gitignore"));
    }

    if let Err(_) = create_dir_all(format!("{}/{}", name, SOURCE_FOLDER)) {
        return Err(format!(
            "Could not make \"{}\" directory when creating package \"{}\"",
            SOURCE_FOLDER, name
        ));
    }

    if let Err(_) = create_dir_all(format!("{}/{}", name, FOREIGN_FOLDER)) {
        return Err(format!(
            "Could not make \"{}\" directory when creating package \"{}\"",
            FOREIGN_FOLDER, name
        ));
    }

    let main_file = format!(
        "{}/{}/{}.{}",
        name, SOURCE_FOLDER, MAIN_FILE_NAME, FILE_EXTENSION
    );

    if let Err(_) = write(&main_file, "println(\"Hello, world!\")") {
        return Err(main_file);
    }

    Ok(())
}
