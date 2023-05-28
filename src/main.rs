mod structs {
    pub mod responses;
}

mod routes {
    pub mod health_check_routes;
}

use actix_web::{App, HttpServer};
use mongodb::options::ClientOptions;
use mongodb::Client;

use crate::routes::health_check_routes::health_check_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure mongodb client
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    // Start server
    println!("🚀 Server listening on port 8080");
    HttpServer::new(move || App::new().app_data(client.clone()).service(health_check_handler))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}