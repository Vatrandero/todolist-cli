use std::path::Path;
use std::fs::File; 
use std::io::{Read, Write, Seek, SeekFrom};
use rusqlite::Connection;
//NOTE: ORM  в данном случаи не применяю.

const SQL_CREATE_TABLE_QUERY : &str = 
"CREATE TABLE IF NOT EXISTS todos ( 
    Name VARCHAR(32) PRIMARY KEY, /* Спорно, можно было  саоздать ID 
    но судя по ТЗ name считается уникальным само по себе. */
    Description TEXT NOT NULL, 
    Category VARCHAR(32) NOT NULL, 
    Is_complteed BOOLEAN DEFAULT FALSE REFERENCES status, 
    date_time INTEGER NOT NULL 
    
)
"; //NOTE: Этот литерал, вроде как, в ходе работы программы не содержит переносов.
const SQL_INIT_TABLE_INDEX_QUERY: &str =  "CREATE INDEX catg ON todos (Cateory)" ;

const SQL_INSERT_QUERY: &str = "send "; 


pub fn load_db(file_path: &Path) -> Connection {
    // Файл базы пригодный? 
    let conn  = Connection::open(file_path).unwrap();
    conn.execute(SQL_CREATE_TABLE_QUERY, ());
    
    conn.execute(SQL_INIT_TABLE_INDEX_QUERY, ()); 
    // Если мы смлшди лькоыьб файд как БД. а в ходе проверки индекса 
    // (с влмзможным его созданием) = то мы можес спокойно отдать соединение.
    return conn
} 

pub fn commit (conn: &Connection, task: &crate::tasks::Task) -> Result<(), &'static str> {
    

    Err("Send help")
}
