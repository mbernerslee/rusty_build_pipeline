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

pub trait DirectoryStructurer {
    fn working_directory_path_exists(&self) -> bool;
    fn build_pipeline_directory_exists(&self) -> bool;

    fn create_build_pipeline_directory(&self) -> std::io::Result<()>;
    fn create_scripts_directory(&self) -> std::io::Result<()>;
    fn create_empty_config_file(&self) -> std::io::Result<std::fs::File>;
    fn write_to_config_file(&self, config_file: std::fs::File) -> std::io::Result<()>;
}

fn new_path(path: &PathBuf, suffix: &str) -> PathBuf {
    let mut new_path = path.clone();
    new_path.push(suffix);
    new_path
}

pub const BUILD_PIPELINE: &'static str = "build_pipeline";
pub const SCRIPTS: &'static str = "scripts";
pub const CONFIG_JSON: &'static str = "config.json";

pub const CONFIG: &'static [u8] = r#"
[
  {
    "build_step_name": "check tires",
    "command_type": "ShellCommand",
    "command": "echo tires OK!",
    "depends_on": []
  },
  {
    "build_step_name": "check enough fuel",
    "command_type": "ShellCommand",
    "command": "echo fuel OK!",
    "depends_on": [],
    "env_vars": [
      {
        "name": "SOME_ENV_VAR",
        "value": "A_COOL_ENV_VAR"
      }
    ]
  },
  {
    "build_step_name": "drive car",
    "command_type": "ShellCommand",
    "command": "echo I am driving OK!",
    "depends_on": [
      "check tires", "check enough fuel"
    ]
  }
]"#
.as_bytes();

#[cfg(test)]
mod test {

    mod determine {
        use super::super::*;
        use crate::build_step::*;

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

        #[test]
        fn the_config_deserliases() {
            let expected = vec![
                BuildStep {
                    build_step_name: String::from("check tires"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo tires OK!"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("check enough fuel"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo fuel OK!"),
                    depends_on: vec![],
                    env_vars: Some(vec![EnvVar {
                        name: String::from("SOME_ENV_VAR"),
                        value: String::from("A_COOL_ENV_VAR"),
                    }]),
                },
                BuildStep {
                    build_step_name: String::from("drive car"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo I am driving OK!"),
                    depends_on: vec![
                        String::from("check tires"),
                        String::from("check enough fuel"),
                    ],
                    env_vars: None,
                },
            ];

            let deserliased_config: Vec<BuildStep> =
                serde_json::from_str(std::str::from_utf8(CONFIG).unwrap()).unwrap();
            assert_eq!(deserliased_config, expected)
        }
    }
}
