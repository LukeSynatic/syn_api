// use std::{error::Error, fmt::{self, Display, Formatter}, pin::Pin};

// use actix_web::{dev::Payload, FromRequest, web::{Bytes, block, Json, self, BytesMut}, HttpRequest, ResponseError, Either};
// use futures::{future::{BoxFuture, LocalBoxFuture, Ready, ready}, Future};
// use serde::de::DeserializeOwned;
// use serde_json::Value;

// #[derive(Debug)]
// struct PayloadSerializationError {
//     message: String,
// }

// impl Error for PayloadSerializationError {}

// impl Display for PayloadSerializationError {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }

// #[derive(Debug)]
// struct PayloadError {
//     message: String,
// }

// impl Error for PayloadError {}
// impl ResponseError for PayloadError {}
// impl ResponseError for PayloadSerializationError {}

// impl Display for PayloadError {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }

// pub struct EJsonExtractor(pub Value);

// impl FromRequest for EJsonExtractor {
//     type Error = actix_web::Error;
//     type Future = Pin<Box<dyn futures::Future<Output = Result<Self, Self::Error>>>>;

//     fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
//         // Check if the content-type header is set to "application/ejson"
//         if let Some(content_type) = req.headers().get("content-type") {
//             if content_type == "application/ejson" {
//                 // Read the request body and transform it into serde_json::Value
//                 let mut bytes = Bytes::new();
//                 let fut = payload
//                     .take_while(|chunk| {
//                         let chunk = chunk.map_err(PayloadError::Io)?;
//                         bytes.extend_from_slice(&chunk);
//                         futures::future::ready(Ok(bytes.len() < 1_048_576)) // Limit the payload size to 1 MB
//                     })
//                     .collect::<Result<Bytes, PayloadError>>()
//                     .map(move |result| {
//                         result
//                             .and_then(|bytes| {
//                                 let json_string = String::from_utf8(bytes.to_vec())?;
//                                 let json_value: Value = serde_json::from_str(&json_string)?;
//                                 Ok(EJsonExtractor(json_value))
//                             })
//                             .map_err(|err| actix_web::error::ErrorBadRequest(err))
//                     });

//                 Box::pin(fut)
//             } else {
//                 // If the content-type is not "application/ejson", return an error
//                 Box::pin(async move {
//                     Err(actix_web::error::ErrorBadRequest("Invalid content-type"))
//                 })
//             }
//         } else {
//             // If the content-type header is not present, return an error
//             Box::pin(async move {
//                 Err(actix_web::error::ErrorBadRequest("Missing content-type header"))
//             })
//         }
//     }
// }

// // impl EJson {
// //     async fn extract(req: &HttpRequest, payload: &mut Payload) -> Result<Self, PayloadSerializationError> {
// //         let bytes = Bytes::from_request(req, payload).await.unwrap_or(Bytes::default());
// //         let value = match serde_json::from_slice(&bytes) {
// //             Ok(v) => v,
// //             Err(_) => {
// //                 return Err(PayloadSerializationError {
// //                     message: "Something went wrong.".to_string(),
// //                 })
// //             }
// //         };

// //         let content_type = req
// //             .headers() 
// //             .iter()
// //             .find(|h| h.0 == "ContentType");

// //         if let Some(header) = content_type {
// //             match header.1.to_str() {
// //                 Ok("application/json") | Ok("application/ejson") => {
// //                     Ok(EJson { payload: value })
// //                 },
// //                 _ => Err(PayloadSerializationError {
// //                     message: "Something went wrong.".to_string(),
// //                 })
// //             }
// //         } else {
// //             Err(PayloadSerializationError {
// //                 message: "Something went wrong.".to_string(),
// //             })
// //         }
// //     }

// //     fn payload(&self) -> &Value {
// //         &self.payload
// //     }
// // }

// // impl FromRequest for EJson {
// //     type Error = PayloadSerializationError;
// //     type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

// //     fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
// //         Box::pin(web::block(move || EJson::extract(req, payload)))
// //     }

// //     fn extract(req: &HttpRequest) -> Self::Future {
// //         Self::from_request(req, &mut Payload::None)
// //     }
// // }