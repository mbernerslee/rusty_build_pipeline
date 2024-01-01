use crate::build_step::*;
use std::collections::{HashMap, HashSet};

pub fn run(build_steps: &Vec<BuildStep>) -> Result<(), String> {
    let mut direct_deps: HashMap<&String, Vec<&String>> = HashMap::new();

    for build_step in build_steps {
        let mut deps: Vec<&String> = Vec::new();

        for dependent in &build_step.depends_on {
            deps.push(&dependent);
        }

        match direct_deps.insert(&build_step.build_step_name, deps) {
            None => (),
            _ => return unique_names_error(),
        }
    }

    check_circular_deps(direct_deps)
}

fn check_circular_deps(direct_deps: HashMap<&String, Vec<&String>>) -> Result<(), String> {
    for name in direct_deps.keys() {
        let my_deps: HashSet<&String> = HashSet::new();
        check_circular_deps_from(name, my_deps, &direct_deps)?
    }
    Ok(())
}

fn check_circular_deps_from<'a>(
    name: &String,
    mut my_deps: HashSet<&'a String>,
    direct_deps: &HashMap<&String, Vec<&'a String>>,
) -> Result<(), String> {
    match direct_deps.get(name) {
        Some(deps) => {
            for dep in deps {
                if my_deps.insert(dep) {
                    return check_circular_deps_from(dep, my_deps, direct_deps);
                } else {
                    return circular_deps_error();
                }
            }
        }
        None => return deps_dont_exist_error(),
    }
    Ok(())
}

fn circular_deps_error() -> Result<(), String> {
    Err(String::from("Giving up because the config.json was invalid. I found a circular dependency! \nAt least one 'depends_on' eventually depends upon itself, meaning that the build_pipeline can never finish. Fix it"))
}

fn unique_names_error() -> Result<(), String> {
    Err(String::from("Giving up because the config.json was invalid. It had duplicate build_step_names, but they must all be unique"))
}

fn deps_dont_exist_error() -> Result<(), String> {
    Err(String::from("Giving up because the config.json was invalid. It has at least one 'depends_on' which doesn't exist. All depends_on must be lists of build_step_names which exist"))
}

#[cfg(test)]
mod test {
    mod unique_names {
        use super::super::*;

        #[test]
        fn when_build_step_names_are_unique_its_ok() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("D"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), Ok(()))
        }

        #[test]
        fn when_build_step_names_are_not_unique_return_error() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("sayHello"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("sayHello"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), unique_names_error())
        }
    }

    mod dependencies_exist {
        use super::super::*;

        #[test]
        fn when_all_depends_on_exist_its_ok() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("B")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), Ok(()))
        }

        #[test]
        fn when_there_are_no_depends_on_its_ok() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), Ok(()))
        }

        #[test]
        fn when_a_depends_on_does_not_exist_its_an_error() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("FAKE_NONESENSE")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), deps_dont_exist_error())
        }

        #[test]
        fn when_many_depends_on_do_not_exist_its_an_error() {
            let build_steps = vec![BuildStep {
                build_step_name: String::from("A"),
                command_type: CommandType::ShellCommand,
                command: String::from("echo 'hello'"),
                depends_on: vec![String::from("NOPE"), String::from("FAKE")],
                env_vars: None,
            }];

            assert_eq!(run(&build_steps), deps_dont_exist_error())
        }
    }

    mod circular_deps {
        use super::super::*;

        #[test]
        fn error_if_a_step_depends_on_itself() {
            let build_steps = vec![BuildStep {
                build_step_name: String::from("A"),
                command_type: CommandType::ShellCommand,
                command: String::from("echo 'hello'"),
                depends_on: vec![String::from("A")],
                env_vars: None,
            }];

            assert_eq!(run(&build_steps), circular_deps_error())
        }

        #[test]
        fn error_if_a_step_eventually_depends_on_itself() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("B")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("C")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("D"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("E")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("E"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), circular_deps_error())
        }

        #[test]
        fn ok_if_a_step_depends_on_nothing() {
            let build_steps = vec![BuildStep {
                build_step_name: String::from("A"),
                command_type: CommandType::ShellCommand,
                command: String::from("echo 'hello'"),
                depends_on: vec![],
                env_vars: None,
            }];

            assert_eq!(run(&build_steps), Ok(()))
        }

        #[test]
        fn ok_if_steps_have_many_dependencies_but_none_are_circular() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("B")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("D"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("E"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("F"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("E")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("G"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("H"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("G")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("I"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("H")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), Ok(()))
        }

        #[test]
        fn ok_if_steps_have_many_branching_dependencies_but_none_are_circular() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("D"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("B"), String::from("C")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("E"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("F"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("G"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("E"), String::from("F")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("H"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D"), String::from("G")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), Ok(()))
        }

        #[test]
        fn error_if_steps_have_many_branching_dependencies_and_one_is_circular() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("H")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("D"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("B"), String::from("C")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("E"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("F"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("G"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("E"), String::from("F")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("H"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D"), String::from("G")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), circular_deps_error())
        }
    }
}
