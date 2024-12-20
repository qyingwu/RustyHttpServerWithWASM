use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    //send msg to server
    stream.write("Hello".as_bytes()).unwrap();


    //read from server
    let mut buffer = [0;5]; 
    stream.read(&mut buffer).unwrap();

    println!("Response from server:{:?}", str::from_utf8(&buffer).unwrap());
}
