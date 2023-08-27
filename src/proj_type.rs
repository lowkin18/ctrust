use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub project_root_path: String,
    pub folders: Vec<String>,
    pub test_folders: Vec<String>,
    pub support_folders: Vec<String>,
    pub output_path: String,
    pub compiler_path: String,
}
