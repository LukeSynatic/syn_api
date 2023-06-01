use actix_web::{Responder, HttpResponse, get, web::{scope, Data, ServiceConfig, self}};
use mongodb::{Database, bson::{doc, Document}};
use futures::{stream::StreamExt};

use crate::utils::{streams::cursor_stream, mongo::collect_docs};

pub fn configure_user_service(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
                .service(get_users)
                .service(get_users_stream)
    );
}

#[get("")]
async fn get_users(client: Data<Database>) -> impl Responder {
    let db = client.collection("users");

    let filter = doc! {
        "favorite_color": "red"
    };

    if let Ok(cursor) = db.find(filter, None).await {
        let results: Vec<Result<Document, mongodb::error::Error>> = cursor.collect().await;
        HttpResponse::Ok().json(collect_docs(results))
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/stream")]
async fn get_users_stream(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection("users");

    let filter = doc! {
        "deactivated": false
    };

    if let Ok(cursor) = collection.find(filter, None).await {
        HttpResponse::Ok().streaming(cursor_stream(cursor))
    } else {
        HttpResponse::InternalServerError().finish()
    }
}