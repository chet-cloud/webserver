pub mod db;
pub mod pool;

use std::io::Error;
use std::thread::JoinHandle;
use std::time::Duration;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;





fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();

    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[cfg(test)]
mod test {
    extern crate r2d2;
    extern crate r2d2_sqlite;
    extern crate rusqlite;
    
    use std::thread;
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::params;

    #[test]
    fn test() {
        let manager = SqliteConnectionManager::file("file.db");
        let pool = r2d2::Pool::new(manager).unwrap();
        pool.get()
            .unwrap()
            .execute("CREATE TABLE IF NOT EXISTS foo (bar INTEGER)", params![])
            .unwrap();
    
        (0..10000)
            .map(|i| {
                let pool = pool.clone();
                thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    conn.execute("INSERT INTO foo (bar) VALUES (?)", &[&i])
                        .unwrap();
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(thread::JoinHandle::join)
            .collect::<Result<_, _>>()
            .unwrap()
    }

}