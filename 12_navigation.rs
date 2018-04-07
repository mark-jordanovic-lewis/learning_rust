use std::env;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use std::fs;
use std::io;

fn main() {
    // having a look to see if our cargo dir is in the std place:
    let home = env::home_dir().expect("There's no home dir!");
    let mut path = PathBuf::new();
    path.push(home);
    path.push(".cargo");
    if path.is_dir() {
        println!("cargo path is: {:?}", path.display());
    }
    // searching backwards for a file (.emacs) and displaying it's metadata
    let mut path = env::current_dir().expect("I'm not in a directory!");
    loop {
        path.push(".emacs");   // putting the file onto the `path` (string like object)
        if path.is_file() {
            match path.metadata() {
                Ok(data) => {
                    println!("found: {:?}", path.display());
                    println!("file type:   {:?}", data.file_type());
                    println!("size:        {:?} bytes", data.len());
                    println!("permissions: {:o}", data.permissions().mode());
                    println!("touched:     {:?}", data.modified());
                },
                Err(e) => println!("Error: {:?}", e)
            }
            break;
        } else {
            path.pop(); // popping the file name off the path
        }
        if !path.pop() { // popping a dir off the path
            break;
        }
    }
    ls_dir("./");
}
// you must return a `Result` if you are going to use `?`
fn ls_dir(dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? { // remember automatic unwrapping is naughty for production code!
        let entry = entry?;
        let data = entry.metadata()?;
        let path = entry.path();
        if data.is_file() {
            if let Some(ex) = path.extension() {
                if ex == "rs" {
                    println!("{} has length:   {} bytes", path.display(), data.len());
                }
            }
        }
    }
    Ok(())
}
