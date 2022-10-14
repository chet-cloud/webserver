use std::{io::{Error, BufReader, BufRead, Read, ErrorKind, Write}, net::TcpStream, str::from_utf8};
use std::fs;


#[derive(Debug,Clone)]
pub struct Request{
    pub method: String,
    pub path: String,
    pub headers: Vec<String>,
    pub data: Option<String>,
}

impl Request {
    pub fn parse(stream:&TcpStream) -> Result<Request,Error>{
        let mut msg = Request {
            method: "".to_string(),
            path: "".to_string(),
            headers: vec![],
            data: None,
        };
        let mut reader = BufReader::new(stream.try_clone().unwrap());

        // parse first line
        let mut first_line = String::new();
        let r = reader.read_line(&mut first_line).unwrap();
        if r == 0 {
            return Err(Error::new(ErrorKind::Other, "First_line of request parse error"));
        } 
        let mut first_line_split = first_line.split(" ");
        if let Some(method) = first_line_split.next(){
            if method == "GET" || method == "HEAD" || method == "POST" || method == "PUT" || method == "DELETE" || method == "CONNECT" || method == "OPTIONS" || method == "TRACE" {
                msg.method = method.to_string();
            }else{
                return Err(Error::new(ErrorKind::Other, "Unknow Method of request parse error"));
            }
        }else{
            return Err(Error::new(ErrorKind::Other, "Method of request parse error"));
        }

        if let Some(path) = first_line_split.next(){
            msg.path = path.to_string();
        }else{
            return Err(Error::new(ErrorKind::Other, "path of request parse error"));
        }

        // parse headers
        let mut headers = String::new();
        loop {
            let r = reader.read_line(&mut headers).unwrap();
            if r < 3 { //detect empty line
                break;
            }
        }
        let mut size = 0;
        let linesplit = headers.split("\n");
        for l in linesplit {
            msg.headers.push(l.to_string());
            if l.starts_with("Content-Length") {
                    let sizeplit = l.split(":");
                    for s in sizeplit {
                        if !(s.starts_with("Content-Length")) {
                            size = s.trim().parse::<usize>().unwrap_or_else(|_|0);
                    }
                }
            }
        }
        if size > 0 {
            let mut buffer = vec![0; size]; //New Vector with size of Content   
            reader.read_exact(&mut buffer).unwrap(); //Get the Body Content.
            if let Some(data) = from_utf8(&buffer).ok(){
                msg.data = Some(data.to_string());
            }
        }
        return Ok(msg);

    }
}


pub struct Response{}
impl Response {
    pub fn response_file(mut stream:TcpStream,filename:&str){
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
    pub fn response_404(mut stream:TcpStream,filename:&str){
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn response_500(mut stream:TcpStream){
        let response = format!("HTTP/1.1 500 Internal Server Error\r\n\r\n");
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
