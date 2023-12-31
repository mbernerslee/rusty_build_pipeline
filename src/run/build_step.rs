use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BuildStep {
    pub build_step_name: String,
    pub command_type: String,
    pub command: String,
    pub depends_on: Vec<String>,
    pub env_vars: Option<Vec<EnvVar>>,
}
