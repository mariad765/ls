use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
pub fn run() {
    
    // use std::env to print the program's first argument
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is: {}", args[1]);
    } else {
        println!("No arguments provided.");
    }
    

    //  - select the directory to be listed
    //  - if the first argument exists, that is the directory
    //  - if it does not, the current directory will be listed

    let directory = env::args().nth(1).unwrap_or(".".to_string());
    println!("Listing files in directory: {}", directory);

    let dir = fs::read_dir(directory.clone()).unwrap();
    let mut files = Vec::new();
    for entry in dir {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                files.push(String::from(file_name));
            }
        }
    }

    //  - print the contents of the directory
  
    // - print the type of each item (dir, file, link)
    

    for entry in fs::read_dir(directory.clone()).unwrap() {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                let file_path = entry.path();
                let metadata = fs::metadata(file_path.clone()).unwrap();

                let file_type = if metadata.is_dir() {
                    "directory"
                } else if metadata.is_file() {
                    "file"
                } else if metadata.file_type().is_symlink() {
                    "symlink"
                } else {
                    "unknown"
                };

                let mode = metadata.permissions().mode();
                let readable = mode & 0o444 != 0;
                let writable = mode & 0o222 != 0;
                let executable = (mode & 0o0010) > 0;
                let forread = if readable { "r" } else { "-" };
                let forwrite = if writable { "w" } else { "-" };
                let forexecute = if executable{ "x" } else { "-" };
                println!(
                    "{} - {} - {}{}{}",
                    file_name,
                    file_type,
                    forread,
                    forwrite,
                    forexecute
                );
        }
    }
}

    // - print the properties of each directory / file
    // use mode

}
