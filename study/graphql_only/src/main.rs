extern crate dotenv;
extern crate env_logger;

use std::io;
use std::sync::Arc;

use actix_web::{
    middleware,
    web,
    App,
    Error,
    HttpResponse,
    HttpServer,
    post,
    get,
};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql_schema;
use crate::graphql_schema::{create_schema, Schema};

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(
        move || {
            let res = data.execute(&st, &());
            Ok::<_, serde_json::error::Error>
            (serde_json::to_string(&res)?)
        }
    )
    .await?;

    Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(user)
    )
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let schema = std::sync::Arc::new(create_schema());

    let app = move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(graphql)
            .service(graphiql)
    };

    HttpServer::new(app)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
