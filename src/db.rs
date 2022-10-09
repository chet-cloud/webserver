
use rusqlite::{params, Connection, Result, OpenFlags};

pub fn init(){
    let conn = Connection::open_with_flags("./db",
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_URI
                // | OpenFlags::SQLITE_OPEN_NO_MUTEX
            ).unwrap();

    conn.execute(
        "

        CREATE TABLE IF NOT EXISTS Record (
            id TEXT PRIMARY KEY,
            day TEXT NOT NULL,
            userId TEXT NOT NULL,
            start INTERGER NOT NULL,
            finish INTERGER NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS User (
            id   TEXT PRIMARY KEY,
            name TEXT NOT NULL
        );
        
        insert INTO User (id, name) VALUES ('1', '1');
        insert INTO User (id, name) VALUES ('2', '1');
        insert INTO User (id, name) VALUES ('3', '1');
        insert INTO User (id, name) VALUES ('4', '1');
        
        insert INTO Record (id, day, userId, start, finish) VALUES ('1-1-1', '2022-12-12', '1', 1, 4);
        insert INTO Record (id, day, userId, start, finish) VALUES ('1-1-2', '2022-12-12', '2', 5, 6);
        insert INTO Record (id, day, userId, start, finish) VALUES ('1-1-3', '2022-12-12', '3', 6, 56);
        insert INTO Record (id, day, userId, start, finish) VALUES ('1-1-4', '2022-12-12', '4', 56, 96);
        
        ",
        (), // empty list of parameters.
    ).unwrap();

}



struct User {
    id: String,
    name: String,
    // data: Option<Vec<u8>>,
}

struct Record {
    id: String,
    day: String,
    userId: String,
    start: i32,
    finish: i32,
}

pub fn addUser(){

}

pub fn login(){

}

pub fn makeAppointment(day:String,start:i32,finish:i32){

}

pub fn cancelAppointment(day:String,start:i32,finish:i32){

}

pub fn getAppointmentByDays(day:String,to:String){

}







// #[derive(Debug)]
// struct Person {
//     id: i32,
//     name: String,
//     data: Option<Vec<u8>>,
// }

// pub fn run() -> Result<()> {
//     let conn = Connection::open_with_flags("./db",
//             OpenFlags::SQLITE_OPEN_READ_WRITE
//                 | OpenFlags::SQLITE_OPEN_CREATE
//                 | OpenFlags::SQLITE_OPEN_URI
//                 // | OpenFlags::SQLITE_OPEN_NO_MUTEX
//             ).unwrap();

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