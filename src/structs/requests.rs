use serde::{Serialize, Deserialize};
use mongodb::{options::{FindOptions, Collation, Hint, WriteConcern, InsertOneOptions, InsertManyOptions}, bson::{self, doc, Document}};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MongoRequest {
    data_source: Option<String>,
    database: Option<String>,
    collection: String,
    filter: Option<Document>,
    projection: Option<Document>,
    sort: Option<Document>,
    limit: Option<i64>,
    skip: Option<u64>,
    collation: Option<Collation>,
    comment: Option<String>,
    hint: Option<Hint>,
    max: Option<Document>,
    min: Option<Document>,
    bypass_document_validation: Option<bool>,
    write_concern: Option<WriteConcern>,
    upsert: Option<bool>,
    ordered: Option<bool>,
    document: Option<Document>,
    documents: Option<Vec<Document>>,
}

impl MongoRequest {
    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn get_filter(&self) -> &Option<Document> {
        &self.filter
    }

    pub fn get_doc(&self) -> &Option<Document> {
        &self.document
    }

    pub fn get_docs(&self) -> &Option<Vec<Document>> {
        &self.documents
    }

    pub fn build_find_options(&self) -> FindOptions {
        let mut find_options = FindOptions::default();

        if let Some(limit) = self.limit {
            find_options.limit = Some(limit);
        }

        if let Some(skip) = self.skip {
            find_options.skip = Some(skip);
        }

        if let Some(sort) = &self.sort {
            find_options.sort = Some(sort.clone());
        }

        if let Some(projection) = &self.projection {
            find_options.projection = Some(projection.clone());
        }

        if let Some(collation) = &self.collation {
            find_options.collation = Some(collation.clone());
        }

        if let Some(comment) = &self.comment {
            find_options.comment = Some(comment.clone());
        }

        if let Some(hint) = &self.hint {
            find_options.hint = Some(hint.clone());
        }

        if let Some(max) = &self.max {
            find_options.max = Some(max.clone());
        }

        if let Some(min) = &self.min {
            find_options.min = Some(min.clone());
        }

        find_options
    }

    pub fn build_insert_one_opts(&self) -> InsertOneOptions {
        let mut insert_one_opts = InsertOneOptions::default();

        if let Some(bypass_document_validation) = self.bypass_document_validation {
            insert_one_opts.bypass_document_validation = Some(bypass_document_validation.clone());
        }

        if let Some(write_concern) = &self.write_concern {
            insert_one_opts.write_concern = Some(write_concern.clone());
        }

        if let (Some(_comment), Ok(bson_comment)) = (&self.comment, bson::to_bson(&self.comment)) {
            insert_one_opts.comment = Some(bson_comment);
        }

        insert_one_opts
    }

    pub fn build_insert_many_opts(&self) -> InsertManyOptions {
        let mut insert_many_opts = InsertManyOptions::default();

        if let Some(bypass_document_validation) = self.bypass_document_validation {
            insert_many_opts.bypass_document_validation = Some(bypass_document_validation.clone());
        }

        if let Some(write_concern) = &self.write_concern {
            insert_many_opts.write_concern = Some(write_concern.clone());
        }

        if let Some(ordered) = &self.ordered {
            insert_many_opts.ordered = Some(ordered.clone());
        }

        if let (Some(_comment), Ok(bson_comment)) = (&self.comment, bson::to_bson(&self.comment)) {
            insert_many_opts.comment = Some(bson_comment);
        }

        insert_many_opts
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindOneRequest {
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertManyRequest {
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertOneRequest {
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOneRequest {
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateManyRequest {
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOneRequest {
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOneRequest {
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteManyRequest {
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateRequest {
    
}