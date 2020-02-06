use actix_web::{HttpServer, App, Responder, get, HttpResponse};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct IndexObj {
    message: String,
}

#[get("/")]
async fn node() -> impl Responder {
    let current_dir = std::env::current_dir().unwrap();
    let mut cmd = Command::new("node");
    cmd.arg("node/index.js").arg("rust");
    cmd.current_dir(&current_dir);

    let output = cmd.output().expect("Failed to process node.");
    let message = String::from_utf8_lossy(&output.stdout).to_string();

    println!("{}", &current_dir.to_string_lossy());

    HttpResponse::Ok().json(IndexObj {
        message: message,
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app = || {
        App::new()
            .service(node)
    };
    let server = HttpServer::new(app);

    server.bind("127.0.0.1:8080")?.run().await
}
