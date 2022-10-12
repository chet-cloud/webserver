use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender,Receiver};
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub struct Pool{
    size:usize,
    sender: Option<Sender<Job>>,
    receiver: Arc<Mutex<Receiver<Job>>>,
    threads:Vec<Option<JoinHandle<()>>>
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

impl Pool{
    pub fn new(size:usize) -> Pool {
        
        let (sender, receiver): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = vec![];
        for i in 0..size {
            let receiver = Arc::clone(&receiver);
            let handler = thread::spawn(move || {
                loop {
                    let message = receiver.lock().unwrap().recv();
                    match message {
                        Ok(job) => {
                            println!("Worker {i} got a job; executing.");
                            job();
                        }
                        Err(_) => {
                            println!("Worker {i} disconnected; shutting down.");
                            break;
                        }
                    }
                }
            });
            threads.push(Some(handler))
        }

        Pool{size,sender:Some(sender),receiver,threads}

    }

    pub fn execute<F>(&self, job:F) where F: FnOnce() + Send + 'static {
        let job = Box::new(job);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

}


impl Drop for Pool {
    fn drop(self: &mut Pool) {
        drop(self.sender.take());
        for thread in &mut self.threads {
            if let Some(handler) = thread.take() {
                handler.join().unwrap();
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;
    use rand::{Rng, thread_rng};
    use crate::db::*;

    #[test]
    fn execute_test() {

        let pool = Pool::new(3);

        for _ in 0..50 {
            pool.execute(||{
                thread::sleep(Duration::from_secs(thread_rng().gen_range(0..5)));
                println!("done");
            });
        }
    }

    #[test]
    fn execute_sql_concurrent_test() {
        let client = DbClient::new().unwrap();

        let pool = Pool::new(10);

        let id = client.add_user(&User{
            id: 0,
            name: "cc".to_string(),
            data: Some("{a:1}".as_bytes().to_vec())
        }).unwrap();
        assert_eq!(id, 1); 

        for _ in 0..100 {
            let client = client.clone();
            pool.execute(move||{
                thread::sleep(Duration::from_secs(thread_rng().gen_range(0..3)));
                let result = client.add_record(&Record { id: "room1-r1-r1".to_string(), day: "2022-10-09".to_string(), userId: 1, start: 1, finish: 12 }).unwrap();
                assert_eq!(result, 1);
            });
        }

        for _ in 0..100 {
            let client = client.clone();
            pool.execute(move||{
                thread::sleep(Duration::from_secs(thread_rng().gen_range(0..3)));
                let result = client.add_record(&Record { id: "room1-r1-r1".to_string(), day: "2022-10-09".to_string(), userId: 1, start: 13, finish: 36 }).unwrap();
                assert_eq!(result, 1);
            });
        }

        for _ in 0..100 {
            let client = client.clone();
            pool.execute(move||{
                thread::sleep(Duration::from_secs(thread_rng().gen_range(0..3)));
                let result = client.add_record(&Record { id: "room1-r1-r1".to_string(), day: "2022-10-09".to_string(), userId: 1, start: 13, finish: 36 }).unwrap();
                assert_eq!(result, 1);  
            });
        }
        
        for _ in 0..20 {
            let client = client.clone();
            pool.execute(move||{
                thread::sleep(Duration::from_secs(thread_rng().gen_range(0..3)));
                let result = client.get_records_by_like_roomid_day_userid("room1-r1-r1","2022-10-09",&1.to_string()).unwrap();
                //assert_eq!(result.len(), 3);
                println!("result:{:?}",result);
            });
        }

        thread::sleep(Duration::from_secs(5));
        DbClient::removeDB();

    }






}