
use rusqlite::{Connection, Result, OpenFlags};
use std::fs;

#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub data: Option<Vec<u8>>,
}
#[derive(Debug)]
pub struct Record {
    pub id: String,
    pub day: String,
    pub userId: usize,
    pub start: usize,
    pub finish: usize,
}

pub struct DbClient{
    conn:Connection
}

impl DbClient {
    
    pub fn new() ->  Result<DbClient> {
        // return Connection::open_in_memory().unwrap();
        let connection =  Connection::open_with_flags("./db",
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_URI
            // | OpenFlags::SQLITE_OPEN_NO_MUTEX
        )?;
        return Ok(DbClient{conn:connection});
    }

    pub fn init(&self)-> Result<(), rusqlite::Error> {
        self.conn.execute(
            "

            CREATE TABLE IF NOT EXISTS User (
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                name    TEXT NOT NULL,
                data    BLOB,
                UNIQUE(name)
            );
            
            ",
            (), // empty list of parameters.
        );

        self.conn.execute(
            "

            CREATE TABLE IF NOT EXISTS Record (
                id      TEXT KEY,
                day     TEXT NOT NULL,
                userId  INTEGER NOT NULL,
                start   INTERGER NOT NULL,
                finish  INTERGER NOT NULL
            );
            
            ",
            (), // empty list of parameters.
        );

        self.conn.execute(
            "
            CREATE INDEX Record_id ON Record (id);
            ",
            (), // empty list of parameters.
        );

        return Ok(())
        
    }

    // https://codebeautify.org/string-binary-converter
    // https://www.binaryhexconverter.com/binary-to-hex-converter
    pub fn insert_fake_date(&self)-> Result<usize, rusqlite::Error> {
        return self.conn.execute(
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
        );

    }

    pub fn add_user(&self,user : &User) -> Result<usize, rusqlite::Error> {

        let mut stmt = self.conn.prepare("
            insert into User (name,data) values (?1,?2) RETURNING id; 
        ")?;
        let result = stmt.query_map((user.name.to_string(),user.data.as_ref()), |row| {
            Ok(row.get(0)?)
        })?.next().ok_or(rusqlite::Error::InvalidQuery)?;
        return result;

    }

    pub fn update_user(&self,user : &User) -> Result<usize, rusqlite::Error> {
        return self.conn.execute(
            "
                UPDATE User
                SET name = ?1, data = ?2
                WHERE id = ?3;
            ",
            (user.name.to_string(),user.data.as_ref(),user.id),
        );
    }

    pub fn get_user_by_name(&self,name : &str) -> Result<User, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM User where name is ?1")?;
        return stmt.query_map([name], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })?.next().ok_or(rusqlite::Error::InvalidQuery)?;
    }

    pub fn get_user_by_id(&self,id :i32) -> Result<User, rusqlite::Error> {
        let mut stmt = self.conn.prepare("select id, name, data from user where id = ?")?;
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

    pub fn add_record(&self,record : &Record) -> Result<usize, rusqlite::Error> {
        return self.conn.execute(
            "
                insert into Record (id,day,userId,start,finish)
                values (?1,?2,?3,?4,?5)
            ",
            (&record.id.to_string(), &record.day, &record.userId.to_string(),&record.start.to_string(),&record.finish.to_string()),
        );
    }

    pub fn get_record_by_id(&self,id :&str) -> Result<Record, rusqlite::Error> {
        let mut stmt = self.conn.prepare("
            select id, day, userId, start, finish
            from Record 
            where id = ?1
        ")?;
        let result = stmt.query_map([id], |row| {
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

    pub fn delete_record_by_id(&self,id : &str) -> Result<usize, rusqlite::Error> {
        return Ok(self.conn.execute(
            "delete from Record where id = ?1",
            [id],
        )?);
    }

    pub fn update_record(&self,record : &Record) -> Result<usize, rusqlite::Error> {
        return Ok(self.conn.execute(
            "
                UPDATE Record
                SET day = ?1, start = ?2, finish = ?3
                WHERE id = ?4
            ",
            (&record.day, &record.start.to_string(),&record.finish.to_string(), &record.id.to_string()),
        )?);
    }

    pub fn get_records_by_like_roomid_day_userid(&self,id : &str, day: &str, userid:&str) -> Result<Vec<(Record, User)>, rusqlite::Error>{
        let mut stmt = self.conn.prepare("
            SELECT Record.id, Record.day, Record.userId, Record.start, Record.finish, User.name, User.data 
            FROM Record 
            join User on User.id = Record.userId
            where Record.id LIKE ?1 and Record.day LIKE ?2 and Record.userId like ?3
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

    pub fn removeDB(){
        fs::remove_file("db").expect("File delete failed")
    }

}


#[cfg(test)]
mod tests {
    use crate::db::*;
    
    #[test]
    fn add_user_test(){
        let client = DbClient::new().unwrap();

        let id = client.add_user(&User{
            id: 0,
            name: "cc".to_string(),
            data: Some("{a:1}".as_bytes().to_vec())
        }).unwrap();
        assert_eq!(id, 1);

        let id = client.add_user(&User{
            id: 0,
            name: "ff".to_string(),
            data: Some("{a:1}".as_bytes().to_vec())
        }).unwrap();
        assert_eq!(id, 2);

        let u = client.get_user_by_id(1).expect("get_user_by_id error");
        assert_eq!(&u.name, "cc");
        let data_arr = u.data.unwrap();
        let data_str = std::str::from_utf8(&data_arr).unwrap();
        assert_eq!(data_str, "{a:1}");

        DbClient::removeDB();
    }

    #[test]
    fn get_user_by_name_test(){
        let client = DbClient::new().unwrap();

        let id = client.add_user(&User{
            id: 0,
            name: "cc".to_string(),
            data: Some("{a:1}".as_bytes().to_vec())
        }).unwrap();

        assert_eq!(id, 1);

        let u = client.get_user_by_name("cc").expect("get_user_by_name error");
        assert_eq!(&u.name, "cc");
        let data_arr = u.data.unwrap();
        let data_str = std::str::from_utf8(&data_arr).unwrap();
        assert_eq!(data_str, "{a:1}");

        DbClient::removeDB();
    }

    #[test]
    fn add_record_test(){
        let client = DbClient::new().unwrap();

        let result = client.add_record(&Record { id: "room1-r1-r1".to_string(), day: "2022-10-09".to_string(), userId: 1, start: 1, finish: 12 }).unwrap();
        assert_eq!(result, 1);

        let re = client.get_record_by_id("room1-r1-r1").unwrap();

        assert_eq!(re.id, "room1-r1-r1");
        assert_eq!(re.day, "2022-10-09");
        assert_eq!(re.userId, 1);
        assert_eq!(re.start, 1);
        assert_eq!(re.finish, 12);

        DbClient::removeDB();
    }

    #[test]
    fn delete_record_by_id_test(){
        let client = DbClient::new().unwrap();


        let result = client.add_record(&Record { id: "room1-r1-r1".to_string(), day: "2022-10-09".to_string(), userId: 1, start: 1, finish: 12 }).unwrap();
        assert_eq!(result, 1);
        let re = client.delete_record_by_id("room1-r1-r1").unwrap();
        assert_eq!(re, 1);
        DbClient::removeDB();
    }

    #[test]
    fn update_record_test(){
        let client = DbClient::new().unwrap();

        let result = client.add_record(&Record { id: "room1-r1-r1".to_string(), day: "2022-10-09".to_string(), userId: 1, start: 1, finish: 12 }).unwrap();
        assert_eq!(result, 1);

        let re = client.update_record(&Record { id: "room1-r1-r1".to_string(), day: "1111-11-11".to_string(), userId: 2, start: 2, finish: 13 }).unwrap();
        assert_eq!(re, 1);

        let re = client.get_record_by_id("room1-r1-r1").unwrap();
        assert_eq!(re.id, "room1-r1-r1");
        assert_eq!(re.day, "1111-11-11");
        assert_eq!(re.userId, 1); // userId didn't change
        assert_eq!(re.start, 2);
        assert_eq!(re.finish, 13);

        DbClient::removeDB();
    }

    #[test]
    fn get_records_by_like_roomid_day_userid_test(){
        let client = DbClient::new().unwrap();

        let id = client.add_user(&User{
            id: 0,
            name: "cc".to_string(),
            data: Some("{a:1}".as_bytes().to_vec())
        }).unwrap();
        assert_eq!(id, 1);

        let result = client.add_record(&Record { id: "room1-r1-r1".to_string(), day: "2022-10-09".to_string(), userId: id, start: 1, finish: 12 }).unwrap();
        assert_eq!(result, 1);

        let result = client.add_record(&Record { id: "room1-r1-r1".to_string(), day: "2022-10-09".to_string(), userId: id, start: 13, finish: 36 }).unwrap();
        assert_eq!(result, 1);

        let result = client.get_records_by_like_roomid_day_userid("room1-r1-r1","2022-10-09",&id.to_string()).unwrap();
        //println!("{:?}",result);

        // [
        //     (
        //         Record { id: "room1-r1-r1", day: "2022-10-09", userId: 1, start: 1, finish: 12 }, 
        //         User { id: 1, name: "cc", data: Some([123, 97, 58, 49, 125]) }
        //     ), 
        //     (
        //         Record { id: "room1-r1-r1", day: "2022-10-09", userId: 1, start: 13, finish: 36 }, 
        //         User { id: 1, name: "cc", data: Some([123, 97, 58, 49, 125]) }
        //     )
        // ]
        assert_eq!(result.len(), 2);

        DbClient::removeDB();
    }



}
