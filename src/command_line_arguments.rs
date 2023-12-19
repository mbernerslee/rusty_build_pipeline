pub enum RunOrInit {
    Run,
    Init,
}

pub struct CLICommand {
    pub kind: RunOrInit,
    pub args: Vec<String>,
}

pub fn parse(cli_args: Vec<String>) -> Option<CLICommand> {
    None

    //let args: Vec<String> = env::args().collect();
    //let mode = &args[1];
    //let mode_args = &args[2..];
    //
    //if mode == "run" {
    //    println!("run!")
    //} else if mode == "init" {
    //    println!("init!")
    //} else {
    //}
}
