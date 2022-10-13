use std::io::Write;
use std::net::TcpStream;
use std::fs;

static OK: &str = "HTTP/1.1 200 OK";
static NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";

pub fn response_file(mut stream:TcpStream,filename:&str){
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        OK,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn response_404(mut stream:TcpStream,filename:&str){
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        NOT_FOUND,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

