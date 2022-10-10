use rusqlite::{Connection, Result, OpenFlags};

pub struct User {
    id: i32,
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

        CREATE TABLE IF NOT EXISTS User (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            data    BLOB
        );
        
        ",
        (), // empty list of parameters.
    ).unwrap();

    get_connection().execute(
        "

        CREATE TABLE IF NOT EXISTS Record (
            id      TEXT PRIMARY KEY,
            day     TEXT NOT NULL,
            userId  INTEGER NOT NULL,
            start   INTERGER NOT NULL,
            finish  INTERGER NOT NULL
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

pub fn add_user(user : &User) -> Result<usize, rusqlite::Error> {
    return get_connection().execute(
        "insert into User (name,data) values (?1,?2)",
        (user.name.to_string(),user.data.as_ref()),
    );
}

pub fn update_user(user : &User) -> Result<usize, rusqlite::Error> {
    return get_connection().execute(
        "
            UPDATE User
            SET name = ?1, data = ?2
            WHERE id = ?3;
        ",
        (user.name.to_string(),user.data.as_ref(),user.id),
    );
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

pub fn get_user_by_id(id :i32) -> Result<User, rusqlite::Error> {
    let conn = get_connection();
    let mut stmt = conn.prepare("select id, name, data from user where id = ?")?;
    let result = stmt.query_map([id], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?.next();
    let result = result.ok_or(rusqlite::Error::InvalidQuery)?;
    return result;
}

pub fn add_record(record : &Record) -> Result<usize, rusqlite::Error> {
    return get_connection().execute(
        "
            insert into Record (id,day,userId,start,finish)
            values (?1,?2,?3,?4,?5)
        ",
        (&record.id.to_string(), &record.day, &record.userId.to_string(),&record.start.to_string(),&record.finish.to_string()),
    );
}

pub fn get_record_by_id(id :&str) -> Result<Record, rusqlite::Error> {
    let conn = get_connection();
    let mut stmt = conn.prepare("
        select id, day, userId, start, finish
        from Record 
        where id = 'room1-r1-r1'
    ")?;
    let result = stmt.query_map([], |row| {
        Ok(Record {
            id: row.get(0)?,
            day: row.get(1)?,
            userId: row.get(2)?,
            start: row.get(3)?,
            finish: row.get(4)?,
        })
    })?.next();
    let result = result.ok_or(rusqlite::Error::InvalidQuery)?;
    return result;
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
    extern crate blob;
    use std::{str::FromStr, error::Error};
    use blob::Blob;

    use crate::db::*;
    use std::fs;

    fn removeDB(){
        fs::remove_file("db").expect("File delete failed")
    }

    #[test]
    fn add_user_test(){
        init();
        let result = add_user(&User{
            id: 0,
            name: "cc".to_string(),
            data: Some("{a:1}".as_bytes().to_vec())
        }).unwrap();
        assert_eq!(result, 1);

        let u = get_user_by_id(1).expect("get_user_by_id error");
        assert_eq!(&u.name, "cc");
        let dataArr = u.data.unwrap();
        let dataStr = std::str::from_utf8(&dataArr).unwrap();
        assert_eq!(dataStr, "{a:1}");

        removeDB();
    }


    #[test]
    fn get_user_by_name_test(){
        init();
        let result = add_user(&User{
            id: 0,
            name: "cc".to_string(),
            data: Some("{a:1}".as_bytes().to_vec())
        }).unwrap();

        assert_eq!(result, 1);

        let u = get_user_by_name("cc").expect("get_user_by_name error");
        assert_eq!(&u.name, "cc");
        let data_arr = u.data.unwrap();
        let data_str = std::str::from_utf8(&data_arr).unwrap();
        assert_eq!(data_str, "{a:1}");

        removeDB();
    }


    #[test]
    fn add_record_test(){
        init();
        let result = add_record(&Record { id: "room1-r1-r1".to_string(), day: "2022-10-09".to_string(), userId: 1, start: 1, finish: 12 }).unwrap();
        assert_eq!(result, 1);

        let re = get_record_by_id("room1-r1-r1").unwrap();

        assert_eq!(re.id, "room1-r1-r1");
        assert_eq!(re.day, "2022-10-09");
        assert_eq!(re.userId, 1);
        assert_eq!(re.start, 1);
        assert_eq!(re.finish, 12);

        removeDB();
    }



    


}
