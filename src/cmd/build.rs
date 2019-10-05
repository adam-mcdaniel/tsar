use crate::builder::{Config, INPUT_TOML, OUTPUT_BUILD_DIR};
use std::fs::write;
use xasm::compile::Compile;

pub fn build<T: Compile>(run: bool) -> Result<(), String> {
    let config = Config::from_file(&format!("{}.toml", INPUT_TOML))?;

    let output_script = config.build(None)?;
    let output_path = format!("{}/{}.x", OUTPUT_BUILD_DIR, "main");
    if let Err(_) = write(&output_path, &output_script) {
        return Err(format!("Could not write to file `{}`", output_path));
    }
    
    let compiled = T::compiler_output(&output_script)?;
    
    if run {
        if let Err(e) = T::run_subcommand(&compiled) {
            return Err(format!("Internal compiler error:\n{}", e));
        }
    } else {
        if let Err(e) = T::compile_subcommand(&compiled, &format!("{}/{}", OUTPUT_BUILD_DIR, "main")) {
            return Err(format!("Internal compiler error:\n{}", e));
        }
    }

    Ok(())
}