use super::config::Config;

use std::path::PathBuf;

pub fn determine(config: &Config) -> DirectoryStructure {
    let working_directory_path = new_path(&PathBuf::new(), &config.working_directory);
    let build_pipeline_path = new_path(&working_directory_path, BUILD_PIPELINE);

    DirectoryStructure {
        working_directory_path: working_directory_path,
        scripts_path: new_path(&build_pipeline_path, SCRIPTS),
        config_path: new_path(&build_pipeline_path, CONFIG_JSON),
        build_pipeline_path: build_pipeline_path,
        config_contents: CONFIG,
    }
}

#[derive(Debug, PartialEq)]
pub struct DirectoryStructure {
    pub working_directory_path: PathBuf,
    pub build_pipeline_path: PathBuf,
    pub scripts_path: PathBuf,
    pub config_path: PathBuf,
    pub config_contents: &'static [u8],
}

fn new_path(path: &PathBuf, suffix: &str) -> PathBuf {
    let mut new_path = path.clone();
    new_path.push(suffix);
    new_path
}

const BUILD_PIPELINE: &'static str = "build_pipeline";
const SCRIPTS: &'static str = "scripts";
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

#[cfg(test)]
mod test {

    mod determine {
        use super::super::*;

        #[test]
        fn given_a_config_returns_a_directory_strcuture() {
            let config = Config {
                working_directory: String::from("cool/path/yo"),
            };

            let directory_structure = DirectoryStructure {
                working_directory_path: PathBuf::from("cool/path/yo"),
                build_pipeline_path: PathBuf::from("cool/path/yo/build_pipeline"),
                scripts_path: PathBuf::from("cool/path/yo/build_pipeline/scripts"),
                config_path: PathBuf::from("cool/path/yo/build_pipeline/config.json"),
                config_contents: CONFIG,
            };

            assert_eq!(directory_structure, determine(&config));
        }

        #[test]
        fn given_a_different_config_returns_a_directory_strcuture() {
            let config = Config {
                working_directory: String::from("different"),
            };

            let directory_structure = DirectoryStructure {
                working_directory_path: PathBuf::from("different"),
                build_pipeline_path: PathBuf::from("different/build_pipeline"),
                scripts_path: PathBuf::from("different/build_pipeline/scripts"),
                config_path: PathBuf::from("different/build_pipeline/config.json"),
                config_contents: CONFIG,
            };

            assert_eq!(directory_structure, determine(&config));
        }
    }
}
