fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // use std::fs::File;
    //
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(f) => f,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    // use std::fs::File;
    // use std::io::ErrorKind;

    // let f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {other_error:?}");
    //         }
    //     },
    // };
    // println!("f:{f:?}");

    // use std::fs::File;
    // use std::io::ErrorKind;

    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });



    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    let username = read_username_from_file().expect("read error:");
    println!("{username}");


}

use std::fs::{self};
use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     match match File::open("hello.txt") {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     }.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt");
//     Ok(())
// }