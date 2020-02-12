#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;

use actix_web::{
    App, HttpServer,
    middleware::{Logger}
};
use dotenv::dotenv;

mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    env_logger::init();

    let url = "127.0.0.1:8080";

    let app = || {
        App::new()
            .service(routes::whoami)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-agent}i"))
    };

    info!("Running server on {}", url);

    HttpServer::new(app)
        .bind(url)?
        .run()
        .await
}
