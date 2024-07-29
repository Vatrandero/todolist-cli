use chrono; 

pub struct Task { 
    pub name : String, 
     is_complete : bool, 
    pub descripion : String,
    craation_date_time : i64,  
    category: String
}

impl Task { 
     /// NOTE: d - это кол-во секунд от UNIX_EPOCH. 
     pub fn new(head : String, desc: String, d: i64, cat: String  ) -> Self
     {
         Self {
            name: head, 
            is_complete:false, 
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
    pub fn update(&mut self, nhead:String, ndesc:String, d : i64, catg :String  ) {
        
        self.name = nhead;
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
    pub fn get_all(&self) 
    -> (String, String, String, i64, bool) {
        // Эьа функцияя существует потому, что не все поля являются

        return( 
            self.name.clone(), self.descripion.clone(),
            self.category.clone(), self.craation_date_time, 
            self.is_complete.clone()
            
        )
    }

}
/// реалиация трейта Display выполнена так, что она пригода 
/// для вывода сразу пользователю. 
impl std::fmt::Display for Task { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if (self.is_complete){"Done"} else{"On"};
        write!(f, "Name: {} \n Description:\n {}; \n Status:{} \n Time of creation:{}   ",
         self.name, self.descripion, status,
         chrono::DateTime::from_timestamp(self.craation_date_time, 0).unwrap()   )        
    }
}

