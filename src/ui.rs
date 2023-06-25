// Import necessary modules and structs from the `cursive` and `std` libraries.
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView, Button};
use cursive::Cursive;
use cursive::traits::{Nameable};
use cursive::CursiveExt;
use cursive::align::{HAlign, VAlign};
use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
    sync::Arc,
};

use std::os::unix::fs::PermissionsExt;

pub fn run(current_dir: PathBuf, mut ok:bool) {

    // Create a new Cursive instance
    let mut siv = Cursive::default();

    // Get the current working directory
    let current_dir_str = current_dir.to_string_lossy();
    let dir = fs::read_dir(current_dir.clone()).unwrap();
    
   // List the files in the directory
   let mut layout = LinearLayout::vertical()
   .child(TextView::new(format!("Current directory - {}", current_dir_str)));

    // Set the dialog title to the current directory
    let mut files = Vec::new();
    for entry in dir{ 
        if let Ok(entry) = entry {
        if let Some(file_name) = entry.file_name().to_str() {
            files.push(String::from(file_name));
        }
        }
    }
    if ok==true{
         // Add an "Exit" button if `ok` is true
        layout.add_child(Button::new("Exit", |s| s.quit()));
        ok =false;}
   
    //.............................................................................

// Create buttons for navigating to the parent directory
let cloned_dir1 = Arc::new(current_dir.clone());
let parent_dir = current_dir.join("..");
let cloned_dir2 = Arc::clone(&cloned_dir1);
let button_prev = Button::new("..", move |s| {

    // Set the current directory to the parent directory
    env::set_current_dir(&cloned_dir2.parent().unwrap()).unwrap();
    s.pop_layer();
    // Run the `run` function recursively with the new directory
    run(parent_dir.to_path_buf(),ok);

});

layout.add_child(button_prev);
//.............................................................................

    // Iterate over the directory entries
    for entry in fs::read_dir(current_dir.clone()).unwrap() {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().into_string().ok() {
                let file_path = entry.path();
                let metadata = fs::metadata(file_path.clone()).unwrap();
                
                // Determine the file type based on metadata
                let file_type = if metadata.is_dir() {
                    "directory"
                } else if metadata.is_file() {
                    "file"
                } else if metadata.file_type().is_symlink() {
                    "symlink"
                } else {
                    "unknown"
                };
                // Extract file permissions
                let mode = metadata.permissions().mode();
                let readable = mode & 0o444 != 0;
                let writable = mode & 0o222 != 0;
                let executable = (mode & 0o0010) > 0;
                let forread = if readable { "r" } else { "-" };
                let forwrite = if writable { "w" } else { "-" };
                let forexecute = if executable{ "x" } else { "-" };


	
    let cloned_dir = current_dir.clone();
    let cloned_file_name = file_name.clone();
    if file_type=="directory"{
        let cloned_dir = current_dir.clone();
                // Create a button for directories to navigate into them
                 let button = Button::new(format!("{}-{}-{}{}{}",file_name,file_type,forread, forwrite, forexecute),move|s| {  if file_type == "directory" {
                    // Set the current directory to the selected directory
                    let new_dir = cloned_dir.join(&cloned_file_name);
                    env::set_current_dir(&new_dir).unwrap();
                    s.pop_layer();
                    // Run the `run` function recursively with the new directory
                    run(new_dir,ok);    
                
                }});
                 
                 layout.add_child(button);
                 

               
                };
    if file_type=="file"{ 
         // Create a TextView to display details for regular files
        let text2=TextView::new(format!("{}-{}-{}{}{}",file_name, file_type, forread, forwrite, forexecute));
                layout.add_child(text2.h_align(HAlign::Center).v_align(VAlign::Center));}
        // Create a TextView to display details for symlinks
    if file_type=="symlink"{ let text2=TextView::new(format!("{}-{}-{}{}{}",file_name, file_type, forread, forwrite, forexecute)).v_align(VAlign::Center).h_align(HAlign::Center);
                layout.add_child(text2.h_align(HAlign::Center).v_align(VAlign::Center));}
                
   
      
    }}}
   
   siv.add_layer(layout);
     
   // Start the event loop
   siv.run();
   
}