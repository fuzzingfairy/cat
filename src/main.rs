#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;


use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::error::Error;
use std::fs;
use rocket::response::content::Html;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use sqlite::State;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Video {
    link: String,
    category: String
}

#[get("/")]
fn index() -> Option<NamedFile> {
    return NamedFile::open("main.html").ok();
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    get_videos();    
    return format!("Hello, {}!", name);
}

#[get("/api/get/videos")]
fn getVids() -> Json<Video> {
    Json(Video {
        link: "https://alicereuter.com".to_string(),
        category: "blog".to_string()
    })
}
//json
fn main() {
   // connect_todo();
    rocket::ignite().mount("/", routes![index,hello,getVids]).launch();
}

fn get_videos() {
    let connection = sqlite::open("todo.db").unwrap();
    let mut statement = connection.prepare("SELECT * FROM vids").unwrap();
    statement.bind(1,50).unwrap();
    while let State::Row = statement.next().unwrap() {
        println!("name = {}", statement.read::<String>(0).unwrap());
    }
    
}

fn connect_todo () {
    let connection = sqlite::open("todo.db").unwrap();
    connection
        .execute(
            "
             CREATE TABLE vids (link TEXT, subject TEXT);
             ",
        ).unwrap();
}

