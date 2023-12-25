use super::config::Config;

use std::fs;
use std::fs::File;
use std::path::PathBuf;

// https://stackoverflow.com/questions/49286608/what-is-the-proper-way-to-pass-a-module-as-an-argument
pub fn create(config: Config) -> Result<(), String> {
    let working_directory_path = working_directory_path(&config.working_directory);
    let build_pipeline_path = build_pipeline_path(&working_directory_path);

    create_working_directory(&working_directory_path)?;
    create_build_pipeline_directory(&build_pipeline_path)
}

fn create_working_directory(working_directory_path: &PathBuf) -> Result<(), String> {
    if working_directory_path.exists() {
        Err(String::from("I failed because the build_pipeline directory I wanted to create already exists. Maybe you've already initialised before?"))
    } else {
        match fs::create_dir_all(working_directory_path) {
            Ok(_) => Ok(()),
            _ => Err(String::from("I failed because the directory I wanted to create already exists. Maybe you've already initialised before?")),
        }
    }
}

fn create_build_pipeline_directory(build_pipeline_path: &PathBuf) -> Result<(), String> {
    if build_pipeline_path.exists() {
        Err(String::from("I failed because the build_pipeline directory I wanted to create already exists. Maybe you've already initialised before?"))
    } else {
        match File::create(build_pipeline_path) {
            Ok(_) => Ok(()),
            _ => Err(String::from("I failed because the build_pipeline directory I wanted to create already exists. Maybe you've already initialised before?")),
        }
    }
}

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

//fn config_file_path(build_pipeline_path: &PathBuf) -> PathBuf {
//    let mut path = build_pipeline_path.clone();
//    path.push(CONFIG_JSON);
//    path
//}

const BUILD_PIPELINE: &'static str = "build_pipeline";
//const CONFIG_JSON: &'static str = "config.json";
