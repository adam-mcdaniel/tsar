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

    if create_dir_all(name).is_err() {
        return Err(format!("Could not create folder \"{}\"", name));
    }

    if write(
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
    ).is_err() {
        return Err(format!("Could not write to file \"{}\"", toml_path));
    }

    if write(format!("{}/.gitignore", name), GITIGNORE).is_err() {
        return Err(String::from("Could not write gitignore"));
    }

    if create_dir_all(format!("{}/{}", name, SOURCE_FOLDER)).is_err() {
        return Err(format!(
            "Could not make \"{}\" directory when creating package \"{}\"",
            SOURCE_FOLDER, name
        ));
    }

    if create_dir_all(format!("{}/{}", name, FOREIGN_FOLDER)).is_err() {
        return Err(format!(
            "Could not make \"{}\" directory when creating package \"{}\"",
            FOREIGN_FOLDER, name
        ));
    }

    let main_file = format!(
        "{}/{}/{}.{}",
        name, SOURCE_FOLDER, MAIN_FILE_NAME, FILE_EXTENSION
    );

    if write(&main_file, "println(\"Hello, world!\")").is_err() {
        return Err(main_file);
    }

    Ok(())
}
