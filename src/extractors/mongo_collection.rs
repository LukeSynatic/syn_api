use actix_web::{FromRequest, HttpRequest, dev::Payload, web::Data};
use futures::{future::{ready, Ready}, TryFutureExt};
use mongodb::{Database, Collection, bson::Document};
use serde_json::Value;

pub struct MongoCollection<'a> {
    database: &'a Database,
    collection_name: String,
}

impl<'a> FromRequest for MongoCollection<'a> {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let app_data = req.app_data::<Data<Database>>();
        match app_data {
            Some(database) => {
                let fut = async move {
                    let body = actix_web::web::Bytes::from_request(req, payload).await?;
                    let json: Value = serde_json::from_slice(&body)?;
                    let collection_name = json
                        .get("collection")
                        .and_then(|value| value.as_str())
                        .unwrap_or("default_collection")
                        .to_string();

                    Ok(MongoCollection {
                        database: &database,
                        collection_name,
                    })
                }.map_err(actix_web::error::ErrorInternalServerError);
                ready(fut)
            }
            None => ready(Err(actix_web::error::ErrorInternalServerError(
                "MongoDB database not found in AppData",
            ))),
        }
    }
}

impl<'a> MongoCollection<'a> {
    pub fn get_collection(&self) -> Collection<Document> {
        self.database.collection(&self.collection_name)
    }
}