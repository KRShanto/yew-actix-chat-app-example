#![allow(dead_code, unused)]

use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{middleware, web};
use actix_web::{App, HttpServer};

use backend::{
    actors::ChatServer,
    db::establish_connection,
    route_functions::{room_create, save_file, signup, ws_index},
};

#[actix_web::main]
async fn main() {
    std::fs::create_dir_all("./img").unwrap();
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
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .supports_credentials(),
            )
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            .service(web::resource("/upload-image").route(web::post().to(save_file)))
            .service(
                web::scope("/auth").route("/sign-up", web::post().to(signup)), // .route("", web::post().to(login)),
            )
            .service(web::resource("/create-room").route(web::post().to(room_create)))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .await
    .unwrap();
}
