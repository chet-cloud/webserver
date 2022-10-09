// use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs};

// fn main(){
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         handle_connection(stream);
//     }
// }

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result|result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();
//     println!("RequestL {:#?}", http_request);

//     let status_line = "HTTP/1.1 200 ok";
//     let contents = fs::read_to_string("hello.html").unwrap();
//     let length = contents.len();
//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//     stream.write_all(response.as_bytes()).unwrap();
// }



// extern crate r2d2;
// extern crate r2d2_sqlite;
// extern crate rusqlite;

// use std::thread;
// use r2d2_sqlite::SqliteConnectionManager;
// use rusqlite::params;

// fn main() {
//     let manager = SqliteConnectionManager::file("file.db");
//     let pool = r2d2::Pool::new(manager).unwrap();
//     pool.get()
//         .unwrap()
//         .execute("CREATE TABLE IF NOT EXISTS foo (bar INTEGER)", params![])
//         .unwrap();

//     (0..10).map(|i| {
//             let pool = pool.clone();
//             thread::spawn(move || {
//                 let conn = pool.get().unwrap();
//                 conn.execute("INSERT INTO foo (bar) VALUES (?)", &[&i])
//                     .unwrap();
//             })
//         })
//         .collect::<Vec<_>>()
//         .into_iter()
//         .map(thread::JoinHandle::join)
//         .collect::<Result<_, _>>()
//         .unwrap()
// }


// use rusqlite::{params, Connection, Result};

// #[derive(Debug)]
// struct Person {
//     id: i32,
//     name: String,
//     data: Option<Vec<u8>>,
// }

// fn main() -> Result<()> {
//     let conn = Connection::open_in_memory()?;

//     conn.execute(
//         "CREATE TABLE person (
//             id   INTEGER PRIMARY KEY,
//             name TEXT NOT NULL,
//             data BLOB
//         )",
//         (), // empty list of parameters.
//     )?;
//     let me = Person {
//         id: 0,
//         name: "Steven".to_string(),
//         data: None,
//     };
//     conn.execute(
//         "INSERT INTO person (name, data) VALUES (?1, ?2)",
//         (&me.name, &me.data),
//     )?;

//     let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
//     let person_iter = stmt.query_map([], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             data: row.get(2)?,
//         })
//     })?;

//     for person in person_iter {
//         println!("Found person {:?}", person.unwrap());
//     }
//     Ok(())
// }


pub mod db;

fn main(){
    db::init();
}