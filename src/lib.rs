#![allow(unused)]

use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::parse::c_parse::*;
use crate::parse::types::{Carg, Cclass, Cfile, Cfunction};
use crate::prelude::*;
use crate::proj_type::*;

mod error;
mod parse;
mod prelude;
mod proj_type;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        assert_eq!(1, 1);
    }
    #[tokio::test]
    async fn find_file() {
        assert_eq!(1, 1);
    }
    #[tokio::test]
    async fn read_yaml_project() {
        let mut file = File::open("./test_files/config.yaml").expect("Failed to open file");

        // Read the file contents into a String
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file contents");

        // Deserialize the YAML into the Config struct
        let config: TestConfig = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

        // Now you can work with the parsed YAML data in the `config` variable
        println!("{:?}", config);
    }

    #[tokio::test]
    async fn strip_comments_test() {
        let mut file = File::open("./test_files/cfile.h").expect("Failed to open file");

        // Read the file contents into a String
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file contents");

        // Deserialize the YAML into the Config struct

        let newstring = Cfile::strip_comments(contents.as_str());

        // Now you can work with the parsed YAML data in the `config` variable
        println!("{:?}", newstring);
        // Create or open the file for writing
        let mut file = File::create("uncommented.c").unwrap();

        // Write the string to the file
        file.write_all(newstring.as_bytes()).unwrap();

        // Optional: Flush the buffer to ensure the content is written immediately
        file.flush().unwrap();
    }

    #[tokio::test]
    async fn create_cfile() {
        let mut file = File::open("./test_files/cfile.h").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file contents");
        let newstring = Cfile::strip_comments(contents.as_str());

        let cfile = Cfile::new(&newstring).unwrap();
        println!("{:?}", cfile);
    }

    #[tokio::test]
    async fn create_cppfile() {
        let mut file = File::open("./test_files/cppclass.h").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file contents");
        let newstring = Cfile::strip_comments(contents.as_str());

        let cfile = Cfile::new(&newstring).unwrap();
        //println!("{:?}", cfile);
        let mut file = File::create("uncommented.cpp").unwrap();

        // Write the string to the file
        file.write_all(newstring.as_bytes()).unwrap();

        // Optional: Flush the buffer to ensure the content is written immediately
        file.flush().unwrap();
    }
}
