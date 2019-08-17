use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    connect_todo();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


fn connect_todo () {
    let connection = sqlite::open("todo.db").unwrap();
    connection
        .execute(
            "
             CREATE TABLE todo (id INTEGER, Description TEXT, Title TEXT, Entered TEXT, Due TEXT, state INTEGER, time INTEGER, Subject TEXT);
             ",
        ).unwrap();

}
