use crate::{
    builder::{Config, INPUT_TOML, OUTPUT_BUILD_DIR}
};
use std::fs::write;
use xasm::compile::Compile;


pub fn build<T: Compile>(run: bool) -> Result<(), String> {
    let config = Config::from_file(&format!("{}.toml", INPUT_TOML))?;

    let build_output = config.build(None, OUTPUT_BUILD_DIR)?;
    let output_script = build_output.0;
    let output_path = format!("{}/{}.x", OUTPUT_BUILD_DIR, "main");
    let foreign_package_paths = (build_output.1).iter().map(|s| format!("{}/{}", OUTPUT_BUILD_DIR, s)).collect::<Vec<String>>();

    if let Err(_) = write(&output_path, &output_script) {
        return Err(format!("Could not write to file `{}`", output_path));
    }
    
    let compiled = T::assemble(&output_script)?;
    
    if run {
        if let Err(e) = T::run_subcommand(&compiled, foreign_package_paths.iter().map(|s| &s[..]).collect()) {
            return Err(format!("Internal compiler error:\n{}", e));
        }
    } else {
        if let Err(e) = T::compile_subcommand(&compiled, foreign_package_paths.iter().map(|s| &s[..]).collect(), &format!("{}/{}", OUTPUT_BUILD_DIR, "main")) {
            return Err(format!("Internal compiler error:\n{}", e));
        }
    }

    Ok(())
}