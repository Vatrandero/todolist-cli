pub struct Task { 
    pub header : String, 
     is_complete : bool, 
    pub descripion : String,
    craation_date_time : u64,  
    category: String
}

impl Task { 
     /// NOTE: d - это кол-во секунд от UNIX_EPOCH. 
     pub fn new(head : String, desc: String, d: u64, cat: String  ) -> Self
     {
         Self {
            header: head, 
            is_complete:false, 
            descripion: desc, 
            craation_date_time : d,
            category : cat                   

                                

         }
     }
    pub fn update(&mut self, nhead:String, ndesc:String, d : u64, catg :String  ) {
        
        self.header = nhead;
        self.descripion = ndesc;
        self.category = catg; 
        self.craation_date_time = d;
    

    }
    pub fn done(&mut self ) {
        self.is_complete = true;
    }    
}