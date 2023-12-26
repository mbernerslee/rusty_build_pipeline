use config::Config;

mod config;
mod directory_structure;
mod directory_structure_creation;
mod exit;

pub fn main(args: Vec<String>) {
    create_dir_structure(config::build(args));
}

fn create_dir_structure(maybe_config: Result<Config, String>) {
    match maybe_config {
        Ok(config) => create_directory_structure(&config),
        Err(error) => exit::with_error(error),
    }
}

fn create_directory_structure(config: &Config) {
    let directory_structure = directory_structure::determine(config);
    if let Err(error) = directory_structure_creation::create(directory_structure) {
        exit::with_error(error)
    }
}
