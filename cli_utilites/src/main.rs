#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use std::fs;

#[derive(Debug)]
enum Config {
    Echo {
        text: String,
    },
    Cat {
        path: String,
        path2: Option<String>,
        output: Option<String>,
    },
    Ls {
        path: String,
    },
    Find {
        path: String,
        query: String,
    },
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Arguments not enough")
        }
        match args[1].as_str() {
            "echo" => Config::Echo {
                text: args[2].to_string(),
            },
            "cat" => {
                let mut path2 = None;
                let mut output = None;
                if args.len() > 4 {
                    path2 = Some(args[3].to_string());
                    output = Some(args[4].to_string());
                } else if args.len() > 3 {
                    path2 = Some(args[3].to_string());
                }

                return Config::Cat {
                    path: (args[2].to_string()),
                    path2,
                    output,
                };
            }
            "ls" => Config::Ls {
                path: args[2].to_string(),
            },
            "find" => {
                if args.len() < 4 {
                    panic!("Where is the query ma man");
                } else {
                    Config::Find {
                        path: args[2].to_string(),
                        query: args[3].to_string(),
                    }
                }
            }
            _ => panic!("Unknown method"),
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args);

    run(config)
    // println!("{:?}" , config)
}

fn run(config: Config) {
    match config {
        Config::Cat {
            path,
            path2,
            output,
        } => {
            let content = match fs::read_to_string(path) {
                Err(_) => panic!("couldn't read the file"),
                Ok(data) => data,
            };

            let content2 = match path2 {
                None => String::from(""),
                Some(data) => match fs::read_to_string(data) {
                    Err(_) => panic!("couldn't read the second file"),
                    Ok(data) => data,
                },
            };

            let all = format!("{}\n{}", content, content2);

            match output {
                None => (),
                Some(data) => {
                    let _ = fs::write(data, &all);
                }
            }

            println!("{}", all)
        }
        Config::Echo { text } => {
            println!("{text}");
        }
        Config::Ls { path } => {
            println!("path: {path}");

            // let path = "./";

            let content_iter = match fs::read_dir(&path) {
                Ok(iter) => iter,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            for item in content_iter {
                match item {
                    Ok(data) => {
                        println!("{}", data.path().as_os_str().to_str().unwrap());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
        Config::Find { path, query } => {
            // let iter = fs::read_dir(&path).unwrap();

            let vec: Vec<String> = get_all_files_inside_dir(&path);

            let matches = search(&vec , &query);

            println!("{:?}", matches)
        }
    }
}

fn search<'a>(vec: &'a [String] , query: &str) -> Vec<&'a String> {
    vec.iter().filter(|item| {
        item.contains(query)
    }).collect()
}

fn get_all_files_inside_dir(path: &str) -> Vec<String> {
    let mut output = vec![];
    let content_iter = match fs::read_dir(&path) {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("can't read the path");
        }
    };

    for item in content_iter {
        match item {
            Ok(data) => {
                let safe_path = data.path().to_string_lossy().to_string().replace("\\" , "/");

                // println!("{} is a dir : {}" , &safe_path , data.metadata().unwrap().is_dir());

                if data.metadata().unwrap().is_dir() {
                    output.extend(get_all_files_inside_dir(&safe_path));
                }
                output.push(safe_path);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    return output;
}
