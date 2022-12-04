mod parse;
use std::fs::File;
use std::io::Read;
use std::str::{self};


// #[cfg(test)]
// #[path = "./foo_test.rs"]
// mod foo_test;

#[allow(dead_code)]
fn main() {
	let mut file = File::open("examples/test.json").unwrap();
	let mut input: Vec<u8> = Vec::new();
	file.read_to_end(&mut input);
    let ast = parse::mast_commands().parse(input.as_slice());
    match ast {
        Ok(ast) => println!("{:?}", ast),
        Err(e) => match e{
            pom::Error::Mismatch { message, position}  => {
                let start = if position <15 { 0} else {position -15};
                let end = if input.len() < position+15 { input.len() } else {position+15};
                println!("{} {}", message, str::from_utf8(&input[start..end]).unwrap())
            },
            _ => println!("{:?}", e),
        }
    }
	
}