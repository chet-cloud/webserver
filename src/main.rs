pub mod db;
pub mod pool;
pub mod utils;

use std::io::BufReader;
use std::time::Duration;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use pool::Pool;
use utils::response_file;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = Pool::new(5);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(move ||{
            handle_connection(stream);
        })
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    for line in http_request.iter() {
        println!("{:?}",line);
    }

    let first_line = &http_request[0];
    let filename= if first_line.cmp(&"GET / HTTP/1.1".to_string()).is_eq() {
        "hello.html"
    } else if first_line.cmp(&"GET /sleep HTTP/1.1".to_string()).is_eq() {
        thread::sleep(Duration::from_secs(5));
        "hello.html"
    } else {
        "404.html"
    };
    response_file(stream,filename);

}












#[cfg(test)]
mod test {

    #[test]
    fn test() {
        
    }

}