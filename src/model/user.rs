use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use validator_derive::Validate;




// User model
#[derive(Debug, Serialize, Deserialize, Clone,Validate)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,


    pub user_id:Option<String>,

    #[validate(email(message="Invalid email format"))]
    pub email: String,

    #[validate(length(min=1,message="Name cannot be empty"))]
    pub name: String,

    #[validate(range(min=18,message="Age must be atleast 18"))]
    pub age: i32,


    //pub common_fields:CommonFields,
}





