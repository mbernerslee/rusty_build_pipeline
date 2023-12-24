use std::process;

pub fn with_error(error: String) {
    eprintln!("{error}");
    process::exit(1);
}
