mod setup;

pub fn main(args: Vec<String>) -> Result<(), String> {
    setup::determine(args);
    Ok(())
}
