// Some Functions for routes.
#![allow(dead_code, unused)]
use actix::prelude::*;
use actix_multipart::Multipart;
use actix_web::{http::StatusCode, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use diesel::pg::PgConnection;
use futures_util::TryStreamExt as _;
use serde::{Deserialize, Serialize};
use std::io::Write;
use uuid::Uuid;

use crate::{
    actors::{ChatServer, ChatSession},
    db::{add_user_into_room, create_room, create_user, establish_connection},
};

// *************** User's info comes from json body; ***************** //
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    username: String,
    password: String,
    nickname: String,
    img_url: String,
}

// *************** Room's info comes from json body; ***************** //
#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    user_id: i32, // user's id. It is needed because the user will be added in this room initially;
    nickname: String, // room's nickname
    img_url: String,
}

// ************************************************************************* //
// ######################## Websocket connection ########################### //
// ************************************************************************* //
pub async fn ws_index(
    request: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    println!("request: {:?}", request);
    let response = ws::start(ChatSession::new(server.get_ref().clone()), &request, stream);

    println!("response: {:?}", response);

    response
}

// ************************************************************************* //
// ######################## Saveing a file ################################# //
// ************************************************************************* //
pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;

        let filename = content_disposition.get_filename().map_or_else(
            || Uuid::new_v4().to_string(),
            |f| sanitize_filename::sanitize(f),
        );

        let filepath = format!("./img/{}", filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
        }
    }

    Ok(HttpResponse::Ok().into())
}

// ************************************************************************* //
// ######################### Creating user account ######################### //
// ************************************************************************* //
pub async fn signup(request: HttpRequest, user_info: web::Json<UserInfo>) -> impl Responder {
    let result = create_user(
        establish_connection(),
        user_info.username.clone(),
        user_info.password.clone(),
        user_info.nickname.clone(),
        user_info.img_url.clone(),
    );

    match result {
        Err(e) => {
            if let Some(error_msg) = e {
                web::Json(None).with_status(StatusCode::CONFLICT)
            } else {
                web::Json(None).with_status(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Ok(user) => {
            println!("user: {:?}", user);
            web::Json(Some(user)).with_status(StatusCode::OK)
            // TODO: Note: Now I am returning the `password` also. But later I will not return password
        }
    }
    // TODO: I will return better error messages later.
}

// ************************************************************************* //
// ######################### Create new Room ######################### //
// ************************************************************************* //
// TODO: I will see how much performance difference without using r2d2 and with using r2d2
pub async fn room_create(room_info: web::Json<RoomInfo>) -> impl Responder {
    let create_room_result = create_room(
        &establish_connection(),
        room_info.nickname.clone(),
        room_info.img_url.clone(),
    );

    match create_room_result {
        Ok(room) => {
            let result_adding_user_in_room =
                add_user_into_room(room_info.user_id, room.id, establish_connection(), true);

            match result_adding_user_in_room {
                Ok(_) => HttpResponse::Ok(),
                Err(fake_error) => {
                    if let Some(_ignore_this_message) = fake_error {
                        // User is not present
                        println!("User is not present so returning BadReqeust");
                        HttpResponse::BadRequest()
                    } else {
                        // Server-Side error
                        HttpResponse::InternalServerError()
                    }
                }
            }
        }
        Err(error) => {
            println!("{}", error);
            HttpResponse::InternalServerError()
        }
    }

    // ""
}
