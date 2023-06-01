use mongodb::bson::Document;

pub fn collect_docs(results: Vec<Result<Document, mongodb::error::Error>>) -> Vec<Document> {
    let documents: Vec<Document> = results
            .into_iter()
            .filter_map(|result| result.ok())
            .collect();
    
    documents
}