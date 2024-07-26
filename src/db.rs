//! Здесь описана логика работы с базой данных и 
//! структура-обёртка. 
use std::path::Path;
use std::fs::File; 
use std::io::{Read, Write, Seek, SeekFrom};
use rusqlite::Connection;
//NOTE: ORM  в данном случаи не применяю.

 
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
    conn.execute("CREATE TABLE IF NOT EXISTS todos ( 
    Name VARCHAR(32) PRIMARY KEY, /* Спорно, можно было бы  саоздать ID 
    но судя по ТЗ name считается уникальным само по себе. */
    Description TEXT NOT NULL, 
    Category VARCHAR(32) NOT NULL, 
    Is_complteed BOOLEAN DEFAULT FALSE REFERENCES status, 
    Creation_date_time INTEGER NOT NULL 
    
)
", ());
    
    conn.execute("CREATE INDEX catg ON todos (Cateory)", ()); 
    // Если мы смлшди лькоыьб файд как БД. а в ходе проверки индекса 
    // (с влмзможным его созданием) = то мы можес спокойно отдать соединение.
    return ConnHandler { conn  }
} 

pub fn commit_new (&self, task: &crate::tasks::Task) 
-> Result<(), &'static str> {

    let prer_res =  self.conn.prepare("INSERT INTO todos ( 
    Name, Description, Category, date_creation
)
    VALUES (?,?,?,? ) ");     
    Err("Not done yet ") 
}

pub fn commit_update(&self, task: crate::tasks::Task) 
-> Result<(), &'static str>  { 
    Err("Not done yet.")
    }
} 
impl crate::tasks::Task { 
    fn fron_table() {}
   
   
    
}  