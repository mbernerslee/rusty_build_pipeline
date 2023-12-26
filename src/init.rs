use config::Config;

mod config;
mod directory_structure;
mod exit;

pub fn main(args: Vec<String>) {
    create_dir_structure(config::build(args));
}

fn create_dir_structure(maybe_config: Result<Config, String>) {
    match maybe_config {
        Ok(config) => create_directory_structure(config),
        Err(error) => exit::with_error(error),
    }
}

fn create_directory_structure(config: Config) {
    if let Err(error) = directory_structure::create(config) {
        exit::with_error(error)
    }
}
