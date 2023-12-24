use super::config::Config;
use super::exit;

use std::fs;
use std::fs::File;
//use std::io::Write;
use std::path::{Path, PathBuf};

// https://stackoverflow.com/questions/49286608/what-is-the-proper-way-to-pass-a-module-as-an-argument
pub fn create(config: Config) {
    let working_directory_path = working_directory_path(&config.working_directory);
    let build_pipeline_path = build_pipeline_path(&working_directory_path);

    create_working_directory(&working_directory_path);
    create_build_pipeline_directory(&build_pipeline_path)
}

fn create_working_directory(working_directory_path: &PathBuf) {
    if working_directory_path.exists() {
        exit("I failed because the  irectory I wanted to create already exists. Maybe you've already initialised before?");
    } else {
        fs::create_dir_all(working_directory_path);
    }
}

fn create_build_pipeline_directory(build_pipeline_path: &PathBuf) {
    if build_pipeline_path.exists() {
        exit("I failed because the build_pipeline directory I wanted to create already exists. Maybe you've already initialised before?");
    } else {
        File::create(build_pipeline_path);
    }
}

fn exit(error: &str) {
    exit::with_error(String::from(error))
}

//fn directory(working_directory: &String) -> &Path {
//    Path::new(working_directory)
//}

fn working_directory_path(working_dir_path: &String) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(working_dir_path);
    path
}

fn build_pipeline_path(working_directory_path: &PathBuf) -> PathBuf {
    let mut path = working_directory_path.clone();
    path.push(BUILD_PIPELINE);
    path
}

fn config_file_path(build_pipeline_path: &PathBuf) -> PathBuf {
    let mut path = build_pipeline_path.clone();
    path.push(CONFIG_JSON);
    path
}

const BUILD_PIPELINE: &'static str = "build_pipeline";
const CONFIG_JSON: &'static str = "config.json";
