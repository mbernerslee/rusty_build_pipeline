mod command_line_arguments;
mod environment_variables;

pub fn determine(args: Vec<String>) -> Result<Setup, String> {
    let mut setup = default();
    let from_failed = environment_variables::read_from_failed()?;
    setup.from_failed = from_failed;
    command_line_arguments::parse(setup, args)
    //Ok(default())
    //match environment_variables::read() {
    //    Ok(from_failed) => from_config_file(from_failed),
    //    Err(error) => Err(error),
    //}
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Verbose,
    Debug,
    AnalyseSelfWorth,
}

#[derive(Debug, PartialEq)]
pub struct Setup {
    pub cwd: String,
    pub mode: Mode,
    pub from_failed: bool,
    pub show_stats: bool,
    pub json_report: bool,
    pub halt_when_done: bool,
}

pub fn default() -> Setup {
    Setup {
        cwd: String::from("."),
        mode: Mode::Normal,
        from_failed: false,
        show_stats: false,
        json_report: false,
        halt_when_done: true,
    }
}

//fn from_config_file(args: Vec<String>, from_failed: bool) -> Result<Setup, String> {
//    match config_file.read_setup()
//}
