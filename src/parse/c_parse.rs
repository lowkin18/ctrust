use crate::parse::types::{Carg, Cclass, Cfile, Cfunction};
use crate::parse::*;
use crate::prelude::*;

use regex::Regex;

impl Cfile {
    pub fn new(filetext: &str) -> Result<Cfile> {
        let mut c_file = Cfile::default();

        //regex functions out

        let class_strings: Vec<String> = c_file.find_class_substring(filetext);
        if (class_strings.len() > 0) {
            c_file.find_class(class_strings);
        }

        c_file.regex_functions(filetext);

        Ok(c_file)
    }

    fn find_class(&mut self, classes: Vec<String>) -> Result<Self> {
        for class in classes {
            self.regex_class_functions(&class);
        }
        todo!();
    }

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

    fn regex_class_functions(&mut self, file_string: &str) -> Result<&Self> {
        let re = Regex::new(r"([^\n\r\=]*)(\s+\w+\n*\r*)(\([^{};]*\))(\s*\w+)*").unwrap();

        //split string between public: private:
        let prv_str_start = file_string.find("private:").unwrap_or(0);
        let pub_str_start = file_string.find("public:").unwrap_or(0);

        let mut pub_str_end = file_string.len() - 1;
        let mut prv_str_end = file_string.len() - 1;

        if pub_str_start == 0 {
            pub_str_end = 0;
        } else if pub_str_start < prv_str_start && prv_str_start != 0 {
            pub_str_end = prv_str_start - 1;
        }

        let pub_string = &file_string[pub_str_start..pub_str_end];
        let private_string = &file_string[prv_str_start..prv_str_end];

        println!("{}", pub_string);
        for cap in re.captures_iter(pub_string) {
            let return_var = cap.get(1).unwrap().as_str().to_string();
            let function_name = cap.get(2).unwrap().as_str().to_string();
            let function_args = cap.get(3).unwrap().as_str().to_string();
            println!("{:?}", return_var);
            println!("{:?}", function_name);
            println!("{:?}", function_args);
        }
        println!("{}", private_string);
        for cap in re.captures_iter(private_string) {
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

impl Cclass {
    pub fn new(text: &str) -> Result<Cclass> {
        let mut class = Cclass::default();

        Ok(class)
    }
}

impl Cfunction {
    pub fn new(text: &str) -> Result<Cclass> {
        let mut class = Cclass::default();

        Ok(class)
    }
}

impl Carg {
    pub fn new(text: &str) -> Result<Cclass> {
        let mut class = Cclass::default();

        Ok(class)
    }
}

//(:\n\r)*(\w+\s+)*(\w+\n*)\(([^.{}:;]*)\)(\s*\w+)*   function regex
