use std::{fs, path::Path, process::Command, io::Write};

use clap::Parser;

const GITIGNORE_URL: &str = "https://gist.githubusercontent.com/Yousha/3830712334ac30a90eb6041b932b68d7/raw/0b51bafac0aa74d8a3af8cdcbc68c205a1117bc6/.gitignore";

#[derive(Parser, Debug)]
struct Cli {
    name: String,
}

async fn get_gitignore() -> String {
    reqwest::get(GITIGNORE_URL).await.unwrap().text().await.unwrap()
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let directory = Path::new(&args.name);

    assert!(!directory.exists()); // if directoy exists, abort the program

    fs::create_dir_all(directory).unwrap(); // create the directory

    // initialize git
    Command::new("git")
        .arg("init")
        .current_dir(directory)
        .spawn().unwrap().wait().unwrap();

    // add .gitignore
    fs::File::create(directory.join(".gitignore")).unwrap()
        .write(get_gitignore().await.as_bytes()).unwrap();

    // create cmake file
    fs::File::create(directory.join("CMakeLists.txt")).unwrap().write(format!("
cmake_minimum_required(VERSION 3.0)
project({} LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 23)    
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable({} main.cpp)", &args.name, &args.name).as_bytes()).unwrap();

    // create main.cpp file
    fs::File::create(directory.join("main.cpp")).unwrap().write("
#include <iostream>

int main() {
    std::cout << \"Hello, world!\" << std::endl;
}".as_bytes()).unwrap();
}
