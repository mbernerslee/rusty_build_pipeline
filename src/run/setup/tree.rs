use super::Tree;
use crate::build_step::*;
use std::collections::{HashMap, HashSet};

//TODO return a data sctructure that gives the complete dependencies list for each build step
// do it as a separate module OR build it here AND rename this module

/*pub fn build<'a>(build_steps: &'a Vec<BuildStep>) -> Result<Tree<'a>, String> {
    let mut tree = Tree::new();
    build_tree(&mut tree, build_steps);
    Ok(tree)
}*/

fn build_tree<'a>(tree: &'a mut Tree<'a>, build_steps: &'a Vec<BuildStep>) {
    for build_step in build_steps {
        tree.insert(
            &build_step.build_step_name,
            HashSet::from_iter(build_step.depends_on.iter().map(|s| s.as_str())),
        );
    }

    for (build_step_name, depends_on) in tree.iter_mut() {
        let deps = get_indirect_dependencies(build_step_name, &tree);
        *depends_on = deps;
    }
}

fn get_indirect_dependencies<'a: 'b, 'b>(current_dep: &'a str, tree: &'a Tree) -> HashSet<&'b str> {
    let mut deps = HashSet::new();
    get_indirect_dependencies_helper(&mut deps, current_dep, tree);
    deps
}

fn get_indirect_dependencies_helper<'a: 'b, 'b>(
    deps: &mut HashSet<&'b str>,
    current_dep: &'a str,
    tree: &'a Tree,
) {
    deps.insert(current_dep);
    if let Some(children) = tree.get(current_dep) {
        for child in children {
            get_indirect_dependencies_helper(deps, child, tree);
        }
    }
}

fn circular_deps_error() -> String {
    String::from("Giving up because the config.json was invalid. I found a circular dependency! \nAt least one 'depends_on' eventually depends upon itself, meaning that the build_pipeline can never finish. Fix it")
}

fn unique_names_error() -> String {
    String::from("Giving up because the config.json was invalid. It had duplicate build_step_names, but they must all be unique")
}

fn deps_dont_exist_error() -> String {
    String::from("Giving up because the config.json was invalid. It has at least one 'depends_on' which doesn't exist. All depends_on must be lists of build_step_names which exist")
}

#[cfg(test)]
mod test {
    //mod unique_names {
    //    use super::super::*;

    //    #[test]
    //    fn when_build_step_names_are_unique_its_ok() {
    //        let a = String::from("A");
    //        let b = String::from("B");
    //        let c = String::from("C");
    //        let d = String::from("D");
    //        let build_steps = vec![
    //            BuildStep {
    //                build_step_name: a.clone(),
    //                command_type: CommandType::ShellCommand,
    //                command: String::from("echo 'hello'"),
    //                depends_on: vec![],
    //                env_vars: None,
    //            },
    //            BuildStep {
    //                build_step_name: b.clone(),
    //                command_type: CommandType::ShellCommand,
    //                command: String::from("echo 'hello'"),
    //                depends_on: vec![],
    //                env_vars: None,
    //            },
    //            BuildStep {
    //                build_step_name: c.clone(),
    //                command_type: CommandType::ShellCommand,
    //                command: String::from("echo 'hello'"),
    //                depends_on: vec![],
    //                env_vars: None,
    //            },
    //            BuildStep {
    //                build_step_name: d.clone(),
    //                command_type: CommandType::ShellCommand,
    //                command: String::from("echo 'hello'"),
    //                depends_on: vec![],
    //                env_vars: None,
    //            },
    //        ];

    //        let tree = Tree::from([
    //            (&a, Branch::from([])),
    //            (&b, Branch::from([])),
    //            (&c, Branch::from([])),
    //            (&d, Branch::from([])),
    //        ]);

    //        assert_eq!(build(&build_steps), Ok(tree))
    //    }

    //    #[test]
    //    fn when_build_step_names_are_not_unique_return_error() {
    //        let build_steps = vec![
    //            BuildStep {
    //                build_step_name: String::from("sayHello"),
    //                command_type: CommandType::ShellCommand,
    //                command: String::from("echo 'hello'"),
    //                depends_on: vec![],
    //                env_vars: None,
    //            },
    //            BuildStep {
    //                build_step_name: String::from("sayHello"),
    //                command_type: CommandType::ShellCommand,
    //                command: String::from("echo 'hello'"),
    //                depends_on: vec![],
    //                env_vars: None,
    //            },
    //        ];

    //        assert_eq!(build(&build_steps), Err(unique_names_error()))
    //    }
    //}

    // mod dependencies_exist {
    //     use super::super::*;

    //     #[test]
    //     fn when_all_depends_on_exist_its_ok() {
    //         let a = String::from("A");
    //         let b = String::from("B");
    //         let c = String::from("C");

    //         let build_steps = vec![
    //             BuildStep {
    //                 build_step_name: a.clone(),
    //                 command_type: CommandType::ShellCommand,
    //                 command: String::from("echo 'hello'"),
    //                 depends_on: vec![],
    //                 env_vars: None,
    //             },
    //             BuildStep {
    //                 build_step_name: b.clone(),
    //                 command_type: CommandType::ShellCommand,
    //                 command: String::from("echo 'hello'"),
    //                 depends_on: vec![a.clone()],
    //                 env_vars: None,
    //             },
    //             BuildStep {
    //                 build_step_name: c.clone(),
    //                 command_type: CommandType::ShellCommand,
    //                 command: String::from("echo 'hello'"),
    //                 depends_on: vec![b.clone()],
    //                 env_vars: None,
    //             },
    //         ];

    //         let tree = Tree::from([
    //             (&a, Branch::from([])),
    //             (&b, Branch::from([&a])),
    //             (&c, Branch::from([&a, &b])),
    //         ]);

    //         assert_eq!(build(&build_steps), Ok(tree))
    //     }

    //     #[test]
    //     fn when_there_are_no_depends_on_its_ok() {
    //         let a = String::from("A");
    //         let b = String::from("B");
    //         let c = String::from("C");

    //         let build_steps = vec![
    //             BuildStep {
    //                 build_step_name: a.clone(),
    //                 command_type: CommandType::ShellCommand,
    //                 command: String::from("echo 'hello'"),
    //                 depends_on: vec![],
    //                 env_vars: None,
    //             },
    //             BuildStep {
    //                 build_step_name: b.clone(),
    //                 command_type: CommandType::ShellCommand,
    //                 command: String::from("echo 'hello'"),
    //                 depends_on: vec![],
    //                 env_vars: None,
    //             },
    //             BuildStep {
    //                 build_step_name: c.clone(),
    //                 command_type: CommandType::ShellCommand,
    //                 command: String::from("echo 'hello'"),
    //                 depends_on: vec![],
    //                 env_vars: None,
    //             },
    //         ];

    //         let tree = Tree::from([
    //             (&a, Branch::from([])),
    //             (&b, Branch::from([])),
    //             (&c, Branch::from([])),
    //         ]);

    //         assert_eq!(build(&build_steps), Ok(tree))
    //     }

    //     #[test]
    //     fn when_a_depends_on_does_not_exist_its_an_error() {
    //         let build_steps = vec![
    //             BuildStep {
    //                 build_step_name: String::from("A"),
    //                 command_type: CommandType::ShellCommand,
    //                 command: String::from("echo 'hello'"),
    //                 depends_on: vec![],
    //                 env_vars: None,
    //             },
    //             BuildStep {
    //                 build_step_name: String::from("B"),
    //                 command_type: CommandType::ShellCommand,
    //                 command: String::from("echo 'hello'"),
    //                 depends_on: vec![String::from("A")],
    //                 env_vars: None,
    //             },
    //             BuildStep {
    //                 build_step_name: String::from("C"),
    //                 command_type: CommandType::ShellCommand,
    //                 command: String::from("echo 'hello'"),
    //                 depends_on: vec![String::from("FAKE_NONESENSE")],
    //                 env_vars: None,
    //             },
    //         ];

    //         assert_eq!(build(&build_steps), Err(deps_dont_exist_error()))
    //     }

    //     #[test]
    //     fn when_many_depends_on_do_not_exist_its_an_error() {
    //         let build_steps = vec![BuildStep {
    //             build_step_name: String::from("A"),
    //             command_type: CommandType::ShellCommand,
    //             command: String::from("echo 'hello'"),
    //             depends_on: vec![String::from("NOPE"), String::from("FAKE")],
    //             env_vars: None,
    //         }];

    //         assert_eq!(build(&build_steps), Err(deps_dont_exist_error()))
    //     }
    // }

    mod circular_deps {
        use super::super::*;

        //#[test]
        //fn error_if_a_step_depends_on_itself() {
        //    let build_steps = vec![BuildStep {
        //        build_step_name: String::from("A"),
        //        command_type: CommandType::ShellCommand,
        //        command: String::from("echo 'hello'"),
        //        depends_on: vec![String::from("A")],
        //        env_vars: None,
        //    }];

        //    assert_eq!(build(&build_steps), Err(circular_deps_error()))
        //}

        //#[test]
        //fn error_if_a_step_eventually_depends_on_itself() {
        //    let build_steps = vec![
        //        BuildStep {
        //            build_step_name: String::from("A"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("B")],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("B"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("C")],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("C"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("D")],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("D"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("E")],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("E"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("A")],
        //            env_vars: None,
        //        },
        //    ];

        //    assert_eq!(build(&build_steps), Err(circular_deps_error()))
        //}

        //#[test]
        //fn ok_if_a_step_depends_on_nothing() {
        //    let a = String::from("A");

        //    let build_steps = vec![BuildStep {
        //        build_step_name: a.clone(),
        //        command_type: CommandType::ShellCommand,
        //        command: String::from("echo 'hello'"),
        //        depends_on: vec![],
        //        env_vars: None,
        //    }];

        //    let tree = Tree::from([(&a, Branch::from([]))]);

        //    assert_eq!(build(&build_steps), Ok(tree))
        //}

        //#[test]
        //fn ok_if_steps_have_many_dependencies_but_none_are_circular() {
        //    let a = String::from("A");
        //    let b = String::from("B");
        //    let c = String::from("C");
        //    let d = String::from("D");
        //    let e = String::from("E");
        //    let f = String::from("F");
        //    let g = String::from("G");
        //    let h = String::from("H");
        //    let i = String::from("I");

        //    let build_steps = vec![
        //        BuildStep {
        //            build_step_name: String::from("A"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("B"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("A")],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("C"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("B")],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("D"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("E"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("D")],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("F"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("E")],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("G"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("H"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("G")],
        //            env_vars: None,
        //        },
        //        BuildStep {
        //            build_step_name: String::from("I"),
        //            command_type: CommandType::ShellCommand,
        //            command: String::from("echo 'hello'"),
        //            depends_on: vec![String::from("H")],
        //            env_vars: None,
        //        },
        //    ];

        //    let tree = Tree::from([
        //        (&a, Branch::from([])),
        //        (&b, Branch::from([&a])),
        //        (&c, Branch::from([&b, &a])),
        //        (&d, Branch::from([])),
        //        (&e, Branch::from([&d])),
        //        (&f, Branch::from([&e, &d])),
        //        (&g, Branch::from([])),
        //        (&h, Branch::from([&g])),
        //        (&i, Branch::from([&h, &g])),
        //    ]);

        //    assert_eq!(build(&build_steps), Ok(tree))
        //}

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
                //BuildStep {
                //    build_step_name: String::from("E"),
                //    command_type: CommandType::ShellCommand,
                //    command: String::from("echo 'hello'"),
                //    depends_on: vec![String::from("D")],
                //    env_vars: None,
                //},
                //BuildStep {
                //    build_step_name: String::from("F"),
                //    command_type: CommandType::ShellCommand,
                //    command: String::from("echo 'hello'"),
                //    depends_on: vec![String::from("D")],
                //    env_vars: None,
                //},
                //BuildStep {
                //    build_step_name: String::from("G"),
                //    command_type: CommandType::ShellCommand,
                //    command: String::from("echo 'hello'"),
                //    depends_on: vec![String::from("E"), String::from("F")],
                //    env_vars: None,
                //},
                //BuildStep {
                //    build_step_name: String::from("H"),
                //    command_type: CommandType::ShellCommand,
                //    command: String::from("echo 'hello'"),
                //    depends_on: vec![String::from("D"), String::from("G")],
                //    env_vars: None,
                //},
            ];
            // (H, D, B, A), (H, D, C, A),
            // (H, G, E, D, B, A), (H, G, E, D, C, A), (H, G, F, D, B, A), (H, G, F, D, C, A)

            let expected_tree = Tree::from([
                ("A", HashSet::from([])),
                ("B", HashSet::from(["A"])),
                ("C", HashSet::from(["A"])),
                ("D", HashSet::from(["A", "B", "C"])),
                ("E", HashSet::from(["A", "B", "C", "D"])),
                ("F", HashSet::from(["A", "B", "C", "D"])),
                ("G", HashSet::from(["A", "B", "C", "D", "E", "F"])),
                ("H", HashSet::from(["A", "B", "C", "D", "E", "F", "G"])),
            ]);

            let mut tree = Tree::new();
            build_tree(&mut tree, &build_steps);
            assert_eq!(expected_tree, tree)
        }

        //    #[test]
        //    fn error_if_steps_have_many_branching_dependencies_and_one_is_circular() {
        //        let build_steps = vec![
        //            BuildStep {
        //                build_step_name: String::from("A"),
        //                command_type: CommandType::ShellCommand,
        //                command: String::from("echo 'hello'"),
        //                depends_on: vec![String::from("H")],
        //                env_vars: None,
        //            },
        //            BuildStep {
        //                build_step_name: String::from("B"),
        //                command_type: CommandType::ShellCommand,
        //                command: String::from("echo 'hello'"),
        //                depends_on: vec![String::from("A")],
        //                env_vars: None,
        //            },
        //            BuildStep {
        //                build_step_name: String::from("C"),
        //                command_type: CommandType::ShellCommand,
        //                command: String::from("echo 'hello'"),
        //                depends_on: vec![String::from("A")],
        //                env_vars: None,
        //            },
        //            BuildStep {
        //                build_step_name: String::from("D"),
        //                command_type: CommandType::ShellCommand,
        //                command: String::from("echo 'hello'"),
        //                depends_on: vec![String::from("B"), String::from("C")],
        //                env_vars: None,
        //            },
        //            BuildStep {
        //                build_step_name: String::from("E"),
        //                command_type: CommandType::ShellCommand,
        //                command: String::from("echo 'hello'"),
        //                depends_on: vec![String::from("D")],
        //                env_vars: None,
        //            },
        //            BuildStep {
        //                build_step_name: String::from("F"),
        //                command_type: CommandType::ShellCommand,
        //                command: String::from("echo 'hello'"),
        //                depends_on: vec![String::from("D")],
        //                env_vars: None,
        //            },
        //            BuildStep {
        //                build_step_name: String::from("G"),
        //                command_type: CommandType::ShellCommand,
        //                command: String::from("echo 'hello'"),
        //                depends_on: vec![String::from("E"), String::from("F")],
        //                env_vars: None,
        //            },
        //            BuildStep {
        //                build_step_name: String::from("H"),
        //                command_type: CommandType::ShellCommand,
        //                command: String::from("echo 'hello'"),
        //                depends_on: vec![String::from("D"), String::from("G")],
        //                env_vars: None,
        //            },
        //        ];

        //        assert_eq!(build(&build_steps), Err(circular_deps_error()))
        //    }
    }
}
