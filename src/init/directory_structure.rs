use super::config::Config;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// https://stackoverflow.com/questions/49286608/what-is-the-proper-way-to-pass-a-module-as-an-argument
pub fn create(config: Config) -> Result<(), String> {
    let working_directory_path = working_directory_path(&config.working_directory);
    let build_pipeline_path = build_pipeline_path(&working_directory_path);
    let config_file_path = config_file_path(&build_pipeline_path);

    check_working_directory_exists(&working_directory_path)?;
    create_build_pipeline_directory(&build_pipeline_path)?;
    create_config_file(&config_file_path)
}

fn check_working_directory_exists(working_directory_path: &PathBuf) -> Result<(), String> {
    if working_directory_path.exists() {
        Ok(())
    } else {
        Err(String::from("I failed because the directory you supplied does not exist. I can only initialise in a directory that already exists."))
    }
}

fn create_build_pipeline_directory(build_pipeline_path: &PathBuf) -> Result<(), String> {
    if build_pipeline_path.exists() {
        Err(create_build_pipeline_directory_error())
    } else {
        match fs::create_dir(build_pipeline_path) {
            Ok(_) => Ok(()),
            _ => Err(create_build_pipeline_directory_error()),
        }
    }
}

fn create_build_pipeline_directory_error() -> String {
    String::from("I failed because the build_pipeline directory I wanted to create already exists. Maybe you've already initialised before?")
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

fn config_file_path(build_pipeline_path: &PathBuf) -> PathBuf {
    let mut path = build_pipeline_path.clone();
    path.push(CONFIG_JSON);
    path
}

fn create_config_file(config_file_path: &PathBuf) -> Result<(), String> {
    match create_empty_config_file(config_file_path) {
        Ok(config_file) => write_config_file(config_file),
        Err(error) => Err(error),
    }
}

fn write_config_file(mut config_file: File) -> Result<(), String> {
    match config_file.write_all(CONFIG) {
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

const BUILD_PIPELINE: &'static str = "build_pipeline";
const CONFIG_JSON: &'static str = "config.json";
const CONFIG: &'static [u8] = r#"[
  {
    "buildStepName": "check tires",
    "commandType": "shellCommand",
    "command": "echo tires OK!",
    "dependsOn": []
  },
  {
    "buildStepName": "check enough fuel",
    "commandType": "shellCommand",
    "command": "echo fuel OK!",
    "dependsOn": [],
    "envVars": [
      {
        "name": "SOME_ENV_VAR",
        "value": "A_COOL_ENV_VAR"
      }
    ]
  },
  {
    "buildStepName": "drive car",
    "commandType": "shellCommand",
    "command": "echo I am driving OK!",
    "dependsOn": [
      "check tires", "check enough fuel"
    ]
  }
]

"#
.as_bytes();
