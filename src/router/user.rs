/// 用户
/// request: https://actix.rs/docs/request
/// response: https://actix.rs/docs/response

use serde::{Deserialize, Serialize};
use serde_json::json;
use mongodb::{
    bson::{
        doc,
        oid:: {
            ObjectId
        }
    },
    options::IndexOptions,
    Client,
    Collection,
    IndexModel
};
use actix_web::{
    get,
    post,
    web,
    App,
    HttpResponse, 
    HttpServer,
    http::header::ContentType,
    Responder
};

use crate::db::{
    models,
    DB_NAME,
};

/// Creates an index on the "username" field to force the values to be unique.
/* async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
} */

/// Adds a new user to the "users" collection in the database.
#[post("/add_user")]
pub async fn add_user(client: web::Data<Client>, form: web::Form<models::user::User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(models::user::COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("success"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Gets the user with the supplied id.
#[get("/get_user/{id}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let id = username.into_inner();
    let collection: Collection<models::user::User> = client.database(DB_NAME).collection(models::user::COLL_NAME);
    match collection.find_one(doc! { "id": &id }, None).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        // Ok(Some(user)) => HttpResponse::Ok().content_type(ContentType::plaintext()).body(json!(user).to_string()),
        // Ok(Some(user)) => Ok(web::Json(user)),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {id}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}