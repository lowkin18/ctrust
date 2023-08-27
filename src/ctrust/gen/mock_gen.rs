use crate::ctrust::types::{Carg, Cclass, Cfile, Cfunc};
use crate::prelude::*;

impl Cfile {
    pub async fn generate_isolated_test(&self) -> Result<()> {
        //grab all files

        //create all mocks

        //create cmake

        //run cmake

        Ok(())
    }

    /// this function takes the Cfile object and returns the mocked string
    pub async fn create_mocks(&self) -> Result<()> {
        if self.functions.is_some() && self.class.is_some() {
            println!("cfile");
        } else if self.class.is_some() {
            println!("classes");
        } else if self.functions.is_some() {
            println!("functions");
        } else {
            println!("nothing found");
        }

        Ok(())
    }

    pub async fn move_files(&self) -> Result<()> {
        Ok(())
    }

    pub async fn create_cmake(&self) -> Result<()> {
        Ok(())
    }
}
