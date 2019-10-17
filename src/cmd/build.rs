use crate::{
    builder::{Config, INPUT_TOML, OUTPUT_BUILD_DIR},
    log::build::{info, warning},
};
use std::env::consts::EXE_SUFFIX;
use std::fs::write;
use xasm::compile::Compile;
use xassembler::{Golang, Rust};

pub fn build(execute: bool) -> Result<(), String> {
    info("Reading", "toml");

    let config = Config::from_file(&format!("{}.toml", INPUT_TOML))?;

    info("Starting", "build");
    let build_output = config.build(None, OUTPUT_BUILD_DIR)?;

    let output_script = build_output.0;
    let output_path = format!("{}/{}.x", OUTPUT_BUILD_DIR, "main");
    let foreign_package_paths = (build_output.1)
        .iter()
        .map(|s| format!("{}/{}", OUTPUT_BUILD_DIR, s))
        .collect::<Vec<String>>();

    if let Err(_) = write(&output_path, &output_script) {
        return Err(format!("Could not write to file \"{}\"", output_path));
    }

    if execute {
        info("Running", "intermediate code");
        run(output_script, foreign_package_paths)?;
    } else {
        info("Compiling", "intermediate code");
        compile(output_script, foreign_package_paths)?;
    }

    Ok(())
}

fn run(output_script: String, foreign_package_paths: Vec<String>) -> Result<(), String> {
    if let Err(e) = run_xasm::<Golang>(&output_script, &foreign_package_paths) {
        if let Err(_) = run_xasm::<Rust>(&output_script, &foreign_package_paths) {
            return Err(e);
        } else {
            warning("Go build failed, but Rust build succeeded")
        }
    }

    Ok(())
}

fn compile(output_script: String, foreign_package_paths: Vec<String>) -> Result<(), String> {
    if let Err(e) = compile_xasm::<Golang>(&output_script, &foreign_package_paths) {
        if let Err(_) = compile_xasm::<Rust>(&output_script, &foreign_package_paths) {
            return Err(e);
        } else {
            warning("Go build failed, but Rust build succeeded")
        }
    }

    Ok(())
}

fn run_xasm<T: Compile>(
    output_script: &str,
    foreign_package_paths: &Vec<String>,
) -> Result<(), String> {
    let compiled = T::assemble(&output_script)?;

    if let Err(e) = T::run_subcommand(
        &compiled,
        foreign_package_paths.iter().map(|s| &s[..]).collect(),
    ) {
        return Err(e);
    }

    Ok(())
}

fn compile_xasm<T: Compile>(
    output_script: &str,
    foreign_package_paths: &Vec<String>,
) -> Result<(), String> {
    let compiled = T::assemble(&output_script)?;

    if let Err(e) = T::compile_subcommand(
        &compiled,
        foreign_package_paths.iter().map(|s| &s[..]).collect(),
        &format!("{}/{}{}", OUTPUT_BUILD_DIR, "bin", EXE_SUFFIX),
    ) {
        return Err(e);
    }

    Ok(())
}
