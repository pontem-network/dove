use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use futures::StreamExt;
use dovecote::State;
use proto::OnRequest;

async fn rpc(mut body: web::Payload, state: web::Data<State>) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item.unwrap();
        bytes.extend_from_slice(&item);
    }
    let resp = web::block(move || state.rpc.handle(bytes.as_ref())).await?;
    Ok(HttpResponse::Ok().body(resp))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    env_logger::init();

    let state = State::new().unwrap();

    HttpServer::new(move || {
        let state = state.clone();
        App::new()
            .wrap(middleware::Logger::default())
            .data(state)
            .service(web::resource("/api/rpc").route(web::post().to(rpc)))
            .service(fs::Files::new("/", "dovecote/client/static/").index_file("index.html"))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
