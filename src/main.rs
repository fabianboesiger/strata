use std::{
    path::{
        Path,
        PathBuf
    },
    env,
    process::exit,
    collections::HashMap
};

mod operator;

struct Settings {
    input: PathBuf,
    output: PathBuf
}

fn main() {
    let mut args = env::args();

    let _working_dir = args.next().unwrap();
    let map: HashMap<String, String> = args.map(|arg| {
        let mut split = arg.split('=');
        let key = split.next().unwrap_or_else(|| {
            eprintln!("Invalid format \"{}\"", arg);
            exit(1);
        });
        let value = split.next().unwrap_or_else(|| {
            eprintln!("No value found for field \"{key}\", use the format \"{key}=value\".", key=key);
            exit(1);
        });
        (String::from(key), String::from(value))
    }).collect();

    let settings = Settings {
        input: {
            let path = Path::new(map.get("in").unwrap_or_else(|| {
                eprintln!("Required field \"in\" not found.");
                exit(1);
            }));
            if !path.is_dir() {
                eprintln!("Path \"{}\" does not point to a directory.", path.display());
                exit(1);
            }
            path.to_path_buf()
        },
        output: {
            let path = Path::new(map.get("out").unwrap_or_else(|| {
                eprintln!("Required field \"out\" not found.");
                exit(1);
            }));
            if path.exists() {
                eprintln!("Output file \"{}\" already exists.", path.display());
                exit(1);
            }
            path.to_path_buf()
        }
    };
}