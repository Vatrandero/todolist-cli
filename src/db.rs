//! Здесь описана логика работы с базой данных и 
//! структура-обёртка. 
use std::path::Path; 
use std::io::{Read, Write, Seek, SeekFrom};
use rusqlite::{params, Connection, Statement};
//NOTE: ORM  в данном случаи не применяю.

 
/// Обёртка, существует для более удобного 
/// вызова функций, связанных с БД и 
/// для консистенции кода. 
pub struct ConnHandler{
    conn : rusqlite::Connection
}
impl ConnHandler {
    

pub fn load_db(file_path: &Path) -> Self {
    /*  
     NOTE: У нас есть опрежедённые гарантии, что соединение 
     к базе данных откроется, поэтому .unwrap() здесь допустим.
     хотя здесь пишется код не отдельной библиотеки - стоит 
     избегать возможных паник и возвращать ошибку через Result.
     
    */ 
    let conn  = Connection::open(file_path).unwrap();
    _ = conn.execute("CREATE TABLE IF NOT EXISTS todos ( 
    Name VARCHAR(32) PRIMARY KEY, /* Спорно, можно было бы  саоздать ID 
    но судя по ТЗ name считается уникальным само по себе. */
    Description TEXT NOT NULL, 
    Category VARCHAR(32) NOT NULL, 
    Is_complteed BOOLEAN DEFAULT FALSE REFERENCES status, 
    Creation_date_time DATETIME NOT NULL 
    
)
", ());
    
    conn.execute("CREATE INDEX catg ON todos (Cateory)", ()); 
    // Если мы смлшди лькоыьб файд как БД. а в ходе проверки индекса 
    // (с влмзможным его созданием) = то мы можес спокойно отдать соединение.
    return ConnHandler { conn  }
} 

pub fn commit_new (&self, task: &crate::tasks::Task) 
/* NOTE: Было бы правильно вместо строк ошибок вовраать корректный 
объект - Enum реализующий Error и Display, но думаю
в контексте текущего проекта - 
который ещё и приложения, то  вполне допустимо вернуть
просто строку. */
-> Result<(), String> 
{
    let prer_res =  self.conn.prepare("INSERT INTO todos ( 
    Name, Description, Category, date_creation
)
    VALUES (?,?,?,? ) ");
    //? Что вообеще модет пойти не так? 
    //? Стоит убрать этот match и просто unwrap'ать, если проблема
    //? возникла при подготовке - вероятно 
    //? неисправимая проблема runtime или 
    //? библиотеки. 
    // Оставлю на тот случай, еслли база данных повреждена.
    let mut stmt =  match prer_res  {
        Ok(s) => s, 

        // NOTE: Возможно тоит избегать строковых ошибок и отдавать ErrorKind, можно составить внутри проекта. 
        // NOTE: Было бы эквивалентом PreparationError
        Err(_) => return Err("Error on preparing query to data base.
                            Database corrupted or SQLite failed itself?".to_string()) 
        
        
    };
    // get_all раскладывает Task в картеж, элементы
    // Которого идут как параметры в запрос.
    // Оптимальность - достаточная, есть ли более оптимальный вариант? 
    // djpvj;yj/
    let p = task.get_all(); 
    match stmt.execute(params![p.0, p.1, p.2, p.3]) { 
        Ok(_) => return  Ok(()) ,
        // Было бы эквивалетно: ExecutionFailure
        // Либо, т.к .execute вовзращает явную ошибку - rusqliteError(e)
        Err(e) => return Err(format!( "Error on query execution. \n {}", e))
    }
    
}

pub fn commit_update(&self, task: crate::tasks::Task) 
-> Result<(), String>
  { 
    /* Здесь произошло столкновение с фактом, до которого 
     достаточно легко догадаться - но его стоит проверитть: 
     SQLite не позволяет напрямую обновлять PRIMARY KEY 
     столбцы, а это значит - что запись придётся сначала обновить 
     т.к в команда update из ТЗ предпологает замену имени. 
     Есть 3 путя решения: 
        1. Сначало удалить запись, затем создать новую. 
        2. Модифицировать таблицу и доавбить в неё поле ID, 
        что следовало сделать изначально.
        3. ТЗ не уточняет список параметров. предпологаюих
        замену, так что name может рассмариваться как поле, котторое
        не подлежит замене.
    
    Вообще - стоит признать, что я виноват не уточнив в первый день 
    такие вопросы.*/
    let prer_res =  self.conn.prepare("UPDATE todos ( 
     Description = ? , Category = ? , 
     Date_creation = ?, Is_completed = ?) 
     WHERE name = ? 
 "); // Порядок для get_all = 1,2,3,4,0
        //? Что вообеще модет пойти не так? 
        //? Стоит убрать этот match и просто unwrap'ать, если проблема
        //? возникла при подготовке - вероятно 
        //? неисправимая проблема runtime или 
        //? библиотеки. 
        // Оставлю на тот случай, еслли база данных повреждена.
        let mut stmt =  match prer_res  {
            Ok(s) => s, 
    
            // NOTE: Возможно тоит избегать строковых ошибок и отдавать ErrorKind, можно составить внутри проекта. 
            // NOTE: Было бы эквивалентом PreparationError
            Err(_) => return Err("Error on preparing query to data base.
                                Database corrupted or SQLite failed itself?".to_string()) 
            
            
        };
        // get_all раскладывает Task в картеж, элементы
        // Которого идут как параметры в запрос.
        // Оптимальность - достаточная, есть ли более оптимальный вариант? 
        // djpvj;yj/
        let p = task.get_all(); 
        match stmt.execute(params![p.1, p.2, p.3, p.4 as u8, p.0]) { 
            Ok(_) => return  Ok(()) ,        //   ^^^^^^^^^ не уверен, превратится ли bool в 0|1.
            // Было бы эквивалетно: ExecutionFailure
            // Либо, т.к .execute вовзращает явную ошибку - rusqliteError(e)
            Err(e) => return Err(format!( "Error on query execution. \n {}", e))
        }
        
    }
    pub fn remove(&self, name:&String) {
        // Не имеет особого смысла проверять ошибку.
        _ = self.conn.prepare("DELETE FROM todos WHERE name = ? ").unwrap().execute(params![name]);
        
        
    }
    pub fn select_where(&self, predicate:  &str ) -> Result<Statement<'_>, String>   {
        //note: в коммите 13 ветки master WHERE и предикаты теперь опциональная.
        match  self.conn.prepare(
            "SELECT * FROM todos PREDICATES  "
            .replace("PREDICATES", predicate).as_str()) { 
                Ok(r) => return Ok(r), 
                Err(e) => return Err( format!("Error ehile prepraring query, bad request? \n {} ", e))
            
            }
        

    }
} 
