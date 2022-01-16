// Some Functions for routes.
#![allow(dead_code, unused)]
use actix::prelude::*;
use actix_multipart::Multipart;
use actix_web::{http::StatusCode, web, web::Json, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use diesel::pg::PgConnection;
use futures_util::TryStreamExt as _;
use serde::{Deserialize, Serialize};
use std::io::Write;
use uuid::Uuid;

use crate::{
    actors::{ChatServer, ChatSession},
    db::{
        add_user_into_room, create_room, create_user, establish_connection,
        get_all_rooms_for_a_user, is_room_present, is_user_present,
    },
    models::Room,
};

// *************** User's id comes from json body; ***************** //
#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    user_id: i32,
}

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

// *************** Room's info send to client; ***************** //
#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfoForClient {
    id: i32,          // room id
    user_id: i32,     // user's id.;
    nickname: String, // room's nickname
    img_url: String,  // room's image's url
}
// *************** Vector of Room send to client; ***************** //
#[derive(Debug, Serialize, Deserialize)]
pub struct ListOfRoom {
    rooms: Vec<Room>,
}

// *************** Some info when sending a join request ***************** //
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinRoomRequest {
    user_id: i32, // user's id
    room_id: i32, // room's id
}

// ************************************************************************* //
// ######################## Websocket connection ########################### //
// ************************************************************************* //
pub async fn ws_index(
    request: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
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
pub async fn signup(request: HttpRequest, user_info: Json<UserInfo>) -> impl Responder {
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
                Json(None).with_status(StatusCode::CONFLICT)
            } else {
                Json(None).with_status(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Ok(user) => {
            println!("user: {:?}", user);
            Json(Some(user)).with_status(StatusCode::OK)
            // TODO: Note: Now I am returning the `password` also. But later I will not return password
        }
    }
    // TODO: I will return better error messages later.
}

// ************************************************************************* //
// ######################### Create new Room ######################### //
// ************************************************************************* //
// TODO: I will see how much performance difference without using r2d2 and with using r2d2
pub async fn room_create(room_info: Json<RoomInfo>) -> impl Responder {
    let create_room_result = create_room(
        &establish_connection(),
        room_info.nickname.clone(),
        room_info.img_url.clone(),
    );

    match create_room_result {
        Ok(room) => {
            let result_of_adding_user_in_room =
                add_user_into_room(room_info.user_id, room.id, establish_connection(), true);

            match result_of_adding_user_in_room {
                Ok(_) => {
                    let room_for_client = Room {
                        nickname: room.nickname,
                        id: room.id,
                        img_url: room_info.img_url.clone(),
                    };
                    Json(Some(room_for_client)).with_status(StatusCode::OK)
                }
                Err(fake_error) => {
                    if let Some(_ignore_this_message) = fake_error {
                        // User is not present
                        println!("User is not present so returning BadReqeust");
                        web::Json(None).with_status(StatusCode::BAD_REQUEST)
                    } else {
                        // Server-Side error
                        Json(None).with_status(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            }
        }
        Err(error) => {
            println!("{}", error);
            Json(None).with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ************************************************************************* //
// ##################### Make Join request to a Room ####################### //
// ************************************************************************* //
pub async fn make_join_request(info: Json<JoinRoomRequest>) -> impl Responder {
    // check if the room is valid or not
    // TODO: If any user already joined to that room or have request pending to that room, then he/she will get a message. I will make that restriction later.
    if let Ok(value) = is_room_present(info.room_id, &establish_connection()) {
        match value {
            true => {
                // Adding user in the room with `accepted` = false
                if let Err(value) =
                    add_user_into_room(info.user_id, info.room_id, establish_connection(), false)
                {
                    match value {
                        Some(_) => {
                            // TODO: user not found; I will handle this later. For now I am just returning BadReqeust
                            HttpResponse::BadRequest()
                        }
                        None => {
                            // Server side error;
                            HttpResponse::InternalServerError()
                        }
                    }
                } else {
                    HttpResponse::Ok()
                }
            }
            false => {
                HttpResponse::NoContent() // 204 http status;
                                          // TODO: For now I am just returning this response, Later I might be considering returning any other response
            }
        }
    } else {
        HttpResponse::InternalServerError()
    }
}

// ************************************************************************* //
// ########################### Get all Room ################################ //
// ************************************************************************* //
pub async fn get_rooms(user_info: Json<UserId>) -> impl Responder {
    // Check if the user is available or not! The client shouldn't send this user's id where the user is not available
    if let Ok(value) = is_user_present(user_info.user_id, &establish_connection()) {
        match value {
            true => {
                let rooms = get_all_rooms_for_a_user(user_info.user_id, establish_connection());

                Json(Some(rooms)).with_status(StatusCode::OK)
            }
            false => Json(None).with_status(StatusCode::BAD_REQUEST),
        }
    } else {
        Json(None).with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
