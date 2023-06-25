# ls
Implementation of ls in rust with ui
Sure! Here's a detailed README for your project:

# Rust Directory Listing

This Rust project is a simple directory listing tool that displays the contents of a directory and provides information about each file and directory within it.

## Features

- List files and directories in a specified directory.
- Display the type of each item (file, directory, or symlink).
- Print the permissions (read, write, and execute) of each item.
- Accept a command-line argument to specify the directory to be listed.

## Prerequisites

To compile and run this project, you need to have Rust installed on your system (and cargo). If you don't have Rust installed, you can follow the official installation guide at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## SetUp

1. Build the project using Cargo:

   ```
   cargo build
   ```

   This command will compile the project and create the executable file.

2. Run the project:

   ```
   cargo run 
   ```
3. Use enter to enter a directory and the arrows up and down to select a directory

## Usage

Once the project is compiled and executed, it will display the contents of the specified directory. For each item, it will show the name, type (file, directory, or symlink), and permissions (read, write, and execute).


![Output:](../ls1.png)
## Acknowledgements

This project was created as a learning exercise in Rust programming. It utilizes the Rust standard library and various Rust features to achieve its functionality.
