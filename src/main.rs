pub mod db;
pub mod pool;
pub mod utils;

use std::io;
use std::io::BufReader;
use std::str::Utf8Error;
use std::str::from_utf8;
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

fn get_body(stream:&TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut name = String::new();
    loop {
    let r = reader.read_line(&mut name).unwrap();
        if r < 3 { //detect empty line
            break;
        }
    }
    let mut size = 0;
    let linesplit = name.split("\n");
    for l in linesplit {
        if l.starts_with("Content-Length") {
                let sizeplit = l.split(":");
                for s in sizeplit {
                    if !(s.starts_with("Content-Length")) {
                        size = s.trim().parse::<usize>().unwrap(); //Get Content-Length
                }
            }
        }
    }
    let mut buffer = vec![0; size]; //New Vector with size of Content   
    reader.read_exact(&mut buffer).unwrap(); //Get the Body Content.

    let str =  from_utf8(&buffer).map_err(|_| {});
    println!("input: {}", str.unwrap());
}


fn handle_connection(mut stream: TcpStream) {
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages
    
    // https://stackoverflow.com/questions/71478238/rust-tcpstream-reading-http-request-sometimes-lose-the-body
    
    get_body(&stream);

    let mut buf_reader = BufReader::new(&mut stream);
    
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        //.take_while(|line| !line.is_empty())
        .collect();

    // for line in http_request.iter() {
    //     println!("{:?}",line);
    // }

    if http_request.len() == 0 {
        stream.flush();
    }

    let first_line = http_request[0].to_string();
    let filename= if first_line.starts_with(&"GET / HTTP/1.1".to_string()){
        "hello.html"
    } else if first_line.starts_with(&"GET /sleep HTTP/1.1".to_string()) {
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
        let s1 = String::from("cccc");
        let s2 = String::from("cccc");
        assert!(s1.ends_with(&s2));
        assert!(s1.ends_with(&s2));
        assert!(s1.cmp(&s2).is_eq());
    }

}