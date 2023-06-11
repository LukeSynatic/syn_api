use actix_web::{Responder, HttpResponse, get, post, patch, delete, web::{scope, Data, Path, Json, ServiceConfig}};
use mongodb::{Database, bson::{self, doc, Document}, Collection};
use futures::{stream::StreamExt};
use serde_json::Value;

use crate::{utils::{streams::cursor_stream, mongo::collect_docs}};

pub fn configure_user_service(cfg: &mut ServiceConfig) {
    cfg.service(
    scope("/users")
                .service(get_users)
                .service(get_user_by_id)
                .service(post_user)
                .service(patch_user)
                .service(delete_user)
                .service(delete_users)
                .service(get_users_stream)
    );
}

#[get("")]
async fn get_users(db: Data<Database>) -> impl Responder {
    let collection = db.collection("users");
    if let Ok(cursor) = collection.find(None, None).await {
        let results: Vec<Result<Document, mongodb::error::Error>> = cursor.collect().await;
        HttpResponse::Ok().json(collect_docs(results))
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/{id}")]
async fn get_user_by_id(db: Data<Database>, path: Path<String>) -> impl Responder {
    let collection: Collection<Document> = db.collection("users");
    let id = path.into_inner();

    let query = doc! {
        "id": id
    };

    match collection.find_one(query, None).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/stream")]
async fn get_users_stream(db: Data<Database>) -> impl Responder {
    let collection = db.collection("users");

    let query = doc! {
        "deactivated": false,
    };

    if let Ok(cursor) = collection.find(query, None).await {
        HttpResponse::Ok().streaming(cursor_stream(cursor))
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("")]
async fn post_user(db: Data<Database>, body: Json<Value>) -> impl Responder {
    let collection = db.collection("users");
    if let Ok(query) = bson::to_document(&body.into_inner()) {
        match collection.insert_one(query, None).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[patch("/{id}")]
async fn patch_user(db: Data<Database>, body: Json<Value>, path: Path<String>) -> impl Responder {
    let collection: Collection<Document> = db.collection("users");
    let id = path.into_inner();

    let query = doc! {
        "id": id
    };

    if let Ok(bson_doc) = bson::to_bson(&body.into_inner()) {
        let bson_doc = doc! { "$set": bson_doc };

        match collection.update_one(query, bson_doc, None).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[delete("/{id}")]
async fn delete_user(db: Data<Database>, path: Path<String>) -> impl Responder {
    let collection: Collection<Document> = db.collection("users");
    let id = path.into_inner();

    let query = doc! {
        "id": id
    };

    match collection.delete_one(query, None).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[delete("")]
async fn delete_users(db: Data<Database>, body: Json<Value>) -> impl Responder {
    let collection: Collection<Document> = db.collection("users");

    if let Ok(query) = bson::to_document(&body.into_inner()) {
        match collection.delete_many(query, None).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}