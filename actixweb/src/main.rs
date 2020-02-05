use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse, get};
use listenfd::{ListenFd};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

#[get("/{id}/{name}/index.html")]
async fn index(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    let id = req.match_info().get("id").unwrap_or("None");

    format!("Hello {}! id: {}", &name, &id)
}

#[get("/hello/{name}")]
async fn hello(obj: web::Path<MyObj>) -> impl Responder {
    HttpResponse::Ok().json(MyObj {
        name: obj.name.to_string()
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app = || {
        App::new()
            .service(index)
            .service(hello)
            .service(
                web::scope("/api")
                    .service(hello)
            )
    };

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(app);

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}