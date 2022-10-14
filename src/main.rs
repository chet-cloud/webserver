pub mod db;
pub mod pool;
pub mod utils;
pub mod http;

use std::net::TcpListener;
use std::net::TcpStream;
use pool::Pool;
use http::Request;
use http::Response;

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



fn handle_connection(stream: TcpStream) {
    let request = Request::parse(&stream);
    println!("{:?}",request.unwrap());
    // request.is_get_then("/test",|response|{
    //     response.send_file();
    // });
    // request.is_post_then("/user",|response|{
    //     response.send_json();
    // });
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