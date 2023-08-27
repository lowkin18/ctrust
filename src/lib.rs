#![allow(unused)]

use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::ctrust::types::{Carg, Cclass, Cfile, Cfunc};
use crate::ctrust::*;
use crate::prelude::*;
use crate::proj_type::*;

mod ctrust;
mod error;
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
    async fn strip_comments_test() {
        let mut file = File::open("./test_files/cfile.h").expect("Failed to open file");

        // Read the file contents into a String
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file contents");

        // Deserialize the YAML into the Config struct

        let newstring = Cfile::strip_comments(contents.as_str()).await;

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
        let newstring = Cfile::strip_comments(contents.as_str()).await;

        let cfile = Cfile::new(&newstring).await;

        println!("{:?}", cfile);
    }

    #[tokio::test]
    async fn create_cppfile() {
        let file_str = "./test_files/cppclass.h".to_owned();
        let mut file_snsr = "./test_files/combined_snsr.h".to_owned();

        let thread_join_handle = tokio::task::spawn(async move {
            // some work here
            let mut file = File::open(file_str).expect("Failed to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file contents");
            let newstring = Cfile::strip_comments(contents.as_str()).await;

            let cfile = Cfile::new(&newstring).await.unwrap();
            //println!("{:?}", cfile);
            let mut file = File::create("uncommented.cpp").unwrap();

            // Write the string to the file
            // file.write_all(newstring.as_bytes()).unwrap();
            // file.flush().unwrap();
        });

        let thread_two = tokio::task::spawn(async move {
            // some work here
            let mut file = File::open(file_snsr).expect("Failed to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file contents");
            let newstring = Cfile::strip_comments(contents.as_str()).await;

            let cfile = Cfile::new(&newstring).await.unwrap();
            //println!("{:?}", cfile);
            let mut file = File::create("uncommented.cpp").unwrap();

            // Write the string to the file
            file.write_all(newstring.as_bytes()).unwrap();
            file.flush().unwrap();
        });
        // some work here
        let res = thread_join_handle.await;
        let res = thread_two.await;

        // Optional: Flush the buffer to ensure the content is written immediately
    }

    #[tokio::test]
    async fn search_for_tests() {
        let mut file = File::open("ctrust_config.yaml").unwrap();
        let mut contents = String::new();

        // Read the contents of the file into a string
        file.read_to_string(&mut contents).unwrap();

        // Deserialize the YAML string into a struct
        let config: ProjectConfig = serde_yaml::from_str(&contents).unwrap();

        // Now you can access the values in the config struct
        println!("project name: {}", config.name);
        println!("project root path: {}", config.project_root_path);
        println!("folders: {:?}", config.folders);
        println!("test folders: {:?}", config.test_folders);
        println!("support folders: {:?}", config.support_folders);
        println!("output path: {}", config.output_path);
        println!("compiler path: {}", config.compiler_path);
    }

    #[tokio::test]
    async fn find_include_list() {
        assert!(true);
    }
    #[tokio::test]
    async fn move_test_files() {
        assert!(true);
    }

    #[tokio::test]
    async fn create_cmake_file() {
        assert!(true);
    }
}
