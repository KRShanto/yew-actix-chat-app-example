// Some Functions for routes.

use actix::prelude::*;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use futures_util::TryStreamExt as _;
use std::io::Write;
use uuid::Uuid;

use crate::actors::{ChatServer, ChatSession};

// ######################## Websocket connection ########################### //
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

// ######################## Saveing a file ################################# //
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
