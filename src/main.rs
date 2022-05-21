use anyhow::{Context, Result};
use std::env;
use std::io::Read;

mod mp4_body;
mod parse;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <file>");
        std::process::exit(1);
    }

    let content = read_file(&args[1])?;
    // let res = parse::parse(&content)?;
    let res = parse::analyzeMp4BoxInfo(&content)?;
    println!("{:?}", res);
    Ok(())
}

fn read_file(file_name: &str) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(file_name)
        .with_context(|| format!("Couldn't open file `{}`", file_name))?;

    let mut buf = vec![];
    file.read_to_end(&mut buf)
        .with_context(|| format!("Couldn't read file `{}`", file_name))?;
    Ok(buf)
}
