#![allow(dead_code, unused)]
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{FormData, Request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::WebSocket;
use web_sys::{Element, HtmlDivElement, HtmlElement, HtmlInputElement};
use weblog::{console_log, console_warn};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HighlightProps {
    pub children: Children,
}

// This component will highlight the child elements.
// TODO: I will provide more details later
// This component can be called by any component
#[function_component(Highlight)]
pub fn highligh(props: &HighlightProps) -> Html {
    // This component will make its child components highlighted. That means it will make every element's opacity to a low value but the child's one will be high(in future)
    // TODO: This component isn't complete. I will complete later;
    let div_ref = NodeRef::default();
    let unique_class_name = Uuid::new_v4().to_string();

    {
        let div_ref = div_ref.clone();
        let unique_class_name = unique_class_name.to_string();
        use_effect_with_deps(move |_| || (), ());
    }

    html! {
        <>

        <div ref={div_ref.clone()} id="Highlight-component">
            {props.children.clone()}
        </div>

        </>
    }
}
