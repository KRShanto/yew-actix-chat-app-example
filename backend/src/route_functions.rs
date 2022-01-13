// Some Functions for routes.

use actix::prelude::*;
use actix_web::HttpResponse;
use actix_web::{web, Error, HttpRequest};
use actix_web_actors::ws;

use crate::actors::{ChatServer, ChatSession};

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
