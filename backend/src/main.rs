// #![allow(dead_code, unused)]

use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{middleware, web};
use actix_web::{App, HttpServer};

mod actors;
mod route_functions;

use actors::ChatServer;
use route_functions::ws_index;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let server = ChatServer {
        addr_of_all_other_actors: Vec::new(),
    }
    .start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .supports_credentials(),
            )
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .await
    .unwrap();
}
