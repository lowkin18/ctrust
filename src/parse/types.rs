use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Cfile {
    name: String,
    functions: Option<Vec<Cfunction>>,
    class: Option<Vec<Cclass>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Cclass {
    name: String,
    pubmethod: Vec<Cfunction>,
    privmethod: Vec<Cfunction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Cfunction {
    name: String,
    args: Vec<Carg>,
    modifier: Option<Vec<Modifier>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Carg {
    name: String,
    var_type: String,
    var_modifier: Option<Vec<Modifier>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Modifier {
    Pointer,
    Const,
    Reference,
    Static,
}
