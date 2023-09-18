use crate::ctrust::types::{Carg, Cfunc, Cret, Modifier};
use crate::ctrust::*;
use crate::prelude::*;
use regex::Regex;

use async_recursion::async_recursion;

/// carg dealing with filtering function arguments
impl Cret {
    pub async fn new(text_arg: &str) -> Option<Cret> {
        


        
    }

    pub fn is_func(&self) -> Result<bool>{
        if self.base_string.contains("(") {
            //parse function
            return Ok(true);
        }
        if self.base_string.contains("function<") {
            //parse function
            
            return Ok(true);
        }
        Ok(false)
    }
}

#[cfg(test)]
mod test_carg_parse {
    use super::*;

    #[tokio::test]
    async fn test_is_function() {}

    #[tokio::test]
    async fn test_parse_stdfunction() {}
}
