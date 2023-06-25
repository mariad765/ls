use std::env;
// recreate ls
mod ls;

// create a text ui ls
mod ui;


fn main() {

   ls::run();
   // Initialize a boolean variable 'ok' and set it to true
   let mut ok:bool=true;
   // Get the current directory and assign it to the 'current_dir' variable
   let current_dir = env::current_dir().unwrap();
   // Call the 'run' function from the 'ui' module, passing the 'current_dir' and 'ok' variables
   ui::run(current_dir,ok);
}
