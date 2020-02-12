use actix_web::{get, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ResponseObj {
    message: String
}


#[get("/whoami")]
pub async fn whoami() -> impl Responder {
    HttpResponse::Ok().json(ResponseObj {
        message: "SUCCESS".to_string()
    })
}
