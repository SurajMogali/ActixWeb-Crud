// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};

// #[derive(Debug,Serialize,Deserialize,Clone)]
// pub struct CommonFields{
//     pub created_by:Option<String>,

//     pub last_modified_by:Option<String>,

//     pub created_date:Option<DateTime<Utc>>,

//     pub last_modified_date:Option<DateTime<Utc>>,
    
// }

// impl CommonFields {
//     pub fn new(created_by:Option<String>)->Self{
//         let now =Utc::now();
//         CommonFields {
//             created_by,
//             last_modified_by:None,
//             created_date:Some(now),
//             last_modified_date:Some(now),
//         }

//     }


//     pub fn update_common_fields(&mut self,modified_by:Option<String>){
//         self.last_modified_by=modified_by;
//         self.last_modified_date=Some(Utc::now());
//     }
// }