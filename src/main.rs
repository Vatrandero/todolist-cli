pub mod db; 
pub mod tasks; 
pub mod utils; 

use std::io::Read;  
use std::path::{Path, PathBuf};
use std::fs; 
// Решаю не сканировать argv сам, использую clap.
use clap::Parser;   
// Для удобной конвертации даты и времени в секунды и обратно.
use chrono; 
use inquire;




// Да, структура и использование сторонней бибилотеки ради двух аргументов - 
// решение не очень оптимальное. 
#[derive(Parser, Debug)]
struct Args { 
/// OPtional path to Todo-list db file.
#[arg(short, long, value_name = "FILE", )] 
path_to_file: Option<PathBuf>,

/// show debug messages? 
#[arg(short, long, value_name="FLAG")]
debug : bool 
} 

fn main()  {
    
    let args = Args::parse();
    let path: &Path = match &args.path_to_file.as_deref(){ 
        Some(p) =>   p, 
        None => Path::new("./todolist.db")

    };
    if !path.exists() {
        if (args.debug) {println!("File not found, will be  created. ")};
        std::fs::File::create(path);


     }
     else if (path.exists() && path.is_dir())  {
        panic!("Path leading to Directory, not File, can not continue. ")
         
     }
     let conn = db::load_db(path);
     println!("{}", todo!());
     print!(" Welcome to TODO  list manager!");
     let sin = std::io::stdin();
     let mut buf = [0u8; 255]; 
     // Основной цикл программы
     print!(
     "you can:  add todo (add name description category). 
     default
     , show all  ");
     'mainloop: loop { 


        std::thread::sleep(std::time::Duration::from_millis(100))
    }  


} 