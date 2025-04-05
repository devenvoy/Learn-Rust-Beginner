# Learning Rust

## Basics

1. Install Rust 
    - we first going to see rust installation & set up
    ```
    // in mac 
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
    ```
    - ` rustc --version ` to check rust installtion 
    - ` cargo --version ` to check cargo installation

2. First Program 
    - print `Hello World!`
    ```rs
    fn main(){
        println!("Hello World!");
    }
    ```
    - `rustc main.rs` to compile code into binary 
    - [mac] `./main ` or [windows] `.\main.exe` to run binary 

3. First Project 
    - **`cargo`** is cli tool , package manager that can be used to create and manage rust project 
    - **`rustfmt`** is cli tool for rust code file formatter Ex: **`rustfmt <file_name>`**
    - **`cargo new <project_name>`** to create new Rust Project
    - **`cargo build`** just to compile, build the project 
    - **`cargo run`** to compile , build and run the project codes in `src` directory of project
    - **`cargo build --release`** it will create production ready optimized code for deploy application


### Biblo
- [Piyush garg]("https://www.youtube.com/watch?v=unRhxbFULII")