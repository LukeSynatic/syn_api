mod structs {
    pub mod responses;
    pub mod user;
    pub mod requests {
        pub mod find;
        pub mod find_one;
        pub mod insert_one;
        pub mod insert_many;
        pub mod update;
        pub mod delete;
        pub mod aggregate;
    }
}

mod traits {
    pub mod requests;
}

mod routes {
    pub mod stats_routes;
    pub mod user_routes;
    pub mod mongo_routes;
}

mod middleware {
    pub mod ejson;
}

mod utils {
    pub mod mongo;
    pub mod streams;
}

use std::env;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use mongodb::Client;
use mongodb::options::ClientOptions;
use dotenv::dotenv;

use crate::routes::mongo_routes::configure_mongo_service;
use crate::routes::stats_routes::configure_stats_service;
use crate::routes::user_routes::configure_user_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Configure mongodb client
    let mongo_uri = env::var("MONGODB_URI").unwrap();
    let db_name = env::var("DB_NAME").unwrap();
    let client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&db_name);

    /*  
        Server configuration and start:

        We need to call move on the closure passed to the server builder to move the ownership
        MongoDB client in to the proper scope. The client is then bound to application data, 
        making it accessible in every handler.

        We then configure our routers by passing their config functions to configure()
    */
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    println!("🚀 Server listening on port {}", port);
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(db.clone()))
            .configure(configure_stats_service)
            .configure(configure_user_service)
            .configure(configure_mongo_service)
    }).bind(("127.0.0.1", port))?
     .run()
     .await

}