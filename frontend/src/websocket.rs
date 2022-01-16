// This module contains some functions for dealing with websocket;
// #![allow(dead_code, unused)]

use js_sys::JsString;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, InputEvent, MessageEvent, WebSocket};
use web_sys::{HtmlElement, HtmlInputElement};
use weblog::{console_error, console_log, console_warn};
use yew::prelude::*;
use yew::NodeRef;

pub fn ws_onmessage(ws: WebSocket) {
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
            console_log!("message event, received Text: {:?}", text.clone());
        } else {
            console_error!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();
}

pub fn ws_onerror(ws: WebSocket) {
    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_error!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();
}

pub fn ws_opopen(ws: WebSocket) {
    let ws_clone = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        // ************************************** //

        console_log!("socket opened");
        ws_clone.send_with_str("I've connected with you").unwrap();

        #[derive(Debug, Serialize, Deserialize)]
        struct Product {
            name: String,
            cost: usize,
        }

        ws_clone
            .send_with_str(
                &serde_json::to_string(&Product {
                    name: "Iphone 6^".to_owned(),
                    cost: 200_999,
                })
                .unwrap(),
            )
            .unwrap();

        // ************************************** //
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
}

pub fn ws_opclose(ws: WebSocket) {
    let onclose_callback = Closure::wrap(Box::new(move |_| {
        console_error!("Socket closed :(");
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
}
