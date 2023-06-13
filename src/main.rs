use anyhow::Error;
use flate2::read::ZlibDecoder;
use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    git().unwrap();
}

fn git() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory")
        }

        "cat-file" => {
            let blob_sha = args[3].clone();
            let dir_name = &blob_sha[0..2];
            let file_name = &blob_sha[2..];
            let contents = fs::read(format!(".git/objects/{}/{}", dir_name, file_name))?;

            let mut z = ZlibDecoder::new(&contents[..]);
            let mut contents = String::new();
            z.read_to_string(&mut contents)?;
            let parts = contents.split('\0').collect::<Vec<_>>();
            let object_type = parts[0].split(' ').collect::<Vec<_>>()[0];
            let content = parts[1];

            match (object_type, content) {
                ("blob", content) => {
                    print!("{}", content);
                }
                (_, _content) => {
                    println!("unknown object type {}", object_type);
                }
            }
        }

        _ => {
            println!("unknown command: {}", args[1])
        }
    };
    Result::Ok(())
}
