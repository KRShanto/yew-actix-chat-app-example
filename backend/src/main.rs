#![allow(dead_code, unused)]

use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{middleware, web};
use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use backend::{
    actors::ChatServer,
    db::establish_connection,
    route_functions::{
        get_a_user, get_messages, get_rooms, get_users_from_room, make_join_request, room_create,
        save_file, signup, ws_index,
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./img").unwrap();
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok().unwrap();

    // set up database connection pool
    let dbconnection = std::env::var("DATABASE_URL").expect("DATABASE_URL not found!");

    let manager = ConnectionManager::<PgConnection>::new(dbconnection);

    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let server = ChatServer {
        addr_of_all_other_actors: Vec::new(),
    }
    .start();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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
            .service(web::resource("/get-rooms").route(web::post().to(get_rooms)))
            .service(web::resource("/get-messages").route(web::post().to(get_messages)))
            .service(web::resource("/get-user").route(web::post().to(get_a_user)))
            .service(web::resource("/room-join-request").route(web::post().to(make_join_request)))
            .service(
                web::resource("/get-users-from-room").route(web::post().to(get_users_from_room)),
            )
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .await
}
