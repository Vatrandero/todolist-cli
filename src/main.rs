//use db::ConnecionHandler; 
pub mod tasks; 
pub mod utils; 
mod db; 
use crate::db::ConnHandler;
use std::io::{BufRead,Read};  
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::fs;
use std::str::FromStr; 
// Решаю не сканировать argv сам, использую clap.
use clap::Parser;   
// Для удобной конвертации даты и времени в секунды и обратно. 
use inquire::{self, Select};
use rusqlite::params;
use tasks::Task;
use chrono::prelude::*;


// каждый элемент перечисления хранит число параметров,
// нужное для проведение командаы.
enum ActionRequested { 
     Add    ,
     Remove , 
     Update ,
     Select   , 
     NoAct   
}

impl ActionRequested { 
/// Взвисимости от запрошенной команды  - 
/// нам может потребоваться разное кол-во 
/// сегментов. метод возвращает кол-во нужное
/// для запрошенной опеации, определённой
/// экземпляром варианта перечисления.
/// ### NOTE: счёт ведётся без учёта самой команды.
    pub fn segments_needded(&self ) -> usize { 
        match *self { 
            Self::Add    => 4,
            Self::Select => 2, // Всё, что после WHERE - передаётся без разбора.
            Self::Update => 1,
            Self::Remove => 1, 
            Self::NoAct =>  0, // Команда не определена, сегменты не считаюся.
            _ => 0 
        }
        
    }
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
     let conn = ConnHandler::load_db(path); //db::load_db(path);
  //   println!("{}", todo!());
     print!(" Welcome to TODO  list manager!");
     let sin = std::io::stdin();
     print!(
     "you can: \n\
        add todo (add name description date category). \n\
        Use quote marks to have multiple words per operands.\n\
        date shoukd be YYYY-MM-DD, also may be YYYY-MM-DD HH:MM \n\
        remove: remove name .  \n\
        update: (update namme); will start interactive mode. \n\
        show all: (select *) \n\
       To show filtered: 
       select where [predicate]: You can use date = 'YYYY-MM-DD HH:MM', 
       if on creation no task creation - 00:00 be used, you also may not 
       specify time. \n\
       predicate may be satus - 'On' or 'Done'
       YOU NEED TO USE '' OR \"\" TO WRAP MORE THHEN ONE WORD VALUES!!!
       other predicate: category='some'. 
       you can combine them by 'and' word. \n ");
       let mut sbuf: String = String::with_capacity(255); 
       // сегмент - целоая часть команды: сама команда или её
       // аргумент.
       //  segment'ы разделены проблеами, но если начинается с кавычек
       // - то пробелы игнорируются.
       let mut segment = String::with_capacity(20);
       // Содержит в себе разделёенные сегменты: команда, аргументы. 
       let mut vbuf: Vec<String> = Vec::with_capacity(255);
       let mut act = ActionRequested::NoAct;
       // Очень важно отслеживать, встртили мы 
       // кавычки или  нет. 
       // в целях гарантии постоянства переменной
       // между циклами - декларирууется перед циклоами, но
       //  не в одном из них.
       let mut in_quote = false; 
       'mainloop: loop { 
        print!(">"); 
        // на всякий слчай очистим все хранилища.
        vbuf.clear();
        sbuf.clear();
        segment.clear();
        act = ActionRequested::NoAct;
        sin.read_line(&mut sbuf).unwrap();
       // act = ActionRequested::NoAct;
        //NOTE: ручной перебор для отлова кавычек.
        //? Возможно, стоило воспользоваться regex? 
        'scanloop: for c in sbuf.chars() {
        // Мы ещё не узнали команду? .
        if c == '"' {
            in_quote ^= true; 
            continue 'scanloop;
        }
        if let ActionRequested::NoAct = act   {
            if c != ' ' || in_quote { 
                segment.push(c);
                continue 'scanloop;
            }
            if c == ' ' && !in_quote { // Вообще-то кавычки в команде быть не долно. 

                // Мы узнали первое слово запроса.
                // попробуем подобрать действие?
               act = match segment.trim().to_ascii_uppercase().as_str() { 
                "ADD"=> { segment.clear(); ActionRequested::Add},
                "REMOVE" => {segment.clear(); ActionRequested::Remove}
                "UPDATE" => {segment.clear(); ActionRequested::Update}
                "SELECT" => {segment.clear(); ActionRequested::Select}
                 _   =>{  print!("unknown command");
                          segment.clear(); 
                          continue 'scanloop;
                        } 
                
                };
            continue 'scanloop; }         

        }
        // Мы знаем запрошенную операцию и пытаемься
        // собрать информацию для 
        else {
         if (c != ' ' && c != '\n' )|| in_quote {
            segment.push(c);
            continue 'scanloop;
         }
         if (c == ' ' || c == '\n'  )&& !in_quote { 
            vbuf.push(std::mem::take(&mut segment) );  
            if vbuf.len() >= act.segments_needded() { 
                // Мы собрали сгменты для запрошенной операции.
                // нам здесь делать больше нечего.
                // переходим в mainloop.
                break 'scanloop
            }
         }              
        }
        
    }  
    // К этому моменту мы должны знать команду
    // и должны иметь набор даннных дляя неё.
    match &act { 
        ActionRequested::Add => { 
            if debug {
                println!("dbg: reached add call, segments are: \n {:?}", vbuf) 
            }
            //FIXME: Оверхед по работе с элементами из вектора.
            //FIXME: Небрежное получение и вызов to_owned.
            // Возможно, итеративный drain() лучше? 
            let name = vbuf.get(0).unwrap().to_owned(); 
            let dscr = vbuf.get(1).unwrap().to_owned(); 
            let date : NaiveDateTime; 
            date = match utils::get_timedate(vbuf.get(2).unwrap().as_str()) {
                Ok(dt) => dt, 
                Err(e) => { 
                    print!("Failed to parse date-time.");
                    if debug{print!("{:?}", e)}
                    vbuf.clear();
                    continue 'mainloop;
                }                
            };
            let cat = vbuf.get(3).unwrap().to_owned();
            match conn.commit_new(&Task::new(name, dscr,
                                  false ,date, cat)) { // завершённость задачи здесь не важна.
                                    Ok(_) => {
                                        print!("add sucess.");
                                        vbuf.clear();
                                    }
                                    Err(e) => { 
                                        println!("failed to add Task");
                                        if debug{ 
                                            
                                            println!("{:?}",e);
                                            vbuf.clear();
                                        }

                                    }
                                 } 

           }  
        ActionRequested::Remove => { 
         conn.remove(vbuf.get(0).unwrap());
         vbuf.clear();   
        }, 
        ActionRequested::Update => { 
            // Для реализации интерактивного режима
            // выбрана библиотека inquire.

            // для начала - попробуем взять оригинал
            // как структуру, нам это понадобится потому что
            //Все поля структуры будут рассматриваться как оригинал.
            // если пользователь не проедложит свой вариант - мы оставим существующий вариант.
            let mut task: Task = match conn.select_by_name_prep() { // Rows живёт столько же, сколько State,emt. 
               Ok(mut _t) => { 
                   match (_t.query(params![vbuf.get(0).unwrap()]).unwrap().next().unwrap()) {
                   Some(t) => {
                
                    let mut dt = t.get::<_,String>(4).unwrap();
                    Task::new(t.get(0).unwrap(),
                         t.get(1).unwrap(),
                          false,utils::get_timedate(dt.as_str()).unwrap(),
                          t.get(2).unwrap())
                   },
                    _ => { print!("no such task."); continue 'mainloop;  }
                   }
                          // FIXME: Слишком большое нагромаждение.
               },
                _ => {vbuf.clear(); 
                    act = ActionRequested::NoAct;  continue 'mainloop;}
            };
                               
                        // Мы не можем заменить имя. 

                       //   let prms = 
                       print!("Enter new feilds of task selected: \n");
                       let _ndescr = inquire::Text::new("new description")
                       .with_default(&task.descripion).prompt().unwrap(); // возможно - лучше было бы промпт editroe'а

                       let _ncat  = inquire::Text::new("Enter new category")
                       .with_default(&task.category).prompt().unwrap(); // NOTE: можно настроить autocomply, собрать из БД категории и предлагать их

                       let  __ndatetime = inquire::DateSelect::new("date:").prompt().unwrap(); // Нет промтпа DateTime. это не проблема.
                       // SQLite адекватно обрабатывает запросы на поиск по дате, если время отсуствует.
                       // тем не менее, Task и некоторые методы уже работают с NaiveDAteTIme
                       let _ndatetime = { 
                        let mut s = __ndatetime.format("%Y-%m-%d").to_string();
                        s.push_str(" 00:00");
                          chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M").unwrap() 
                       };
                       // обновим таску и отправим её в БД.
                       task.update(_ndescr, _ndatetime, _ncat);
                       conn.commit_update(task).unwrap();



        }

        _ => { vbuf.clear(); continue;}  
    }

    std::thread::sleep(std::time::Duration::from_millis(100))       
    }
}