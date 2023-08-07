use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TestConfig {
    name: String,
}
