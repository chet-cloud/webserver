pub mod db;
pub mod pool;
pub mod utils;
pub mod http;

use std::collections::HashMap;
use std::net::TcpListener;
use std::net::TcpStream;
use pool::Pool;
use http::Request;
use http::Response;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = Pool::new(5);
    let mut handler_map:HashMap<&str, Box<dyn FnMut(Request,TcpStream) -> bool>> = HashMap::new();

    handler_map.insert("GET /cc", Box::new(|request:Request,stream: TcpStream|->bool {
        return true
    }));

    handler_map.insert("GET /ff", Box::new(|request:Request,stream: TcpStream|->bool {
        return true
    }));
    

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(move ||{
            handle_connection(stream,handler_map);
        })
    }
    println!("Shutting down.");
}





fn handle_connection(stream: TcpStream,map:HashMap<&str, Box<dyn FnMut(Request,TcpStream) -> bool>>) {
    let request = Request::parse(&stream);
    println!("{:?}",request.unwrap());
    // request.is_get_then("/test",|response|{
    //     response.send_file();
    // });
    // request.is_post_then("/user",|response|{
    //     response.send_json();
    // });
    
    map.get("GET /cc").unwrap()(request.unwrap(),stream);

    Response::response_file(stream,"hello.html");
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