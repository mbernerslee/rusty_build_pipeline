use crate::build_step::*;

mod circular_dependencies;
mod dependencies_exist;
mod unique_names;

pub fn run(build_steps: &Vec<BuildStep>) -> Result<(), String> {
    unique_names::run(&build_steps)?;
    dependencies_exist::run(&build_steps)?;
    circular_dependencies::run(&build_steps)?;
    Ok(())
}

//TODO write appropriate amount of tests here
//#[cfg(test)]
//mod test {
//    mod determine_build_steps {
//        use super::super::*;
//
//        #[test]
//        fn when_build_step_names_are_unique_its_ok() {
//            let build_steps = vec![BuildStep {
//                build_step_name: String::from("sayHello"),
//                command_type: CommandType::ShellCommand,
//                command: String::from("echo 'hello'"),
//                depends_on: vec![],
//                env_vars: None,
//            }];
//
//            assert_eq!(run(&build_steps), Ok(()))
//        }
//
//        #[test]
//        fn when_build_step_names_are_not_unique_return_error() {
//            let build_steps = vec![
//                BuildStep {
//                    build_step_name: String::from("sayHello"),
//                    command_type: CommandType::ShellCommand,
//                    command: String::from("echo 'hello'"),
//                    depends_on: vec![],
//                    env_vars: None,
//                },
//                BuildStep {
//                    build_step_name: String::from("sayHello"),
//                    command_type: CommandType::ShellCommand,
//                    command: String::from("echo 'hello'"),
//                    depends_on: vec![],
//                    env_vars: None,
//                },
//            ];
//
//            assert_eq!(run(&build_steps), error())
//        }
//    }
//}
