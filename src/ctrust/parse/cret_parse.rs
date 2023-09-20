use crate::ctrust::types::{Carg, Cfunc, Cret, Modifier};
use crate::ctrust::*;
use crate::prelude::*;
use regex::Regex;

use async_recursion::async_recursion;

/// carg dealing with filtering function arguments
impl Cret {
    pub async fn new(text_arg: &str) -> Result<Option<Cret>> {

        let mut cret = Cret::default(); 
        cret.base_string = text_arg.to_owned();
        
        if (cret.is_func_ptr().await)
        {
            //parse function poitner
            cret.parse_func_ptr().await?;
        } 
        else if (cret.is_std_func().await)
        {
            //parse std function
        }
        else {
            //parse regular return
        }
        
        Ok(Some(cret))
    }

    pub async fn is_func_ptr(&self) -> bool{
        if self.base_string.contains("(") {
            return true;
        }
        return false;
    }
    pub async fn is_std_func(&self) -> bool
    {
        if self.base_string.contains("function<") {
            return true;
        }
        false
    }
    
    #[async_recursion]
    pub async fn parse_func_ptr(&mut self) ->Result<&mut Self>{
        let base_str = &self.base_string;

        let mut input_output: Vec<&str> = base_str.split("(").collect();
        
        if(input_output.len()<=1)
        {
            return Err(Error::Generic( format!("return function could not be parsed {}",&self.base_string)))
        } 
        else if(input_output.len()<=2)
        {
            
        }
        else {
            //function ptr that returns a function ptr
            let return_param = input_output.remove(0);
            let ret_string = return_param.to_owned() + input_output.pop().as_deref().unwrap();

            //recursive call to parse another function pointer
            self.var_func = Some(Cfunc::new_ptr_func(ret_string.as_str()).await?);
            self.var_type = input_output.remove(0);
            self
            

            
        }
        
        Ok(self)
    }
 
    pub fn parse_return(&self) -> Result<&Self>
    {
           
        Ok(self)
    }  
}

#[cfg(test)]
mod test_carg_parse {
    use super::*;

    #[tokio::test]
    async fn test_is_function() {
        print!("testing is function ");
    }

    #[tokio::test]
    async fn test_parse_stdfunction() {
        let test_string = "void (*func(float a, floatb))(int x, int y)".to_owned(); 
        println!("testing this function string {}", &test_string);
        let mut test_one = Cret::new(&test_string).await.unwrap().unwrap();
        println!("{:?}", &test_one.base_string); 
        assert!(true);
    } 
}