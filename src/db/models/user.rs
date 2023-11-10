use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

pub const COLL_NAME: &str = "users"; // 集合名称

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<ObjectId>,
    pub create_time: String,
    pub update_time: String,
    pub user_name: String,
    pub birthday: String,
    pub photo: String,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub personal_profile: String,
    pub password: String,
    pub sex: i8,
}

trait UserTrait {
    
}

impl UserTrait for User {
    
}