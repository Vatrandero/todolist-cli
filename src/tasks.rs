use chrono::{self, Local}; 

pub struct Task { 
    // FIXME: не все поля в начале были  публичными.
    // поэтому существует get_all().
    //TODO: заменить обработки get_all на непосредственное взятие полей.
    pub name : String, 
    pub is_complete : bool, 
    pub descripion : String,
    pub craation_date_time : chrono::NaiveDateTime,  
    pub category: String
}

impl Task { 
     /// NOTE: d - это кол-во секунд от UNIX_EPOCH. 
     pub fn new(head : String, desc: String, is_completed:bool, d: chrono::NaiveDateTime, cat: String  ) -> Self
     {
         Self {
            name: head, 
            is_complete:is_completed, 
            descripion: desc, 
            // не смотря на то, что в ТЗ 
            // дата рассмаривается как дата сздания - 
            // Она принимается в команде add, поэтому 
            // creation_date_time задаётся вручную, а не через 
            // конвертацию Systemtime через chrono. 
            craation_date_time : d,
            category : cat                   

                                
            
         }
     }
    pub fn update(&mut self,  ndesc:String, d : chrono::NaiveDateTime, catg :String  ) {
        
    
        self.descripion = ndesc;
        self.category = catg; 
        self.craation_date_time = d;
    

    }
    pub fn done(&mut self ) {
        self.is_complete = true;
    }    
/// Генерирует картеж, который передаётся как параметры для записи SQL.
    /// * 0 - название,
    /// * 1 - описание, тело задачи,
    /// * 2 - категория,
    /// * 3 = дата слхжания,
    #[deprecated(since="0.0.8", note="Все поля  структуры  стали публичными.")]
    pub fn get_all(&self) 
    -> (String, String, String, String, bool) {
        // Эьа функцияя существует потому, что не все поля являются

        return( 
            self.name.clone(), self.descripion.clone(),
            self.category.clone(), 
            self.craation_date_time.format("%Y-%m-%d %H:%M").to_string(), 
            self.is_complete.clone()
            
        )
    }

}
/// реалиация трейта Display выполнена так, что она пригода 
/// для вывода сразу пользователю. 
impl std::fmt::Display for Task { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if (self.is_complete){"Done"} else{"On"};
        write!(f, "Name: {} \n Description:\n {}; \n Status:{} \n datetime of creation:{} \n ",
         self.name, self.descripion, status,
         self.craation_date_time.format("%Y-%m-%d %H:%M").to_string())        
    }
}

