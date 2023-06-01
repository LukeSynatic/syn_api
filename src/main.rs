mod structs {
    pub mod responses;
}

mod routes {
    pub mod stats_routes;
    pub mod user_routes;
}

mod utils {
    pub mod mongo;
    pub mod streams;
}

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use mongodb::Client;
use mongodb::options::ClientOptions;

use crate::routes::stats_routes::configure_stats_service;
use crate::routes::user_routes::configure_user_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure mongodb client
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("synatic-mongodb-api");

    /*  
        Server configuration and start:

        We need to call move on the closure passed to the server builder to move the ownership
        MongoDB client in to the proper scope. The client is then bound to application data, 
        making it accessible in every handler.

        We then configure our routers by passing their config functions to configure()
    */
    println!("🚀 Server listening on port 8080");
    HttpServer::new(move || 
        App::new()
            .app_data(Data::new(db.clone()))
            .configure(configure_stats_service)
            .configure(configure_user_service)
    ).bind(("127.0.0.1", 8080))?
     .run()
     .await

}