//use db::ConnecionHandler; 
pub mod tasks; 
pub mod utils; 
mod db; 
use crate::db::ConnHandler;
use std::io::{BufRead,Read};  
use std::path::{Path, PathBuf};
use std::fs; 
// Решаю не сканировать argv сам, использую clap.
use clap::Parser;   
// Для удобной конвертации даты и времени в секунды и обратно.
use chrono; 
use inquire;



enum Action { 
    Add,
    Remove, 
    Update, 
    Select, 
    NoAct
}

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
  
    //let debug: bool; 
  
    // debug? переменная влияет на появление некоторых сообщения.
    // для отладки, включена всегда если дебаг-сборка, 
    // В релизе включается только аргументом.
    // Можно было бы вынести a lazy_static или 
    // каждый раз парсить argv. что бы предоставиь 
    // генерацию сообщений в друие участки программы, 
    // но тогда лучше задуматься о добавлении log+simplelog библиотек.
    #[cfg(debug_assertions)] 
    let debug = true; 
    #[cfg(not(debug_assertions))]
    let debug = args::debug; 


    if !path.exists() {
        if (args.debug) {println!("File not found, will be  created. ")};
        std::fs::File::create(path);


     }
     else if (path.exists() && path.is_dir())  {
        panic!("Path leading to Directory, not File, can not continue. ")
         
     }
     let conn = todo!(); //db::load_db(path);
  //   println!("{}", todo!());
     print!(" Welcome to TODO  list manager!");
     let sin = std::io::stdin();
     // В какой-то момент стали нужны сращу 3 вида буферов.
     // TODO: Следующему второму оставить один, в худшем случаи 2.
     let mut buf = [0u8; 255];
     let mut sbuf: String = String::with_capacity(255); 
     let mut vbuf: Vec<u8> = Vec::with_capacity(255);
     // Основной цикл программы
     print!(
     "you can:  add todo (add name description date category). 
        Use quote marks to have multiple words per operands.
        date shoukd be YYYY-MM-DD
        remov: remove name
        update: (update namme); will start interactive mode. 
     show all  ");
     'mainloop: loop { 
        sin.read_line(&mut sbuf).unwrap();
        'scanloop: for c in sbuf.chars() {
        if c == ' ' {
            todo!() 
        }
        } 
        std::thread::sleep(std::time::Duration::from_millis(100))
    }  


} 