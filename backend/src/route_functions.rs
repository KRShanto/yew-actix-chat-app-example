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
    db::{create_user, establish_connection},
};

// *************** User's info comes from json body; ***************** //
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    username: String,
    password: String,
    nickname: String,
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
            if let Some(text) = e {
                web::Json(None)
            } else {
                web::Json(None)
            }
        }
        Ok(user) => {
            println!("user: {:?}", user);
            web::Json(Some(user))
            // TODO: Now I am returning the `password` also. But later I will not return password
        }
    }
    // TODO: I will return better error messages later.
}
