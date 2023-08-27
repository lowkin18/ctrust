use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Cfile {
    pub name: String,
    pub dependency: Vec<String>,
    pub functions: Option<Vec<Cfunc>>,
    pub class: Option<Vec<Cclass>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Cclass {
    pub name: String,
    pub pubmethod: Option<Vec<Cfunc>>,
    pub privmethod: Option<Vec<Cfunc>>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]

pub struct Cfunc {
    pub name: String,
    pub args: Option<Vec<Carg>>,
    pub ret: Option<Cret>,
    pub modifier: Option<Vec<Modifier>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Carg {
    pub base_string: String,
    pub name: Option<String>,
    pub var_type: String,
    pub var_modifier: Option<Vec<Modifier>>,
    pub var_default: Option<String>,
    pub var_func: Option<Box<Cfunc>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Cret {
    pub var_type: String,
    pub var_modifier: Option<Vec<Modifier>>,
    pub var_func: Option<Box<Cfunc>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Modifier {
    Pointer,
    Reference,
    ConstType,
    ConstPtr,
    ConstRef,
    ConstReturn,
    ConstexprReturn,
    StaticFunc,
}
