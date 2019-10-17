use crate::log::build::{building, downloading, importing};

use fs_extra::dir::{copy, CopyOptions};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string};
use url::Url;

extern crate remove_dir_all;
use remove_dir_all::remove_dir_all;

use crate::{
    assembler::{format_error, lower::Lower, parser::ProgramParser},
    builder::{
        FILE_EXTENSION, FOREIGN_FOLDER, INPUT_TOML, MAIN_FILE_NAME, OUTPUT_BUILD_DIR, SOURCE_FOLDER,
    },
};

/// The package info: Name, Version, Authors
#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    include: Vec<String>,
    foreign: Vec<String>,
}

/// This is the main file, the actual TOML
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

impl Config {
    /// Create new config object from TOML script
    pub fn new(text: &str) -> Result<Self, String> {
        // let config: Self = toml::from_str(text);
        // config
        let config: Self = match toml::from_str(text) {
            Ok(c) => Ok(c),
            Err(e) => {
                if let Some(line_col) = e.line_col() {
                    Err(format!(
                        "Error parsing toml, failed to parse line \"{}\"",
                        text.lines().nth(line_col.0).unwrap()
                    ))
                } else {
                    Err(format!("Error parsing toml"))
                }
            }
        }?;

        Ok(config)
    }

    /// Create new config object from TOML file
    pub fn from_file(path: &str) -> Result<Self, String> {
        match std::fs::read_to_string(path) {
            Ok(contents) => Ok(Self::new(&contents)?),
            Err(_) => Err(format!("Could not open file \"{}\"", path)),
        }
    }

    /// Assemble files in this package's "src" directory
    pub fn assemble(&self, package_path: &str) -> Result<String, String> {
        let mut imports = self.package.include.clone();
        imports.push(String::from(MAIN_FILE_NAME));

        let mut result = String::from("");

        for import in imports {
            importing(import.clone());

            let path = format!(
                "{}/{}/{}.{}",
                package_path, SOURCE_FOLDER, import, FILE_EXTENSION
            );

            match read_to_string(&path) {
                Ok(script) => match ProgramParser::new().parse(&script) {
                    Ok(parsed) => result += &parsed.lower(),
                    Err(e) => return Err(format_error(&script, e))
                },
                Err(_) => {
                    return Err(format!("Could not open file \"{}\"", &path))
                },
            }
        }

        Ok(result)
    }

    pub fn build(
        &self,
        package_path: Option<&str>,
        base_build_dir: &str,
    ) -> Result<(String, Vec<String>), String> {
        let mut copy_opts = CopyOptions::new();
        copy_opts.overwrite = true;

        // Foreign packages required for building
        let mut foreign_package_names = self.package.foreign.clone();

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
                    "Could not create directory \"{}\"",
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
                downloading(name, path);

                match Repository::clone(url, path) {
                    Ok(_) => {
                        // The downloaded package's "package.toml"
                        let package = Self::from_file(&format!("{}/{}.toml", path, INPUT_TOML))?;
                        // The downloaded package's dependencies
                        let package_dependencies =
                            package.dependencies.values().collect::<Vec<&String>>();
                        // Verify we have none in common!!!
                        for dep in self.dependencies.values() {
                            // If we have a dependency in common, return "Reused dependency" error
                            if package_dependencies.contains(&dep) {
                                return Err(format!("Reused dependency \"{}\"", dep));
                            }
                        }

                        // Add the built dependency to the script
                        let build_output = package.build(Some(&path), base_build_dir)?;

                        result_xasm_script += &build_output.0;
                        foreign_package_names.extend(build_output.1);
                    }
                    Err(_) => {
                        return Err(format!("Could not clone git repository from \"{}\"", url))
                    }
                };
            } else {
                // If the url is invalid, return an error
                return Err(format!("\"{}\" is not a valid url", url));
            }
        }

        for foreign_package in &self.package.foreign {
            let from = format!("{}{}/{}", prefix, FOREIGN_FOLDER, foreign_package);
            if let Err(_) = copy(&from, base_build_dir, &copy_opts) {
                return Err(format!(
                    "Could not copy foreign package from \"{}\" to \"{}\"",
                    from, base_build_dir
                ));
            }
        }

        let assemble_path = match package_path {
            Some(path) => path,
            None => ".",
        };

        building(&self.package.name);
        match self.assemble(assemble_path) {
            Ok(script) => result_xasm_script += &script,
            Err(e) => {
                return Err(format!(
                    "Error while compiling package \"{}\" -->\n{}",
                    self.package.name, e
                ))
            }
        }

        Ok((result_xasm_script, foreign_package_names))
    }
}
