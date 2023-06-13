use actix_web::{Responder, HttpResponse, post, web::{scope, Data, Json, ServiceConfig}};
use mongodb::{Database, bson::Document, options::ReplaceOptions};
use futures::stream::StreamExt;

use crate::{structs::requests::{find::FindRequest, find_one::FindOneRequest, insert_one::InsertOneRequest, insert_many::InsertManyRequest, update::{UpdateRequest, UpdateOptionsWrapper}, delete::DeleteRequest, aggregate::AggregateRequest}, traits::requests::{MongoRequest, FilterQuery, DocumentPayload}, utils::mongo::{collect_docs}, middleware::ejson::EJsonV1};

pub fn configure_mongo_service(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/v1")
                    .wrap(EJsonV1)
                    .service(find)
                    .service(find_one)
                    .service(insert_one)
                    .service(insert_many)
                    .service(update_one)
                    .service(update_many)
                    .service(replace_one)
                    .service(delete_one)
                    .service(delete_many)
                    .service(aggregate)
    );
}

#[post("/find")]
async fn find(db: Data<Database>, json: Json<FindRequest>) -> impl Responder {
    let req = json.into_inner();

    let query = match req.filter() {
        Some(Ok(doc)) => Some(doc),
        Some(Err(_)) => {
            return HttpResponse::BadRequest().finish();
        },
        None => None
    };

    if let Ok(cursor) = db.collection(req.coll()).find(query, req.opts()).await
    {
        HttpResponse::Ok().json(collect_docs(cursor.collect().await))
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/findOne")]
async fn find_one(db: Data<Database>, json: Json<FindOneRequest>) -> impl Responder {
    let req = json.into_inner();

    let query = match req.filter() {
        Some(Ok(doc)) => Some(doc),
        Some(Err(_)) => {
            return HttpResponse::BadRequest().finish();
        },
        None => None
    };

    let doc: Result<Option<Document>, mongodb::error::Error> = db.collection(req.coll()).find_one(query, req.opts()).await;

    match doc {
        Ok(Some(result)) => HttpResponse::Ok().json(result),
        Ok(None) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("/insertOne")]
async fn insert_one(db: Data<Database>, json: Json<InsertOneRequest>) -> impl Responder {
    let req = json.into_inner();

    let body = match req.payload() {
        Ok(doc) => doc,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        } 
    };

    let result = db.collection(req.coll()).insert_one(body, req.opts()).await;

    match result {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        },
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[post("/insertMany")]
async fn insert_many(db: Data<Database>, json: Json<InsertManyRequest>) -> impl Responder {
    let req = json.into_inner();

    let body = match req.payload() {
        Ok(doc) => doc,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        } 
    };

    let result = db.collection(req.coll()).insert_many(body, req.opts()).await;

    match result {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        },
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[post("/updateOne")]
async fn update_one(db: Data<Database>, json: Json<UpdateRequest>) -> impl Responder {
    let req = json.into_inner();

    let query = match req.filter() {
        Some(Ok(doc)) => doc,
        Some(Err(_)) | None => {
            return HttpResponse::BadRequest().finish();
        }
    };

    let body = match req.payload() {
        Ok(doc) => doc,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        } 
    };

    let result = db.collection::<Document>(req.coll()).update_one(query, body, req.opts()).await;

    match result {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        },
        Err(e) => {
            println!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[post("/updateMany")]
async fn update_many(db: Data<Database>, json: Json<UpdateRequest>) -> impl Responder {
    let req = json.into_inner();

    let query = match req.filter() {
        Some(Ok(doc)) => doc,
        Some(Err(_)) | None => {
            return HttpResponse::BadRequest().finish();
        }
    };

    let body = match req.payload() {
        Ok(doc) => doc,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        } 
    };

    let result = db.collection::<Document>(req.coll()).update_many(query, body, req.opts()).await;

    match result {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        },
        Err(e) => {
            println!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[post("/replaceOne")]
async fn replace_one(db: Data<Database>, json: Json<UpdateRequest>) -> impl Responder {
    let req = json.into_inner();

    let query = match req.filter() {
        Some(Ok(doc)) => doc,
        Some(Err(_)) | None => {
            return HttpResponse::BadRequest().finish();
        }
    };

    let body = match req.payload() {
        Ok(doc) => doc,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        } 
    };

    let opts: Option<ReplaceOptions> = Some(UpdateOptionsWrapper(req.opts()).into());

    let result = db.collection::<Document>(req.coll()).replace_one(query, body, opts).await;

    match result {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        },
        Err(e) => {
            println!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[post("/deleteOne")]
async fn delete_one(db: Data<Database>, json: Json<DeleteRequest>) -> impl Responder {
    let req = json.into_inner();

    let query = match req.filter() {
        Some(Ok(doc)) => doc,
        Some(Err(_)) | None => {
            return HttpResponse::BadRequest().finish();
        }
    };

    let result = db.collection::<Document>(req.coll()).delete_one(query, req.opts()).await;

    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("/deleteMany")]
async fn delete_many(db: Data<Database>, json: Json<DeleteRequest>) -> impl Responder {
    let req = json.into_inner();

    let query = match req.filter() {
        Some(Ok(doc)) => doc,
        Some(Err(_)) | None => {
            return HttpResponse::BadRequest().finish();
        }
    };

    let result = db.collection::<Document>(req.coll()).delete_many(query, req.opts()).await;

    match result {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        },
        Err(e) => {
            println!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[post("/aggregate")]
async fn aggregate(db: Data<Database>, json: Json<AggregateRequest>) -> impl Responder {
    let req = json.into_inner();

    let pipeline = match req.payload() {
        Ok(p) => p,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        }
    };

    println!("{:?}", pipeline);

    let result = db.collection::<Document>(req.coll()).aggregate(pipeline, req.opts()).await;
    
    match result {
        Ok(cursor) => HttpResponse::Ok().json(collect_docs(cursor.collect().await)),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}