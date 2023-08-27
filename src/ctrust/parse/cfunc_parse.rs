use crate::ctrust::gen::interface_gen;
use crate::ctrust::types::{Carg, Cfunc, Cret, Modifier};
use crate::ctrust::*;
use crate::prelude::*;
use regex::Regex;

use async_recursion::async_recursion;

/// carg dealing with filtering function arguments
impl Carg {
    /// this function checks to see if the text string is a function referece/ptr
    pub async fn is_func(text_arg: &str) -> bool {
        if text_arg.contains("(") {
            return true;
        }
        if text_arg.contains("function<") {
            return true;
        }
        false
    }

    pub async fn parse_func_argument(text_arg: &str) -> Result<Option<Box<Cfunc>>> {
        let mut func_arg = Cfunc::default();

        if (text_arg.contains("function<")) {}

        Ok(None)
    }
    ///this func will parse any std::function input arguments used
    pub async fn parse_stdfunc_argument(text_arg: &str) -> Result<Option<Box<Cfunc>>> {
        let mut func = Cfunc::default();
        let input_output: Vec<&str> = text_arg
            .split(|c| c == '<' || c == '(' || c == ',' || c == '>' || c == ')')
            .collect();
        let mut input_output: Vec<&str> =
            input_output.into_iter().filter(|s| !s.is_empty()).collect();

        println!("{}", input_output.remove(0));
        let output_type = input_output.remove(0);
        let input_types = input_output;

        println!("{:?}", input_types);

        func.ret = Cret::new(output_type).await;
        let mut vec_carg: Vec<Carg> = Vec::new();
        for input_arg in input_types {
            if let Some(x) = Carg::new_arg(input_arg).await? {
                vec_carg.push(x);
            }
        }

        /// if the vector has some arguments populate array
        if (vec_carg.len() > 0) {
            func.args = Some(vec_carg);
        }

        Ok(Some(Box::new(func)))
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
