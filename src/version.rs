pub fn display() -> () {
    write_version(&mut std::io::stdout())
}

fn write_version(stdout: &mut dyn std::io::Write) -> () {
    writeln!(stdout, "{}", VERSION).unwrap();
}

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod test {

    mod write_version {
        use super::super::*;

        #[test]
        fn returns_the_version_of_this_software() {
            let mut stdout = Vec::new();
            write_version(&mut stdout);
            assert_eq!(stdout, b"0.1.0\n")
        }
    }
}
