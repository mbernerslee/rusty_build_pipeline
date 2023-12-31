extern crate serde_json;

use crate::build_step::*;

pub fn determine_build_steps(raw_config: &String) -> Result<Vec<BuildStep>, String> {
    match deserialise(raw_config) {
        Ok(build_steps) => Ok(build_steps),
        Err(parsing_error) => error(parsing_error.to_string()),
    }
}

fn deserialise(raw_config: &String) -> serde_json::Result<Vec<BuildStep>> {
    let build_steps: Vec<BuildStep> = serde_json::from_str(raw_config.as_str())?;
    Ok(build_steps)
}

fn error(serde_error: String) -> Result<Vec<BuildStep>, String> {
    Err(String::from(format!(
        r#"Failed to parse the config.json file.
See more specific parsing error:

{}
"#,
        serde_error
    )))
}

#[cfg(test)]
mod test {
    mod determine_build_steps {
        use super::super::*;

        #[test]
        fn returns_build_steps_when_its_valid_json_build_steps() {
            let expected_result = vec![BuildStep {
                build_step_name: String::from("sayHello"),
                command_type: CommandType::ShellCommand,
                command: String::from("echo 'hello'"),
                depends_on: vec![],
                env_vars: None,
            }];

            let raw_config = String::from(
                r#"
                    [
                      {
                        "build_step_name": "sayHello",
                        "command_type": "ShellCommand",
                        "command": "echo 'hello'",
                        "depends_on": []
                      }
                    ]
                "#,
            );

            assert_eq!(determine_build_steps(&raw_config), Ok(expected_result))
        }

        #[test]
        fn returns_build_steps_when_its_valid_json_with_env_vars() {
            let expected_result = vec![BuildStep {
                build_step_name: String::from("sayHello"),
                command_type: CommandType::ShellCommand,
                command: String::from("echo 'hello'"),
                depends_on: vec![String::from("something")],
                env_vars: Some(vec![
                    EnvVar {
                        name: String::from("env_var_name_1"),
                        value: String::from("env_var_value_1"),
                    },
                    EnvVar {
                        name: String::from("env_var_name_2"),
                        value: String::from("env_var_value_2"),
                    },
                ]),
            }];

            let raw_config = String::from(
                r#"
                    [
                      {
                        "build_step_name": "sayHello",
                        "command_type": "ShellCommand",
                        "command": "echo 'hello'",
                        "depends_on": ["something"],
                        "env_vars": [
                            {"name": "env_var_name_1", "value": "env_var_value_1"},
                            {"name": "env_var_name_2", "value": "env_var_value_2"}
                        ]
                      }
                    ]
                "#,
            );

            assert_eq!(determine_build_steps(&raw_config), Ok(expected_result))
        }

        #[test]
        fn returns_error_when_its_invalid_json() {
            let expected_result = String::from("Failed to parse the config.json file.\nSee more specific parsing error:\n\ninvalid type: map, expected a sequence at line 1 column 0\n");

            assert_eq!(
                determine_build_steps(&String::from("{dffds{")),
                Err(expected_result)
            )
        }

        #[test]
        fn returns_invalid_when_its_valid_json_but_not_build_steps() {
            let expected_result = String::from("Failed to parse the config.json file.\nSee more specific parsing error:\n\nmissing field `depends_on` at line 12 column 23\n");

            let raw_config = String::from(
                r#"
                    [
                      {
                        "build_step_name": "sayHello",
                        "command_type": "ShellCommand",
                        "command": "echo 'hello'",
                        "BAD_THING": ["something"],
                        "env_vars": [
                            {"name": "env_var_name_1", "value": "env_var_value_1"},
                            {"name": "env_var_name_2", "value": "env_var_value_2"}
                        ]
                      }
                    ]
                "#,
            );

            assert_eq!(determine_build_steps(&raw_config), Err(expected_result))
        }
    }
}
