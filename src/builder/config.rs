use git2::Repository;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, remove_dir_all};
use url::Url;

use crate::{
    assembler::{format_error, lower::Lower, parser::ProgramParser},
    builder::{FILE_EXTENSION, INPUT_TOML, MAIN_FILE_NAME, OUTPUT_BUILD_DIR, SOURCE_FOLDER},
};

/// The package info: Name, Version, Authors
#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    include: Vec<String>,
}

/// This is the main file, the actual TOML
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

impl Config {
    /// Create new config object from TOML script
    pub fn new(text: &str) -> Self {
        let config: Self = toml::from_str(text).unwrap();
        config
    }

    /// Create new config object from TOML file
    pub fn from_file(path: &str) -> Result<Self, String> {
        match std::fs::read_to_string(path) {
            Ok(contents) => Ok(Self::new(&contents)),
            Err(_) => Err(format!("Could not open file `{}`", path)),
        }
    }

    /// Assemble files in this package's `src` directory
    pub fn assemble(&self, package_path: &str) -> Result<String, String> {
        let mut imports = self.package.include.clone();
        imports.push(String::from(MAIN_FILE_NAME));

        let mut result = String::from("");

        for import in imports {
            let path = format!(
                "{}/{}/{}.{}",
                package_path, SOURCE_FOLDER, import, FILE_EXTENSION
            );
            match read_to_string(&path) {
                Ok(script) => match ProgramParser::new().parse(&script) {
                    Ok(parsed) => result += &parsed.lower(),
                    Err(e) => return Err(format_error(&script, e)),
                },
                Err(_) => return Err(format!("Could not open file `{}`", &path)),
            }
        }

        Ok(result)
    }

    pub fn build(&self, package_path: Option<&str>) -> Result<String, String> {
        // This is the final product to be built by XASM
        let mut result_xasm_script = String::from("");

        // Get the prefix of where to download the dependencies
        let prefix = match package_path {
            Some(path) => format!("{}/", path),
            None => String::from(""),
        };

        let primary_build_dir = &format!("{}{}", prefix, OUTPUT_BUILD_DIR);
        match remove_dir_all(primary_build_dir) {
            _ => {}
        };
        match create_dir_all(primary_build_dir) {
            Ok(_) => {}
            Err(_) => {
                return Err(format!(
                    "Could not create directory `{}`",
                    primary_build_dir
                ))
            }
        }

        // Iterate over dependencies
        // Download and build them recursively
        for name in self.dependencies.keys() {
            // Get the url to the package
            let url = &self.dependencies[name];
            // If the url is valid, continue
            if let Ok(_) = Url::parse(url) {
                // The path to download the build to
                let path = &format!("{}/{}", primary_build_dir, name);
                match Repository::clone(url, path) {
                    Ok(_) => {
                        // The downloaded package's `package.toml`
                        let package = Self::from_file(&format!("{}/{}.toml", path, INPUT_TOML))?;
                        // The downloaded package's dependencies
                        let package_dependencies =
                            package.dependencies.values().collect::<Vec<&String>>();
                        // Verify we have none in common!!!
                        for dep in self.dependencies.values() {
                            // If we have a dependency in common, return `Reused dependency` error
                            if package_dependencies.contains(&dep) {
                                return Err(format!("Reused dependency `{}`", dep));
                            }
                        }
                        // Add the built dependency to the script
                        result_xasm_script += &package.build(Some(&path))?;
                    }
                    Err(_) => return Err(format!("Could not clone git repository from `{}`", url)),
                };
            } else {
                // If the url is invalid, return an error
                return Err(format!("`{}` is not a valid url", url));
            }
        }

        let assemble_path = match package_path {
            Some(path) => path,
            None => ".",
        };

        match self.assemble(assemble_path) {
            Ok(script) => result_xasm_script += &script,
            Err(e) => return Err(format!("Error while compiling package `{}` -->\n{}", self.package.name, e))
        }

        Ok(result_xasm_script)
    }
}
