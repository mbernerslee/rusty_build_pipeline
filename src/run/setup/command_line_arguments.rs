use crate::run::setup::Setup;
//use super::*;

pub fn parse(args: Vec<String>) -> Result<Setup, String> {
    Err(String::from("nope"))
}

#[cfg(test)]
mod test {

    mod parse {
        use super::super::*;
        use crate::run::setup;

        #[test]
        fn given_no_command_line_args_returns_the_default_setup() {
            assert_eq!(Ok(setup::default()), parse([].to_vec()));
        }
    }
}
