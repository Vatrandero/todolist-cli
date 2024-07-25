//! Здесь описана логика работы с базой данных и 
//! структура-обёртка. 
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

const SQL_INSERT_QUERY_TEMPLATE: &str = "send "; 
const SQL_UPDATE_QUERY_TEMPLATE: &str = "it's fine.";

/// Обёртка, существует для более удобного 
/// вызова функций, связанных с БД и 
/// для консистенции кода. 
pub struct ConnHandler{
    conn : rusqlite::Connection
}
impl ConnHandler {
    

pub fn load_db(file_path: &Path) -> Self {
    // Файл базы пригодный? 
    let conn  = Connection::open(file_path).unwrap();
    conn.execute(SQL_CREATE_TABLE_QUERY, ());
    
    conn.execute(SQL_INIT_TABLE_INDEX_QUERY, ()); 
    // Если мы смлшди лькоыьб файд как БД. а в ходе проверки индекса 
    // (с влмзможным его созданием) = то мы можес спокойно отдать соединение.
    return ConnHandler { conn  }
} 

pub fn commit_new (&self, task: &crate::tasks::Task) -> Result<(), &'static str> {
    
    Ok(())
}

pub fn commit_update(&self, task: crate::tasks::Task)  { 

    }
} 