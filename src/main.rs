use flate2::read::ZlibDecoder;
use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
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
            // TODO: implement arg parsing lel
            let blob_sha = args[3].clone();
            let dir_name = &blob_sha[0..2];
            let file_name = &blob_sha[2..];
            let contents = fs::read(format!(".git/objects/{}/{}", dir_name, file_name)).unwrap();

            let mut z = ZlibDecoder::new(&contents[..]);
            let mut contents = String::new();
            z.read_to_string(&mut contents).unwrap();

            print!("{}", contents);
        }

        _ => {
            println!("unknown command: {}", args[1])
        }
    }
}
