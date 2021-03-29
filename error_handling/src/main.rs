use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("panik");
    // let f: u32 = File::open("hello.txt"); // compile time error cos it returns a Result<T, E> generic enum, not u32

    let f = File::open("hello.txt");
    let f1 = match f {
        Ok(ref file) => file,   // ref only so I can have multiple examples without move
        Err(error) => panic!("Problem opening the file: {:?}", error), // Literally prints "No such file or directory"
    };

    let f = File::open("hello.txt");
    let f2 = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

use std::io;
use std::io::Read;
fn read_username_from_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
