use crate::run::setup::Mode;
use crate::run::setup::Setup;

pub fn parse(setup: Setup, args: Vec<String>) -> Result<Setup, String> {
    if args_are_compatible(&args) {
        build_setup(args.into_iter(), setup)
    } else {
        Err(incompatible())
    }
}

fn args_are_compatible(args: &Vec<String>) -> bool {
    for (arg, bad_args) in bad_arg_combos() {
        if args.iter().any(|a| a == arg) {
            for bad_arg in &bad_args {
                if args.iter().any(|a| a == bad_arg) {
                    return false;
                }
            }
        }
    }
    true
}

fn bad_arg_combos() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        (DEBUG, vec![VERBOSE, STATS, JSON_REPORT]),
        (RUN_ALL, vec![FROM_FAILED]),
        (
            ANALYSE_SELF_WORTH,
            vec![VERBOSE, DEBUG, FROM_FAILED, RUN_ALL, STATS, JSON_REPORT],
        ),
    ]
}

const CWD: &'static str = "--cwd";
const ANALYSE_SELF_WORTH: &'static str = "--analyse-self-worth";
const DEBUG: &'static str = "--debug";
const FROM_FAILED: &'static str = "--ff";
const RUN_ALL: &'static str = "--ra";
const VERBOSE: &'static str = "--verbose";
const STATS: &'static str = "--stats";
const JSON_REPORT: &'static str = "--json-report";

fn build_setup<'a, T>(mut args: T, setup: Setup) -> Result<Setup, String>
where
    T: Iterator<Item = String>,
{
    match args.next() {
        Some(arg) => build_setup_from_arg(arg, args, setup),
        None => Ok(setup),
    }
}

fn build_setup_from_arg<'a, T>(arg: String, mut args: T, mut setup: Setup) -> Result<Setup, String>
where
    T: Iterator<Item = String>,
{
    let result = match arg.as_str() {
        CWD => add_cwd(&mut args, setup),
        VERBOSE => {
            setup.mode = Mode::Verbose;
            Ok(setup)
        }
        ANALYSE_SELF_WORTH => {
            setup.mode = Mode::AnalyseSelfWorth;
            setup.from_failed = false;
            Ok(setup)
        }
        DEBUG => {
            setup.mode = Mode::Debug;
            Ok(setup)
        }
        FROM_FAILED => {
            setup.from_failed = true;
            Ok(setup)
        }
        RUN_ALL => {
            setup.from_failed = false;
            Ok(setup)
        }
        STATS => {
            setup.show_stats = true;
            Ok(setup)
        }
        JSON_REPORT => {
            setup.json_report = true;
            Ok(setup)
        }
        _ => error(),
    };

    match result {
        Ok(setup) => build_setup(args, setup),
        Err(error) => Err(error),
    }
}

fn add_cwd<'a, T>(args: &mut T, mut setup: Setup) -> Result<Setup, String>
where
    T: Iterator<Item = String>,
{
    match args.next() {
        Some(dir) => {
            setup.cwd = dir.to_string();
            Ok(setup)
        }
        None => error(),
    }
}

fn error() -> Result<Setup, String> {
    Err(usage())
}

//keep this in sync with what's in the README
pub fn usage() -> String {
    r#"usage: ./bp run [--cwd ./path/to/directory/to/use] [--verbose or --debug] [--ff or --ra] [--stats] [--json-report]

    --verbose  - prints output from successful as well as failed build steps to the terminal. Cannot be set with --debug

    --debug    - build steps run one at a time and their output is printed to the terminal in real time. Cannot be set with --verbose. If you're scratching your head wondering what's going wrong, this flag is reccommended.

    --cwd path - the path in which to look for the `build_pipeline` directory which must contain `config.json` and build `scripts` folder. Defaults to "."

    --ff       - from-failed: saves the results of this run to "{your cwd}/build_pipeline/previous_run_result.json", and if sed file already exists, then only build steps that were either failed or not started from the previous build will run. Previously successful build steps will not be run. Cannot be set with --ra. from-failed is smart enough to know that if all the build steps we were about to run were going to be skipped - to instead run all the steps.

    --ra       - run-all: in the event that from-failed mode is set by an environment variable, this can be used to override it and force all build steps to run (as is the default behaviour). Cannot be set with --ff

    --stats    - puts some additional output at the end of the run - showing the ranking of each dependency "branch" by speed, showing the speed of each build step within it too. Cannot be set with --debug

    --json-report - creates a json report including all the steps run, their timings and whether they succeeded or not. Cannot be set with --debug

    --analyse-self-worth    - Runs the full build pipeline twice. Once with full parallism including build_pipeine overhead, and once serially without build_pipeline overhead. Reports the timings of both. Useful for finding out how much time (if any) is saved by running your build with build_pipeline. Doesn't work unless `bp` is in your PATH! Basically every other command line argument except --cwd is incompatible with this one."#.to_string()
}

pub fn incompatible() -> String {
    r#"I was given some incompatible arguments.

    --ff cannot be set with --ra
    --debug cannot be set with --verbose
    --debug cannot be set with --stats
    --debug cannot be set with --json-report

    --analyse-self-worth cannot be set with any of the following
       --verbose
       --debug
       --ff
       --ra
       --stats
       --json-report
    "#
    .to_string()
}

#[cfg(test)]
mod test {

    fn build_args(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    mod parse {
        use super::super::*;
        use super::*;
        use crate::run::setup;
        use crate::run::setup::Mode;
        use crate::run::setup::Setup;

        #[test]
        fn given_no_command_line_args_returns_the_default_setup() {
            let expected_default_setup = Setup {
                cwd: String::from("."),
                mode: Mode::Normal,
                from_failed: false,
                show_stats: false,
                json_report: false,
                halt_when_done: true,
                terminal_width: 80,
            };
            assert_eq!(&setup::default(), &expected_default_setup);
            assert_eq!(
                Ok(setup::default()),
                parse(setup::default(), build_args(&[]))
            );
        }

        #[test]
        fn errors_given_jank_command_line_args() {
            let args = build_args(&["jankety", "jank"]);
            assert_eq!(Err(usage()), parse(setup::default(), args));
        }

        #[test]
        fn given_cwd_sets_it_in_the_setup() {
            let args = build_args(&["--cwd", "some/cool/path"]);
            let mut expected_setup = setup::default();
            expected_setup.cwd = String::from("some/cool/path");
            assert_eq!(Ok(expected_setup), parse(setup::default(), args));
        }

        #[test]
        fn given_verbose_sets_verbose_mode() {
            let args = build_args(&["--verbose"]);
            let mut expected_setup = setup::default();
            expected_setup.mode = Mode::Verbose;
            assert_eq!(Ok(expected_setup), parse(setup::default(), args));
        }

        #[test]
        fn given_debug_sets_debug_mode() {
            let args = build_args(&["--debug"]);
            let mut expected_setup = setup::default();
            expected_setup.mode = Mode::Debug;
            assert_eq!(Ok(expected_setup), parse(setup::default(), args));
        }

        #[test]
        fn given_from_failed_sets_from_failed() {
            let args = build_args(&["--ff"]);
            let mut expected_setup = setup::default();
            expected_setup.from_failed = true;
            assert_eq!(Ok(expected_setup), parse(setup::default(), args));
        }

        #[test]
        fn given_run_all_sets_from_failed_false() {
            let args = build_args(&["--ra"]);
            let mut setup = setup::default();
            setup.from_failed = true;

            let mut expected_setup = setup::default();
            expected_setup.from_failed = false;

            assert_eq!(Ok(expected_setup), parse(setup, args));
        }

        #[test]
        fn given_analyse_self_worth_sets_it_as_the_mode() {
            let args = build_args(&["--analyse-self-worth"]);
            let mut expected_setup = setup::default();
            expected_setup.mode = Mode::AnalyseSelfWorth;

            assert_eq!(Ok(expected_setup), parse(setup::default(), args));
        }

        #[test]
        fn analyse_self_worth_sets_from_failed_to_false() {
            let args = build_args(&["--analyse-self-worth"]);
            let mut setup = setup::default();
            setup.from_failed = true;

            let mut expected_setup = setup::default();
            expected_setup.mode = Mode::AnalyseSelfWorth;
            expected_setup.from_failed = false;

            assert_eq!(Ok(expected_setup), parse(setup, args));
        }

        #[test]
        fn given_multiple_flags_we_can_set_them_all() {
            let args = build_args(&[
                "--verbose",
                "--cwd",
                "some/path",
                "--ff",
                "--stats",
                "--json-report",
            ]);
            let mut expected_setup = setup::default();
            expected_setup.mode = Mode::Verbose;
            expected_setup.cwd = String::from("some/path");
            expected_setup.from_failed = true;
            expected_setup.show_stats = true;
            expected_setup.json_report = true;
            assert_eq!(Ok(expected_setup), parse(setup::default(), args));
        }

        #[test]
        fn errors_given_multiple_valid_flags_but_one_bad_one() {
            let args = build_args(&[
                "--verbose",
                "--cwd",
                "some/path",
                "--ff",
                "--stats",
                "--json-report",
                "--jank",
            ]);
            assert_eq!(Err(usage()), parse(setup::default(), args));
        }
    }

    mod parse_incompatible_args {
        use super::super::*;
        use super::*;
        use crate::run::setup;

        #[test]
        fn debug_and_verbose_are_incompatible() {
            let args = build_args(&["--debug", "--verbose"]);
            assert_eq!(Err(incompatible()), parse(setup::default(), args));
        }

        #[test]
        fn debug_and_stats_are_incompatible() {
            let args = build_args(&["--debug", "--stats"]);
            assert_eq!(Err(incompatible()), parse(setup::default(), args));
        }

        #[test]
        fn debug_and_json_report_are_incompatible() {
            let args = build_args(&["--debug", "--json-report"]);
            assert_eq!(Err(incompatible()), parse(setup::default(), args));
        }

        #[test]
        fn run_all_and_from_failed_are_incompatible() {
            let args = build_args(&["--ra", "--ff"]);
            assert_eq!(Err(incompatible()), parse(setup::default(), args));
        }

        #[test]
        fn analyse_self_worth_and_verbose_are_incompatible() {
            let args = build_args(&["--analyse-self-worth", "--verbose"]);
            assert_eq!(Err(incompatible()), parse(setup::default(), args));
        }

        #[test]
        fn analyse_self_worth_and_debug_are_incompatible() {
            let args = build_args(&["--analyse-self-worth", "--debug"]);
            assert_eq!(Err(incompatible()), parse(setup::default(), args));
        }

        #[test]
        fn analyse_self_worth_and_from_failed_are_incompatible() {
            let args = build_args(&["--analyse-self-worth", "--ff"]);
            assert_eq!(Err(incompatible()), parse(setup::default(), args));
        }

        #[test]
        fn analyse_self_worth_and_run_all_are_incompatible() {
            let args = build_args(&["--analyse-self-worth", "--ra"]);
            assert_eq!(Err(incompatible()), parse(setup::default(), args));
        }

        #[test]
        fn analyse_self_worth_and_stats_are_incompatible() {
            let args = build_args(&["--analyse-self-worth", "--stats"]);
            assert_eq!(Err(incompatible()), parse(setup::default(), args));
        }

        #[test]
        fn analyse_self_worth_and_json_report_are_incompatible() {
            let args = build_args(&["--analyse-self-worth", "--json-report"]);
            assert_eq!(Err(incompatible()), parse(setup::default(), args));
        }
    }
}
