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

struct ThreadPool {
    threads:Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, Error> {
        assert!(size > 0);
        let threads = Vec::with_capacity(size);
        for _ in 0..size {
            // create some threads and store them in the vector
        }
        Ok(ThreadPool {threads})
    }
    pub fn spawn<F>(&self, f: F) where F: FnOnce(), F: Send + 'static
    {
        
    }
}




fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4).unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.spawn( move ||{
            handle_connection(stream);
        });
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

    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::{channel, Receiver};
    use std::thread;
    use std::time::Duration;
    use rand::{Rng, thread_rng};

    fn worker(id:usize,rx:&Arc<Mutex<Receiver<i32>>>) {
        let rx = Arc::clone(rx);
        thread::spawn(move|| {
            println!("id:{} start",id);
            loop {
                let j = rx.lock().unwrap().recv().unwrap();
                let mut rng = thread_rng();
                thread::sleep(Duration::from_secs(rng.gen_range(0..5)));
                println!("id:{}, done:{}",id,j)
            }
            println!("id:{} end",id);
        });
    }

    #[test]
    fn test1() {
        let (tx, rx) = channel();
        let rx = &Arc::new(Mutex::new(rx));

        for i in 0..100 {
            let tx = tx.clone();
            thread::spawn(move|| {
                tx.send(i).unwrap();
                println!("send: {}",i)
            });
        }

        for i in 0..10 {
            worker(i,  rx);
        }

        thread::sleep(Duration::from_secs(60));

    }
}