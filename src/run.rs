mod setup;

pub mod build_step;

pub fn main(args: Vec<String>) -> Result<(), String> {
    let _ = setup::determine(args);
    Ok(())
}
