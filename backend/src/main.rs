use actix::prelude::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_session::CookieSession;
use actix_web::cookie::SameSite;
use actix_web::{middleware, web};
use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use backend::{
    actors::ChatServer,
    route_functions::{
        check_user_account, get_a_user, get_messages, get_rooms, get_users_from_room,
        make_join_request, room_create, save_file, show_join_requests, signup,
        validate_user_account, ws_index,
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

    // PostgreSQL connnection manager
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
                // TODO: I will not use different domain for production. In production I will serve frontend files using the actix-files
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials(),
            )
            .wrap(middleware::Logger::default())
            .wrap(
                CookieSession::signed(&[0; 32])
                    .secure(true)
                    .name("yewchat")
                    .max_age(120)
                    .same_site(SameSite::Strict),
            )
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            .service(web::resource("/upload-image").route(web::post().to(save_file)))
            .service(
                web::scope("/auth")
                    .route("/sign-up", web::post().to(signup))
                    .route("/login", web::post().to(validate_user_account))
                    .route("/check-user", web::post().to(check_user_account)),
            )
            .service(web::resource("/create-room").route(web::post().to(room_create)))
            .service(web::resource("/get-rooms").route(web::post().to(get_rooms)))
            .service(web::resource("/get-messages").route(web::post().to(get_messages)))
            .service(web::resource("/get-user").route(web::post().to(get_a_user)))
            .service(web::resource("/room-join-request").route(web::post().to(make_join_request)))
            .service(
                web::resource("/get-users-from-room").route(web::post().to(get_users_from_room)),
            )
            .service(web::resource("/get-join-requests").route(web::post().to(show_join_requests)))
            .service(Files::new("/get-user-image", "img/"))
        // .service(
        //     // fronted
        //     Files::new("/", "dist/")
        //         .prefer_utf8(true)
        //         .index_file("index.html"),
        // )
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .await
}
