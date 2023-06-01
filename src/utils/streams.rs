use actix_web::web;
use futures::stream::{Stream, StreamExt};
use mongodb::bson::Document;

pub fn cursor_stream(
    cursor: mongodb::Cursor<Document>,
) -> impl Stream<Item = Result<web::Bytes, mongodb::error::Error>> {
    cursor
        .map(|doc_result| {
            doc_result.map(|doc| web::Bytes::from(doc.to_string()))
        })
}