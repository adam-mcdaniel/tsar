pub mod config;

pub const OUTPUT_BUILD_DIR: &str = "target";
pub const INPUT_TOML: &str = "package";
pub const SOURCE_FOLDER: &str = "src";
pub const FILE_EXTENSION: &str = "ts";
pub const MAIN_FILE_NAME: &str = "bin";

pub use config::Config;