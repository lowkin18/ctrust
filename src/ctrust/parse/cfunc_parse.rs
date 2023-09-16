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
        let mut test_one = Carg::new_arg("int16_t arg").await.unwrap().unwrap();
        assert!(!test_one.is_func().await.unwrap());


        let mut test_two = Carg::new_arg("void(*func)(int,int)").await.unwrap().unwrap();
        assert!(test_two.is_func().await.unwrap());

        
        let mut test_two = Carg::new_arg("std::function<int,<int,int>").await.unwrap().unwrap();
        assert!(test_two.is_func().await.unwrap());
    }
    #[tokio::test]
    async fn test_parse_stdfunction() {
        ///test not a function
        let mut test_one = Carg::new_arg("std::function<int,<int,int> func").await.unwrap().unwrap();
        test_one.parse_stdfunc_argument().await;
        println!("{:?}", test_one);
         
        let mut test_two = Carg::new_arg("std::function<int<std::function<int<int,int>>,int>> func").await.unwrap().unwrap(); 
        test_two.parse_stdfunc_argument().await;
        println!("{:?}", test_two);
    }

    #[tokio::test]
    async fn test_parse_function_ptr() {}
}
