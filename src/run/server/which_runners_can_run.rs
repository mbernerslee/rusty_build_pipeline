use super::Runner;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};

pub fn determine(
    runners: &HashMap<usize, Runner>,
    started: &HashSet<usize>,
    finished: &HashSet<usize>,
) -> Vec<usize> {
    let startable: Vec<usize> = Vec::new();

    let mut maybe_runnable: HashSet<usize> = HashSet::new();

    for n in runners.keys() {
        if !started.contains(n) && !finished.contains(n) {
            maybe_runnable.insert(*n);
        }
    }
    dbg!(maybe_runnable);
    startable
}

//#[cfg(test)]
//mod test {
//    mod determine {
//        use super::super::*;
//
//        fn build_runner<'a>(name: &'a String, depends_on: &'a Vec<String>) -> Runner<'a> {
//            let (start_sender, _): (Sender<()>, Receiver<()>) = channel();
//
//            Runner {
//                start_sender: start_sender,
//                build_step_name: name,
//                depends_on: depends_on,
//            }
//        }
//
//        #[test]
//        fn with_no_runners_returns_empty() {
//            let runners: HashMap<usize, Runner> = HashMap::new();
//            let started: HashSet<usize> = HashSet::new();
//            let finished: HashSet<usize> = HashSet::new();
//
//            let expected: Vec<usize> = Vec::new();
//            assert_eq!(determine(&runners, &started, &finished), expected)
//        }
//
//        #[test]
//        fn when_a_runner_has_no_depends_on_it_should_start() {
//            let mut runners: HashMap<usize, Runner> = HashMap::new();
//            let started: HashSet<usize> = HashSet::new();
//            let finished: HashSet<usize> = HashSet::new();
//
//            let name = String::from("A");
//            let depends_on: Vec<String> = Vec::new();
//
//            let runner = build_runner(&name, &depends_on);
//
//            runners.insert(0, runner);
//
//            let expected = vec![0];
//            assert_eq!(determine(&runners, &started, &finished), expected)
//        }
//    }
//}
