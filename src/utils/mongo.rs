use mongodb::{bson::{Document, self}};
use serde_json::Value;

pub fn collect_docs(results: Vec<Result<Document, mongodb::error::Error>>) -> Vec<Document> {
    let documents: Vec<Document> = results
            .into_iter()
            .filter_map(|result| result.ok())
            .collect();

    documents
}

pub fn parse_docs(json_list: &Vec<Value>) -> Result<Vec<Document>, bson::ser::Error> {
    if json_list.is_empty() {
        let result: Vec<Document> = vec![];
        return Ok(result);
    }

    json_list
        .clone()
        .into_iter()
        .map(|json| bson::to_document(&json))
        .collect()
}

pub fn parse_filter(json: &Value) -> Result<Document, bson::ser::Error> {
    match bson::to_document(json) {
        Ok(doc) => Ok(doc),
        Err(e) => Err(e.into()) 
    }
}