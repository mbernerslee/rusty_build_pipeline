pub fn main(args: Vec<String>) {
    build_config(&args)
}

fn build_config(args: &Vec<String>) {}

struct Config {
    pub working_directory: String,
}

const DEFAULT_WORKING_DIRECTORY: &'static str = ".";
