use crate::build_step::{BuildStep, CommandType, EnvVar};
use crate::run::setup::Setup;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

//#[derive(Debug, PartialEq)]
mod which_runners_can_run;

#[derive(Debug)]
enum RunnerResponse {
    Success,
    Failure(String),
}

#[derive(Debug)]
pub struct Runner<'a> {
    start_sender: Sender<()>,
    build_step_name: &'a str,
    depends_on: &'a Vec<String>,
}

#[derive(Debug)]
struct RunnerCommand {
    command_type: CommandType,
    command: String,
    env_vars: Option<Vec<EnvVar>>,
}

pub fn run(setup: Setup, build_steps: Vec<BuildStep>) -> Result<(), String> {
    let (main_sender, main_receiver): (
        Sender<(usize, RunnerResponse)>,
        Receiver<(usize, RunnerResponse)>,
    ) = channel();

    let mut runners: HashMap<usize, Runner> = HashMap::new();

    for (n, build_step) in build_steps.iter().enumerate() {
        let (start_sender, start_receiver): (Sender<()>, Receiver<()>) = channel();
        let result_sender = main_sender.clone();

        let runner = Runner {
            build_step_name: &build_step.build_step_name,
            start_sender: start_sender,
            depends_on: &build_step.depends_on,
        };

        let runner_command = RunnerCommand {
            command_type: build_step.command_type.clone(),
            command: build_step.command.clone(),
            env_vars: build_step.env_vars.clone(),
        };

        thread::spawn(move || {
            println!("Runner {} - {}", n, runner_command.command);
            start_receiver.recv().unwrap();
            result_sender.send((n, RunnerResponse::Success)).unwrap()
        });

        runners.insert(n, runner);
    }

    let runner_count = runners.len();
    let mut started_runners: HashSet<usize> = HashSet::new();
    let mut finished_runners: HashSet<usize> = HashSet::new();

    for n in which_runners_can_run::determine(&runners, &started_runners, &finished_runners) {
        started_runners.insert(n);
        runners.get(&n).unwrap().start_sender.send(());
    }

    //for (_, runner) in &runners {
    //    match runner.start_sender.send(()) {
    //        Ok(()) => (),
    //        _ => return Err(String::from("nope")),
    //    }
    //}

    while finished_runners.len() < runner_count {
        match main_receiver.recv() {
            Ok((runner_num, msg)) => {
                finished_runners.insert(runner_num);
                println!("Runner {} finished", runner_num)
            }
            _ => {
                println!("A Runner got into serious trouble");
                return Err(String::from("nope"));
            }
        }
    }
    Ok(())
}
