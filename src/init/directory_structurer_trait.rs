use std::io::Result;

trait DirectoryStructurer {

    fn working_directory_path_exists(&self) -> bool
    fn build_pipeline_directory_exists(&self) -> bool

    fn create_build_pipeline_directory(&self) -> Result<()>
    fn create_scripts_directory(&self) -> Result<()>
    fn create_empty_config_file(&self) -> Result<File>
    fn write_to_config_file(Result<File>, &'static [u8]) -> Result<()>
}
