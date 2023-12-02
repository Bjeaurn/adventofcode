pub mod file_utils {
    use std::env;
    use std::fs::File;
    use std::io::{self, Read};

    pub fn load_file(name: String) -> io::Result<String> {
        // println!("Hello, world!");
        // let name = "Test";
        // println!("Hello, {}!", name);
        if let Ok(current_dir) = env::current_dir() {
            println!("Current directory: {}", current_dir.display());
        } else {
            eprintln!("Failed to retrieve current directory");
        }
        let path = "./assets/".to_string() + &name;
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(e) => Err(e),
        }
    }
}
