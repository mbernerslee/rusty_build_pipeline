use super::directory_structure::DirectoryStructure;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// https://stackoverflow.com/questions/49286608/what-is-the-proper-way-to-pass-a-module-as-an-argument
pub fn create(d: DirectoryStructure) -> Result<(), String> {
    check_working_directory_exists(&d.working_directory_path)?;
    check_build_pipeline_directory_does_not_exist(&d.build_pipeline_path)?;
    create_build_pipeline_directory(&d.build_pipeline_path)?;
    create_scripts_directory(&d.scripts_path)?;
    create_config_file(&d.config_path, &d.config_contents)
}

fn check_working_directory_exists(working_directory_path: &PathBuf) -> Result<(), String> {
    if working_directory_path.exists() {
        Ok(())
    } else {
        Err(String::from("I failed because the directory you supplied does not exist. I can only initialise in a directory that already exists."))
    }
}

fn check_build_pipeline_directory_does_not_exist(
    build_pipeline_path: &PathBuf,
) -> Result<(), String> {
    if build_pipeline_path.exists() {
        Err(create_build_pipeline_directory_error())
    } else {
        Ok(())
    }
}

fn create_build_pipeline_directory(build_pipeline_path: &PathBuf) -> Result<(), String> {
    match fs::create_dir(build_pipeline_path) {
        Ok(_) => Ok(()),
        _ => Err(create_build_pipeline_directory_error()),
    }
}

fn create_scripts_directory(scripts_path: &PathBuf) -> Result<(), String> {
    match fs::create_dir(scripts_path) {
        Ok(_) => Ok(()),
        _ => Err(String::from(
            "I Failed because I couldn't create the build_pipeline/scripts directory",
        )),
    }
}

fn create_build_pipeline_directory_error() -> String {
    String::from("I failed because the build_pipeline directory I wanted to create already exists. Maybe you've already initialised before?")
}

fn create_config_file(config_file_path: &PathBuf, contents: &'static [u8]) -> Result<(), String> {
    match create_empty_config_file(config_file_path) {
        Ok(config_file) => write_config_file(config_file, contents),
        Err(error) => Err(error),
    }
}

fn write_config_file(mut config_file: File, contents: &'static [u8]) -> Result<(), String> {
    match config_file.write_all(contents) {
        Ok(()) => Ok(()),
        _ => Err(String::from("I failed because I could not write to the build_pipeline/config.json file, which I have created but have left empty.")),
    }
}

fn create_empty_config_file(config_file_path: &PathBuf) -> Result<File, String> {
    match File::create(config_file_path) {
        Ok(config_file) => Ok(config_file),
        _ => Err(String::from(
            "I failed because I could not create the build_pipeline/config.json file",
        )),
    }
}
