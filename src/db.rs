use rusqlite::{Connection, Result, OpenFlags};

pub struct User {
    id: String,
    name: String,
    data: Option<Vec<u8>>,
}

pub struct Record {
    id: String,
    day: String,
    userId: i32,
    start: i32,
    finish: i32,
}

pub fn get_connection() -> Connection {
    return Connection::open_with_flags("./db",
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_URI
                // | OpenFlags::SQLITE_OPEN_NO_MUTEX
            ).unwrap();
}

pub fn init(){
    get_connection().execute(
        "

            CREATE TABLE IF NOT EXISTS Record (
                id      TEXT PRIMARY KEY,
                day     TEXT NOT NULL,
                userId  INTEGER NOT NULL,
                start   INTERGER NOT NULL,
                finish  INTERGER NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS User (
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                name    TEXT NOT NULL,
                data    BLOB,
            );
        
        ",
        (), // empty list of parameters.
    ).unwrap();
}

// https://codebeautify.org/string-binary-converter
// https://www.binaryhexconverter.com/binary-to-hex-converter
pub fn insert_fake_date(){
    get_connection().execute(
        "

            insert INTO User (id, name) VALUES ('2', '1', X'7B613A317D');
            insert INTO User (id, name) VALUES ('3', '1', X'7B613A317D');
            insert INTO User (id, name) VALUES ('4', '1', X'7B613A317D');
            
            insert INTO Record (id, day, userId, start, finish) VALUES ('1-1-1', '2022-12-12', '1', 1, 4);
            insert INTO Record (id, day, userId, start, finish) VALUES ('1-1-2', '2022-12-12', '2', 5, 6);
            insert INTO Record (id, day, userId, start, finish) VALUES ('1-1-3', '2022-12-12', '3', 6, 56);
            insert INTO Record (id, day, userId, start, finish) VALUES ('1-1-4', '2022-12-12', '4', 56, 96);
            
        ",
        (), // empty list of parameters.
    ).unwrap();

}

pub fn add_user(user : User) -> Result<usize, rusqlite::Error> {
    return Ok(get_connection().execute(
        "insert into User (name,data) values (?1,?2)",
        (user.name,user.data),
    )?);
}

pub fn update_user(user : User) -> Result<usize, rusqlite::Error> {
    return Ok(get_connection().execute(
        "
            UPDATE User
            SET name = ?1, data = ?2
            WHERE id = ?3;
        ",
        (user.name,user.data,user.id),
    )?);
}

pub fn get_user_by_name(name : &str) -> Result<User, rusqlite::Error> {
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM User where name is ?1")?;
    return stmt.query_map([name], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?.next().ok_or(rusqlite::Error::InvalidQuery)?;
}

pub fn get_user_by_id(id :isize) -> Result<User, rusqlite::Error> {
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM User where id is ?2")?;
    return stmt.query_map([&id.to_string()], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?.next().ok_or(rusqlite::Error::InvalidQuery)?;
}

pub fn delete_record_by_id(id : &str) -> Result<usize, rusqlite::Error> {
    return Ok(get_connection().execute(
        "delete from Record where id = ?1",
        [id],
    )?);
}

pub fn update_record(record : &Record) -> Result<usize, rusqlite::Error> {
    return Ok(get_connection().execute(
        "
            UPDATE Record
            SET day = ?1, from = ?2, to = ?3
            WHERE id = ?4
        ",
        (&record.day, &record.start.to_string(),&record.finish.to_string(), &record.id.to_string()),
    )?);
}

pub fn get_records_by_like_roomid_day_userid(id : &str, day: &str, userid:&str) -> Result<Vec<(Record, User)>, rusqlite::Error>{
    let conn = get_connection();
    let mut stmt = conn.prepare("
        SELECT id,day,userId,start,finish,name,data FROM Record 
        join User on User.id = Record.userId
        where id LIKE ?1 and day LIKE ?2 amd userId like ?3
    ")?;
    let records = stmt.query_map([id,day,userid], |row| {
        Ok((
            Record {
                id: row.get(0)?,
                day: row.get(1)?,
                userId: row.get(2)?,
                start: row.get(3)?,
                finish: row.get(4)?,
            },
            User {
                id: row.get(2)?,
                name: row.get(5)?,
                data: row.get(6)?
            }
        ))
    })?;

    let mut result = Vec::new();
    for item in records {
        result.push(item?);
    }
    return Ok(result);
}





#[cfg(test)]
mod tests {
    use crate::db::*;
    use std::fs;

    fn removeDB(){
        fs::remove_file("db").expect("File delete failed")
    }

    #[test]
    fn add_user_test() {
        init();

        add_user(User{
            id: 1.to_string(),
            name: "cc".to_string(),
            data: todo!(),
        });

        assert_eq!(4, 4);
    }



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