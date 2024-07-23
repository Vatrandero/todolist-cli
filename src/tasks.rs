pub struct Note { 
    pub header : String, 
     is_complete : bool, 
    pub descripion : String,
    date_time : u64, // Дата чего? 
    category: String
}

impl Note { 
     /// NOTE: d - это кол-во секунд от UNIX_EPOCH. 
     pub fn new(head : String, desc: String, d: u64, cat: String  ) -> Self
     {
         Self {
            header: head, 
            is_complete:false, 
            descripion: desc, 
            date_time : d,
            category : cat                   

                                

         }
     }
    pub fn update(&mut self, nhead:String, ndesc:String, d : u64, catg :String  ) {
        
        self.header = nhead;
        self.descripion = ndesc;
        self.category = catg; 
        self.date_time = d;
    

    }
    pub fn done(&mut self ) {
        self.is_complete = true;
    }    
}