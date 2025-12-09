use std::{self, path::Path};

pub fn run(config: impl FileReader) -> Result<(), Box<dyn std::error::Error>> {
    let contents = config.read_file();

    let ocurences = config.search(&contents)?;

    println!("Occurences {:?}", ocurences);
    Ok(())
}

pub trait FileReader {
    fn read_file(&self) -> String;
    fn search<'b>(&self, content: &'b str) -> Result<Vec<&'b str>, std::io::Error>; // I implement lifetime only on the method, instead of the whole trait
}

pub struct Config<'a> {
    pub argument: &'a String,
    pub file_path: Box<&'a Path>,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Config<'a>, &'a str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query_arg: &String = &args[1];
        let file_path_arg: &String = &args[2];
        let file_path= Box::new(Path::new(file_path_arg));

        // Check existence
        if !file_path.try_exists().map_err(|_| "File existence check failed")? {
            return Err("File does not exist");
        }

        let config = Config { argument: query_arg, file_path: file_path};
        Ok(config)
    }
}

impl<'a> FileReader for Config<'a> {
    fn read_file(&self) -> String {
        let try_read_file = std::fs::read_to_string(*self.file_path);

        let contents = match try_read_file {
            Ok(contents) => contents,
            Err(_e) => {
                eprintln!("Problem with reading file at the specified file path");
                std::process::exit(1);
            }
        };

        contents
    }

    fn search<'b>(&self, content: &'b str) -> Result<Vec<&'b str>, std::io::Error> {
        let mut results = Vec::new();

        for line in content.lines() {
            if line.contains(self.argument) {
                results.push(line);
            }
        };

        Ok(results)
    }
}