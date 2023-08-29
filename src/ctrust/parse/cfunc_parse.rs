use crate::ctrust::gen::interface_gen;
use crate::ctrust::types::{Carg, Cfunc, Cret, Modifier};
use crate::ctrust::*;
use crate::prelude::*;
use regex::Regex;

use async_recursion::async_recursion;

/// carg dealing with filtering function arguments
impl Carg {
    /// this function checks to see if the text string is a function referece/ptr
    pub async fn is_func(&mut self) -> Result<bool> {
        if self.base_string.contains("(") {
            //parse function
            self.parse_func_ptr();

            return Ok(true);
        }
        if self.base_string.contains("function<") {
            //parse function

            return Ok(true);
        }
        Ok(false)
    }

    pub async fn parse_func_ptr(&mut self) -> Result<()> {
        let mut func_arg = Cfunc::default();

        Ok(())
    }
    ///this func will parse any std::function input arguments used
    pub async fn parse_stdfunc_argument(&mut self) -> Result<()> {
        let mut func_arg = Cfunc::default();
        let input_output: Vec<&str> = self
            .base_string
            .split(|c| c == '<' || c == '(' || c == ',' || c == '>' || c == ')')
            .collect();
        let mut input_output: Vec<&str> =
            input_output.into_iter().filter(|s| !s.is_empty()).collect();

        println!("{}", input_output.remove(0));
        let output_type = input_output.remove(0);
        let input_types = input_output;

        println!("{:?}", input_types);

        func_arg.ret = Cret::new(output_type).await;
        let mut vec_carg: Vec<Carg> = Vec::new();
        for input_arg in input_types {
            if let Some(x) = Carg::new_arg(input_arg).await? {
                vec_carg.push(x);
            }
        }

        /// if the vector has some arguments populate array
        if (vec_carg.len() > 0) {
            func_arg.args = Some(vec_carg);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_carg_parse {
    use super::*;

    #[tokio::test]
    async fn test_is_function() {
        //test not a function
        assert!(!Carg::is_func("int16_t arg").await);

        assert!(Carg::is_func("void(*func)(int,int)").await);
        assert!(Carg::is_func("std::function<int,<int,int>").await);
    }
    #[tokio::test]
    async fn test_parse_stdfunction() {
        //test not a function
        let value = Carg::parse_stdfunc_argument("std::function<int,<int,int> func").await;
        // println!("{:?}", value);
        let value = Carg::parse_stdfunc_argument(
            "std::function<int<std::function<int<int,int>>,int>> func",
        )
        .await;
        // println!("{:?}", value);
    }

    #[tokio::test]
    async fn test_parse_function_ptr() {}
}
