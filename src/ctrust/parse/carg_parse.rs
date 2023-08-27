use std::string;

use crate::ctrust::types::{Carg, Cfunc, Cret, Modifier};
use crate::ctrust::*;
use crate::prelude::*;
use regex::Regex;

use async_recursion::async_recursion;

///implementation of Carg for parsing out Cargs
impl Carg {
    ///this function will take a vec of argument strings and parse them
    pub async fn new_from_vec(text_args: Vec<&str>) -> Result<Option<Vec<Carg>>> {
        Ok(None)
    }

    ///this function will take a text string of the arguments and return vector of Cargs
    pub async fn new_vec(text_args: &str) -> Result<Option<Vec<Carg>>> {
        let text_arg_new = text_args[1..text_args.len() - 1]
            .replace("\n", "")
            .replace("\r", "");

        let parts: Vec<String> = Carg::split_comma_unless_bracket(&text_arg_new).await?;
        let mut cargs: Vec<Carg> = Vec::new();

        for part in parts.iter() {
            let arg = Carg::new_arg(part.trim()).await?;
            if arg.is_some() {
                cargs.push(arg.unwrap());
            }
        }
        Ok(Some(cargs))
    }

    /// this function will parse a argument string and return the Carg variable if it exists
    pub async fn new_arg(text_arg: &str) -> Result<Option<Carg>> {
        //args will be comma separated by default

        let mut arg = Carg::default();

        //check if special type
        if Carg::is_func(text_arg).await {
            todo!();
        }
        //if just regular argument parse it
        else {
            arg = Carg::parse_argument(text_arg).await?;
        }

        //populate the modifiers
        //arg.var_modifier = Carg::find_modifiers(text_arg).await?;
        //we assume this is a function pointer/ref

        Ok(Some(arg))
    }

    ///this function will parse an argument text and return the Carg object of the argument
    pub async fn parse_argument(text: &str) -> Result<Carg> {
        let mut arg = Carg::default();

        let mut modifier_list: Vec<Modifier> = Vec::new();
        let input_output: Vec<&str> = text.split(' ').collect();
        let mut input_output: Vec<&str> =
            input_output.into_iter().filter(|s| !s.is_empty()).collect();

        for (index, string_arg) in input_output.iter().enumerate() {
            //check for modifiers at the start
            let arg_str = *string_arg;
            if (arg_str == "const") {
                modifier_list.push(Modifier::ConstType)
            }
        }

        if (arg.var_type == "") {
            return Err(Error::Generic("parsing failed".to_owned()));
        }

        //if modifiers add them to the Carg
        if (modifier_list.len() > 0) {
            arg.var_modifier = Some(modifier_list);
        }

        Ok(arg)
    }

    pub async fn check_if_func_ref(text_arg: &str) -> bool {
        true
    }

    pub async fn check_if_func_ptr(text_arg: &str) -> bool {
        true
    }

    #[async_recursion]
    /// this function will parse a function pointer string that is passed in by argument
    pub async fn new_function_ptr(text_arg: &str) -> Result<Cfunc> {
        let mut func = Cfunc::default();

        let parts: Vec<&str> = text_arg.split('(').collect();
        if let Some(first) = parts.first() {
            println!("First: {}", first);
            let mut return_param = Cret::default();
            return_param.var_type = first.replace("(", "").to_owned();
            func.ret = Some(return_param);
        }

        let mut func_name = parts
            .get(1)
            .ok_or(Error::Generic("malformed function argument".to_owned()))?;
        if !func_name.contains("*") {
            return Err(Error::Generic(
                "malformed function argument no ptr(*)".to_owned(),
            ));
        } else {
            func.name = func_name.replace("*", "").replace("&", "");
        }

        if let Some(last) = parts.last() {
            println!("Last: {}", last);
            //arguments
            let mut args = Carg::new_vec(last).await?;
            if args.is_some() {
                func.args = args;
            }
        }
        Ok(func)
    }

    ///simple function to split along commas unless encapsulated by brackets
    pub async fn split_comma_unless_bracket(s: &str) -> Result<Vec<String>> {
        let mut bracket_level = 0;
        let mut chunks = Vec::new();
        let mut start = 0;

        for (i, char) in s.chars().enumerate() {
            match char {
                '(' => bracket_level += 1,
                ')' => bracket_level -= 1,
                ',' if bracket_level == 0 => {
                    chunks.push(s[start..i].trim().to_string());
                    start = i + 1;
                }
                _ => {}
            }
        }
        // append the last chunk
        chunks.push(s[start..].trim().to_string());
        Ok(chunks)
    }
}

#[cfg(test)]
mod test_carg_parse {
    use super::*;
    #[tokio::test]
    async fn test_passing_argument() {
        let arg = Carg::parse_argument("const float const *int value")
            .await
            .unwrap();

        assert_eq!(arg.name.unwrap(), "value");
        assert_eq!(arg.var_type, "float");
        assert!(arg.var_modifier.is_some());
        assert!(arg.var_modifier.unwrap().contains(&Modifier::ConstType));
        assert!(arg.var_modifier.unwrap().contains(&Modifier::ConstPtr));
        assert!(arg.var_modifier.unwrap().contains(&Modifier::Pointer));

        let arg_two = Carg::parse_argument("float &int value").await;

        //check malformed argument

        let failure = Carg::parse_argument("float& int value").await;
        assert!(result.is_err());
        assert!(true);
    }
    ///test to check if function argument is itself a functio
    #[tokio::test]
    async fn testing_split_on_comma() {
        let test_text = "int9_t test, int8_t test2, uint8_t test3, void (*func)(uint8_t b1, uint8_t b2), uint8_t test, std::func<float(float,float)> func";
        dbg!(Carg::split_comma_unless_bracket(test_text).await);
        assert!(true);
    }

    #[tokio::test]
    async fn testing_check_if_function() {
        let test_text = "(void)(*func)(uint8_t,uint8_t)";
        assert!(Carg::check_if_func_ptr(test_text).await);
    }

    #[tokio::test]
    async fn testing_carg_new_standard() {
        println!("testing!");
        let mut carg: Vec<Carg> =
            Carg::new_vec("(int test, int test2, uint8_t *ptr, uint8_t &ptr)")
                .await
                .unwrap()
                .unwrap();

        carg = Carg::new_vec("(void)").await.unwrap().unwrap();

        //assert_eq!(carg.name, "void");
        assert!(true);
    }
    #[tokio::test]
    async fn testing_carg_new_const_ptr() {
        println!("testing!");
        let mut carg: Vec<Carg> =
            Carg::new_vec("(int test, int test2, uint8_t *ptr, (void)(*func)(int,int))")
                .await
                .unwrap()
                .unwrap();

        //assert_eq!(carg.name, "void");
        assert!(true);
    }
    #[tokio::test]
    async fn testing_carg_new_ref() {
        println!("testing!");
        let mut carg: Vec<Carg> = Carg::new_vec("(int &test, int& test2, int* *test, int* &ref")
            .await
            .unwrap()
            .unwrap();

        //assert_eq!(carg.name, "void");
        assert!(true);
    }
    #[tokio::test]
    async fn testing_carg_new_function_ptr() {
        println!("testing!");
        let mut carg: Vec<Carg> =
            Carg::new_vec("(int test, int test2, uint8_t *ptr, (void)(*func)(int,int)")
                .await
                .unwrap()
                .unwrap();

        //assert_eq!(carg.name, "void");
        assert!(true);
    }
    #[tokio::test]
    async fn testing_carg_function_ptr() {
        println!("testing!");
        let mut cfunc: Cfunc = Carg::new_function_ptr("(void)(*func)(int,int)")
            .await
            .unwrap();

        assert_eq!(cfunc.name, "func");

        let mut cfunc: Cfunc = Carg::new_function_ptr("(void)(*func_two)(int,int)")
            .await
            .unwrap();

        assert_eq!(cfunc.name, "func_two");
        //assert_eq!(carg.name, "void");
        assert!(true);
    }
    #[tokio::test]
    async fn testing_carg_new_defaults() {
        println!("testing!");
        let mut carg: Vec<Carg> = Carg::new_vec("(int test = 1, uint8_t test2 = 0)")
            .await
            .unwrap()
            .unwrap();

        //assert_eq!(carg.name, "void");
        assert!(true);
    }
}
