mod server;
mod setup;

pub fn main(args: Vec<String>) -> Result<(), String> {
    let (setup, build_steps) = setup::determine(args)?;
    server::run(setup, build_steps)?;
    Ok(())
}
