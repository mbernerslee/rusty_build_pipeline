pub fn read(cwd: &String) -> Result<String, String> {
    read_with_reader(RealFileReader {}, cwd)
}

fn read_with_reader<R: FileReader>(reader: R, cwd: &String) -> Result<String, String> {
    let file_path = file_path(cwd);
    match reader.read_to_string(&file_path) {
        Ok(contents) => Ok(contents),
        _ => error(&file_path),
    }
}

fn error(file_path: &String) -> Result<String, String> {
    let error = String::from(format!("Failed to open the {} file", file_path));
    Err(error)
}

fn file_path(cwd: &String) -> String {
    format!("{}/build_pipeline/config.json", cwd)
}

struct RealFileReader {}

impl FileReader for RealFileReader {
    fn read_to_string(&self, file: &String) -> std::io::Result<String> {
        std::fs::read_to_string(file)
    }
}

trait FileReader {
    fn read_to_string(&self, file: &String) -> std::io::Result<String>;
}

#[cfg(test)]
mod test {
    mod read_with_reader {
        use super::super::*;

        struct ErrorReader {}

        impl FileReader for ErrorReader {
            fn read_to_string(&self, _file: &String) -> std::io::Result<String> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no"))
            }
        }

        #[test]
        fn when_reading_the_file_errors_return_error() {
            let cwd = String::from("some/path");
            let error = error(&file_path(&cwd));
            assert_eq!(read_with_reader(ErrorReader {}, &cwd), error)
        }
        //TODO continue here
    }

    mod file_path {
        use super::super::*;

        #[test]
        fn given_a_dir_returns_the_config_file_path() {
            let cwd = String::from("some/path");
            let expected = String::from("some/path/build_pipeline/config.json");
            assert_eq!(file_path(&cwd), expected)
        }
    }
}
