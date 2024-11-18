use crate::model::{user::{self, User}};
use anyhow::{bail, Result};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{ClientOptions, FindOneAndUpdateOptions, ReturnDocument},
    Client, Collection,
};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone)]
pub struct UserService {
    pub collection: Collection<User>,
}

impl UserService {
    pub async fn new(database_url: &str) -> Self {
        let client_options = ClientOptions::parse(database_url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("test_db");
        let collection = db.collection::<User>("users");

        UserService { collection }
    }

    //Post Operation
    pub async fn create_user(&self, mut new_user: User) -> Result<User> {
        if let Err(validation_errors) = new_user.validate() {
            bail!(format!("Validation error : {}", validation_errors));
        }

        let filter = doc! { "email": &new_user.email };
        if self.collection.find_one(filter, None).await?.is_some() {
            bail!("User already exists");
        }



        if new_user.user_id.is_none() {
            new_user.user_id = Some(Uuid::new_v4().to_string());
        }

        // If not exists, proceed with insertion

      

        let result = self.collection.insert_one(new_user.clone(), None).await?;
        new_user.id = result.inserted_id.as_object_id();
        Ok(new_user)
    }

    //Get Operation
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>> {
        // let user_id=ObjectId::parse_str(id)?;
        let filter = doc! {"user_id":user_id};
        let user = self.collection.find_one(filter, None).await?;
        Ok(user)
    }

    //Update Operation
    pub async fn update_user(&self, user_id: &str, updated_user: User) -> Result<Option<User>> {
        let filter = doc! {"user_id":user_id};
        let update = doc! {
            "$set":{
                "name":updated_user.name,
                "email":updated_user.email,
                "age":updated_user.age,

            }



        };

        let options=FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

        let result = self
            .collection
            .find_one_and_update(filter, update, options)
            .await?;
        Ok(result)
    }

    //Delete Operation
    pub async fn delete_user(&self, user_id: &str) -> Result<Option<User>> {
        let filter = doc! {"user_id":user_id};
        let result = self.collection.find_one_and_delete(filter, None).await?;
        Ok(result)
    }

    //Get All Operation
    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let mut fetchUsers = self.collection.find(None, None).await?;
        let mut users = Vec::new();
        while let Some(user) = fetchUsers.try_next().await? {
            users.push(user);
        }

        Ok(users)
    }


}
