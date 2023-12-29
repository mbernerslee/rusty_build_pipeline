use super::directory_structure::DirectoryStructure;
use super::directory_structure::DirectoryStructurer;

use std::fs;
use std::fs::File;
use std::io::Write;

pub fn create<D: DirectoryStructurer>(d: &D) -> Result<(), String> {
    check_working_directory_exists(d)?;
    check_build_pipeline_directory_does_not_exist(d)?;
    create_build_pipeline_directory(d)?;
    create_scripts_directory(d)?;
    create_config_file(d)
}

impl DirectoryStructurer for DirectoryStructure {
    fn working_directory_path_exists(&self) -> bool {
        self.working_directory_path.exists()
    }

    fn build_pipeline_directory_exists(&self) -> bool {
        self.build_pipeline_path.exists()
    }

    fn create_build_pipeline_directory(&self) -> std::io::Result<()> {
        fs::create_dir(&self.build_pipeline_path)
    }

    fn create_scripts_directory(&self) -> std::io::Result<()> {
        fs::create_dir(&self.scripts_path)
    }

    fn create_empty_config_file(&self) -> std::io::Result<std::fs::File> {
        File::create(&self.config_path)
    }

    fn write_to_config_file(&self, mut config_file: std::fs::File) -> std::io::Result<()> {
        config_file.write_all(self.config_contents)
    }
}

fn check_working_directory_exists<D: DirectoryStructurer>(d: &D) -> Result<(), String> {
    if d.working_directory_path_exists() {
        Ok(())
    } else {
        Err(String::from("I failed because the directory you supplied does not exist. I can only initialise in a directory that already exists."))
    }
}

fn check_build_pipeline_directory_does_not_exist<D: DirectoryStructurer>(
    d: &D,
) -> Result<(), String> {
    if d.build_pipeline_directory_exists() {
        Err(create_build_pipeline_directory_error())
    } else {
        Ok(())
    }
}

fn create_build_pipeline_directory<D: DirectoryStructurer>(d: &D) -> Result<(), String> {
    match d.create_build_pipeline_directory() {
        Ok(_) => Ok(()),
        _ => Err(create_build_pipeline_directory_error()),
    }
}

fn create_scripts_directory<D: DirectoryStructurer>(d: &D) -> Result<(), String> {
    match d.create_scripts_directory() {
        Ok(_) => Ok(()),
        _ => Err(String::from(
            "I Failed because I couldn't create the build_pipeline/scripts directory",
        )),
    }
}

fn create_build_pipeline_directory_error() -> String {
    String::from("I failed because the build_pipeline directory I wanted to create already exists. Maybe you've already initialised before?")
}

fn create_config_file<D: DirectoryStructurer>(d: &D) -> Result<(), String> {
    match d.create_empty_config_file() {
        Ok(config_file) => write_config_file(d, config_file),
        _ => Err(String::from(
            "I failed because I could not create the build_pipeline/config.json file",
        )),
    }
}

fn write_config_file<D: DirectoryStructurer>(d: &D, config_file: File) -> Result<(), String> {
    match d.write_to_config_file(config_file) {
        Ok(()) => Ok(()),
        _ => Err(String::from("I failed because I could not write to the build_pipeline/config.json file, which I have created but have left empty.")),
    }
}

#[cfg(test)]
mod test {

    mod create {
        use super::super::super::directory_structure::DirectoryStructurer;
        use super::super::*;

        struct Works {}

        impl DirectoryStructurer for Works {
            fn working_directory_path_exists(&self) -> bool {
                true
            }

            fn build_pipeline_directory_exists(&self) -> bool {
                false
            }

            fn create_build_pipeline_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_scripts_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_empty_config_file(&self) -> std::io::Result<std::fs::File> {
                File::open("src/init/directory_structure_creation.rs")
            }

            fn write_to_config_file(&self, _config_file: std::fs::File) -> std::io::Result<()> {
                Ok(())
            }
        }

        struct MissingWorkingDirectoryPath {}

        impl DirectoryStructurer for MissingWorkingDirectoryPath {
            fn working_directory_path_exists(&self) -> bool {
                false
            }

            fn build_pipeline_directory_exists(&self) -> bool {
                false
            }

            fn create_build_pipeline_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_scripts_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_empty_config_file(&self) -> std::io::Result<std::fs::File> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no"))
            }

            fn write_to_config_file(&self, _config_file: std::fs::File) -> std::io::Result<()> {
                Ok(())
            }
        }

        struct BuildPipelineDirectoryExists {}

        impl DirectoryStructurer for BuildPipelineDirectoryExists {
            fn working_directory_path_exists(&self) -> bool {
                true
            }

            fn build_pipeline_directory_exists(&self) -> bool {
                true
            }

            fn create_build_pipeline_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_scripts_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_empty_config_file(&self) -> std::io::Result<std::fs::File> {
                File::open("src/init/directory_structure_creation.rs")
            }

            fn write_to_config_file(&self, _config_file: std::fs::File) -> std::io::Result<()> {
                Ok(())
            }
        }

        struct CreateBPDirFails {}

        impl DirectoryStructurer for CreateBPDirFails {
            fn working_directory_path_exists(&self) -> bool {
                true
            }

            fn build_pipeline_directory_exists(&self) -> bool {
                false
            }

            fn create_build_pipeline_directory(&self) -> std::io::Result<()> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no"))
            }

            fn create_scripts_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_empty_config_file(&self) -> std::io::Result<std::fs::File> {
                File::open("src/init/directory_structure_creation.rs")
            }

            fn write_to_config_file(&self, _config_file: std::fs::File) -> std::io::Result<()> {
                Ok(())
            }
        }

        struct CreateScriptDirFails {}

        impl DirectoryStructurer for CreateScriptDirFails {
            fn working_directory_path_exists(&self) -> bool {
                true
            }

            fn build_pipeline_directory_exists(&self) -> bool {
                false
            }

            fn create_build_pipeline_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_scripts_directory(&self) -> std::io::Result<()> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no"))
            }

            fn create_empty_config_file(&self) -> std::io::Result<std::fs::File> {
                File::open("src/init/directory_structure_creation.rs")
            }

            fn write_to_config_file(&self, _config_file: std::fs::File) -> std::io::Result<()> {
                Ok(())
            }
        }

        struct CreateConfigFileFails {}

        impl DirectoryStructurer for CreateConfigFileFails {
            fn working_directory_path_exists(&self) -> bool {
                true
            }

            fn build_pipeline_directory_exists(&self) -> bool {
                false
            }

            fn create_build_pipeline_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_scripts_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_empty_config_file(&self) -> std::io::Result<std::fs::File> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no"))
            }

            fn write_to_config_file(&self, _config_file: std::fs::File) -> std::io::Result<()> {
                Ok(())
            }
        }

        struct WriteConfigFileFails {}

        impl DirectoryStructurer for WriteConfigFileFails {
            fn working_directory_path_exists(&self) -> bool {
                true
            }

            fn build_pipeline_directory_exists(&self) -> bool {
                false
            }

            fn create_build_pipeline_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_scripts_directory(&self) -> std::io::Result<()> {
                Ok(())
            }

            fn create_empty_config_file(&self) -> std::io::Result<std::fs::File> {
                File::open("src/init/directory_structure_creation.rs")
            }

            fn write_to_config_file(&self, _config_file: std::fs::File) -> std::io::Result<()> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no"))
            }
        }

        //no tests change the file system, asserting the file system is actually changed only
        //happens once in the init.rs tests
        #[test]
        fn works() {
            let directory_structure = Works {};
            assert_eq!(Ok(()), create(&directory_structure));
        }

        #[test]
        fn errors_if_the_working_directory_path_exists() {
            let directory_structure = MissingWorkingDirectoryPath {};
            let error = "I failed because the directory you supplied does not exist. I can only initialise in a directory that already exists.";

            assert_eq!(Err(String::from(error)), create(&directory_structure));
        }

        #[test]
        fn errors_if_the_build_pipeline_directory_exists() {
            let directory_structure = BuildPipelineDirectoryExists {};
            let error = "I failed because the build_pipeline directory I wanted to create already exists. Maybe you've already initialised before?";

            assert_eq!(Err(String::from(error)), create(&directory_structure));
        }

        #[test]
        fn errors_if_creating_the_bp_directory_fails() {
            let directory_structure = CreateBPDirFails {};
            let error = "I failed because the build_pipeline directory I wanted to create already exists. Maybe you've already initialised before?";

            assert_eq!(Err(String::from(error)), create(&directory_structure));
        }

        #[test]
        fn errors_if_creating_the_scripts_directory_fails() {
            let directory_structure = CreateScriptDirFails {};
            let error = "I Failed because I couldn't create the build_pipeline/scripts directory";
            assert_eq!(Err(String::from(error)), create(&directory_structure));
        }

        #[test]
        fn errors_if_creating_the_config_file_fails() {
            let directory_structure = CreateConfigFileFails {};
            let error = "I failed because I could not create the build_pipeline/config.json file";

            assert_eq!(Err(String::from(error)), create(&directory_structure));
        }

        #[test]
        fn errors_if_writing_the_config_file_fails() {
            let directory_structure = WriteConfigFileFails {};
            let error = "I failed because I could not write to the build_pipeline/config.json file, which I have created but have left empty.";

            assert_eq!(Err(String::from(error)), create(&directory_structure));
        }
    }
}
