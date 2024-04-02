use std::env;
use std::path::Path;

pub fn cd(args: Vec<String>) {
    let path = Path::new(args[0].as_str());

    let result = env::set_current_dir(&path);

    if !result.is_ok() {
        eprintln!(
            "Error occured changing dirs: {} is not a dir",
            path.display()
        );
    }
}
