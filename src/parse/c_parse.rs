use crate::parse::types::{Carg, Cclass, Cfile, Cfunction};
use crate::parse::*;
use crate::prelude::*;

use regex::Regex;

impl Cfile {
    pub fn new(filetext: &str) -> Result<Cfile> {
        let mut c_file = Cfile::default();

        //regex functions out

        let class_string = c_file.find_class_substring(filetext);
        if (class_string.len() > 0) {
            c_file.get_class();
        }

        c_file.regex_functions(filetext);

        Ok(c_file)
    }

    pub fn find_class(&mut self, classes: Vec<String>) -> Result<Self> {}

    pub fn strip_comments(input: &str) -> String {
        let mut output = String::new();
        let mut in_line_comment = false;
        let mut in_block_comment = false;
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if in_line_comment {
                if ch == '\n' {
                    in_line_comment = false;
                    output.push('\n');
                }
            } else if in_block_comment {
                if ch == '*' && chars.peek() == Some(&'/') {
                    in_block_comment = false;
                    chars.next();
                }
            } else if ch == '/' {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '/' {
                        in_line_comment = true;
                        chars.next();
                    } else if next_ch == '*' {
                        in_block_comment = true;
                        chars.next();
                    } else {
                        output.push(ch);
                    }
                } else {
                    output.push(ch);
                }
            } else {
                output.push(ch);
            }
        }
        output
    }

    fn find_class_substring(&self, file_string: &str) -> Vec<String> {
        let mut offset = 0;
        let mut class_string = String::new();
        let mut bracket_count = 0;
        let mut found_class = false;

        let mut class_strings: Vec<String> = Vec::new();
        let re = Regex::new(r"class(?:[^;]*)(?:[^}])*};").unwrap();
        for cap in re.captures_iter(&file_string) {
            class_strings.push(cap.get(0).unwrap().as_str().to_string());
        }
        class_strings
    }

    fn regex_functions(&mut self, file_string: &str) -> Result<&Self> {
        let re = Regex::new(r"([^\n\r\=]*)(\s+\w+\n*\r*)(\([^{};]*\))(\s*\w+)*").unwrap();

        // Iterate over each match and add the function name to the result vector
        //let mut result = Vec::new();
        println!("{}", file_string);
        for cap in re.captures_iter(&file_string) {
            let return_var = cap.get(1).unwrap().as_str().to_string();
            let function_name = cap.get(2).unwrap().as_str().to_string();
            let function_args = cap.get(3).unwrap().as_str().to_string();
            println!("{:?}", return_var);
            println!("{:?}", function_name);
            println!("{:?}", function_args);
        }
        Ok(self)
    }

    pub fn get_class(&self) -> Result<Cclass> {
        todo!()
    }

    pub fn get_functions(&self) -> Result<Vec<Cfunction>> {
        todo!()
    }
}

impl Cclass {}

impl Cfunction {}

impl Carg {}

//(:\n\r)*(\w+\s+)*(\w+\n*)\(([^.{}:;]*)\)(\s*\w+)*   function regex
