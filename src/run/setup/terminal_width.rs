extern crate termion;
use crate::run::setup::Setup;

pub fn add_to_setup(setup: Setup) -> Result<Setup, String> {
    add_width_to_setup(setup, fetch_terminal_size())
}

fn add_width_to_setup(
    mut setup: Setup,
    result: std::io::Result<(u16, u16)>,
) -> Result<Setup, String> {
    match result {
        Ok((columns, _rows)) => {
            setup.terminal_width = columns;
            Ok(setup)
        }
        _termion_error => error(),
    }
}

//when not in test, run the real termion::terminal_size()
#[cfg(not(test))]
fn fetch_terminal_size() -> std::io::Result<(u16, u16)> {
    termion::terminal_size()
}

//when in test, return a stubbed successul termion::terminal_size() response, because when tests
//run there's no terminal so termion always returns error
#[cfg(test)]
fn fetch_terminal_size() -> std::io::Result<(u16, u16)> {
    Ok((80, 123))
}

fn error() -> Result<Setup, String> {
    Err(String::from(
        "Giving up because I could not determine the terminal width of your terminal.",
    ))
}

#[cfg(test)]
mod test {
    mod add_width_to_setup {
        use super::super::*;
        use crate::run::setup;

        #[test]
        fn when_we_can_find_the_terminal_width_add_it_to_the_setup() {
            let initial = setup::default();
            let mut expected = setup::default();

            let cols = 123;
            let rows = 456;
            let size = (cols, rows);

            expected.terminal_width = cols;

            // a bit weird to do this, but I want to make sure we're not accidentally getting cols
            // & rows muddled up, so they must be different
            assert_ne!(cols, rows);
            assert_eq!(add_width_to_setup(initial, Ok(size)), Ok(expected))
        }

        #[test]
        fn errors_when_we_cannot_find_the_terminal_width() {
            let initial = setup::default();

            let bad = Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no"));
            assert_eq!(add_width_to_setup(initial, bad), error())
        }
    }
}
