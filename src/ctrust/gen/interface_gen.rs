use crate::prelude::*;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Write};

use crate::ctrust::types::{Carg, Cclass, Cfile, Cfunc};

static FORMAT_OFF: &str = "// clang-format off";
static FORMAT_ON: &str = "// clang-format on";
static GEN_STRING_START: &str = "/* generated interface code */";
static GEN_STRING_END: &str = "/* generated interface code end */";

impl Cfile {
    pub fn remove_generated(&self) -> Result<()> {
        Ok(())
    }
}
