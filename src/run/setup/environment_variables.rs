use std::env;
use std::env::VarError;
use std::env::VarError::*;

pub fn read() -> Result<bool, String> {
    read_with_reader(RealEnvVarReader {})
}

fn read_with_reader<R: EnvVarReader>(reader: R) -> Result<bool, String> {
    match reader.read() {
        Ok(from_failed) => parse(from_failed),
        Err(NotPresent) => Ok(false),
        _ => error(),
    }
}

struct RealEnvVarReader {}

impl EnvVarReader for RealEnvVarReader {
    fn read(&self) -> Result<String, VarError> {
        env::var("BUILD_PIPELINE_FROM_FAILED")
    }
}

trait EnvVarReader {
    fn read(&self) -> Result<String, VarError>;
}

fn parse(from_failed: String) -> Result<bool, String> {
    if from_failed == "true" {
        Ok(true)
    } else if from_failed == "false" {
        Ok(false)
    } else {
        error()
    }
}

fn error() -> Result<bool, String> {
    Err(String::from("I failed to initialise because the environment variable BUILD_PIPELINE_FROM_FAILED had a value I didn't understand, or I failed ot read it for some reason maybe. It can only be true or false"))
}

#[cfg(test)]
mod test {

    mod read_with_reader {
        use super::super::*;
        use std::env::VarError;
        use std::ffi::OsString;

        struct True {}

        impl EnvVarReader for True {
            fn read(&self) -> Result<String, VarError> {
                Ok(String::from("true"))
            }
        }

        struct False {}

        impl EnvVarReader for False {
            fn read(&self) -> Result<String, VarError> {
                Ok(String::from("false"))
            }
        }

        struct Jank {}

        impl EnvVarReader for Jank {
            fn read(&self) -> Result<String, VarError> {
                Ok(String::from("Jank"))
            }
        }

        struct ErrorNotUnicode {}

        impl EnvVarReader for ErrorNotUnicode {
            fn read(&self) -> Result<String, VarError> {
                Err(NotUnicode(OsString::new()))
            }
        }

        struct NotSet {}

        impl EnvVarReader for NotSet {
            fn read(&self) -> Result<String, VarError> {
                Err(NotPresent)
            }
        }

        #[test]
        fn when_its_true_return_true() {
            assert_eq!(read_with_reader(True {}), Ok(true))
        }

        #[test]
        fn when_its_false_return_false() {
            assert_eq!(read_with_reader(False {}), Ok(false))
        }

        #[test]
        fn error_when_its_jank() {
            let error = "I failed to initialise because the environment variable BUILD_PIPELINE_FROM_FAILED had a value I didn't understand, or I failed ot read it for some reason maybe. It can only be true or false";
            assert_eq!(read_with_reader(Jank {}), Err(String::from(error)))
        }

        #[test]
        fn error_when_theres_a_env_var_reading_error() {
            let error = "I failed to initialise because the environment variable BUILD_PIPELINE_FROM_FAILED had a value I didn't understand, or I failed ot read it for some reason maybe. It can only be true or false";
            assert_eq!(
                read_with_reader(ErrorNotUnicode {}),
                Err(String::from(error))
            )
        }

        #[test]
        fn false_when_env_var_not_set() {
            assert_eq!(read_with_reader(NotSet {}), Ok(false))
        }
    }
}
