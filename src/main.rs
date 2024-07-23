pub mod db; 
pub mod tasks; 
use std::io::Read;  
use std::path::{Path, PathBuf};
use std::fs; 
// Решаю не сканировать argv сам, использую clap.
use clap::Parser;   
// Для удобной конвертации даты и времени в секунды и обратно.
use chrono; 





// Да, структура и использование сторонней бибилотеки ради двух аргументов - 
// решение не очень оптимальное. 
#[derive(Parser, Debug)]
struct Args { 
/// OPtional path to Todo-list db file.
#[arg(short, long, value_name = "FILE", )] 
path_to_file: Option<PathBuf>,

/// show debug messages? 
#[arg(short, long)]
debug : bool 
} 

fn main()  {
    
    let args = Args::parse();
    let path: &Path = match args.path_to_file.as_deref(){ 
        Some(p) =>   p, 
        None => Path::new("./todolist.db")

    };
    if !path.exists() {
        #[cfg(debug_assertions)]
        println!("File not found, will be  created. ");
        std::fs::File::create(path);


     }
     else if (path.exists() && path.is_dir())  {
        panic!("Path leading to Drectory, not File, can not continue. ")
         
     }
     let conn = db::load_db(path);

     let sin = std::io::stdin();
     // Основной цикл программы
     'mainloop: loop { 

    }  


} 