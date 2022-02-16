use reqwasm::http::{Request, RequestCredentials, RequestMode};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::chat_app::{server_url, LoginRender, User};

// Status of the user
#[derive(Clone, PartialEq)]
enum AuthStatus<T> {
    // The user is logged in. T will store the user's value
    LoggedIn(T),
    // The user is not logged in
    NotLoggedIn,
    // The request is pending. It means the request has sent but it is still not coming
    RequestPending,
}

#[derive(Properties, PartialEq)]
pub struct AuthProps {
    // childrens to be rendered if the user is logged in
    pub children: Children,
    // ```Login``` component renderer. If the user is not logged in then this component will be render
    pub login_render: UseStateHandle<LoginRender>,
}

// Component for checking if the user is logged in or not
// This component will request to the server to check if the user is logged in or not and if the user is not logged in then it will render the ```ChatApp``` component. Otherwise it will render a ```Login``` component
// This component is called by the ```App``` component
#[function_component(Auth)]
pub fn auth(props: &AuthProps) -> Html {
    // status of the user
    let user_status: UseStateHandle<AuthStatus<User>> = use_state(|| AuthStatus::RequestPending);

    let login_render = props.login_render.clone();

    // On the first render of this component, fetch the url "auth/check-user" and see whether the user is logged in or not
    {
        let user_status = user_status.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    // making a post request at the route "auth/check-user" to check if the user is logged in or not
                    let resp = Request::post(&server_url(Some("auth/check-user")))
                        // The property "credentials" and "mode" is needed for cookie related works. This not be need if the frontend and backend is the same domain/port. While the frontend is developed in different port, so this is needed
                        .credentials(RequestCredentials::Include)
                        .mode(RequestMode::Cors)
                        .send()
                        .await
                        .unwrap();

                    // Server will send the User if the user is logged in, otherwise the server will UNAUTHORIZED status
                    // Try to convert the json from response to User
                    let resp = resp.json::<User>().await;

                    // If it converts successfully then the User is logged in
                    if let Ok(user) = resp {
                        // User is logged in and server sent User
                        user_status.set(AuthStatus::LoggedIn(user));
                    } else {
                        // User is not logged in
                        user_status.set(AuthStatus::NotLoggedIn);
                    }
                });

                move || ()
            },
            (),
        );
    }

    // if user is not logged in then render the component ```Login```.
    {
        use_effect_with_deps(
            move |user_status| {
                if let AuthStatus::NotLoggedIn = &**user_status {
                    // Set its value to "true" so the ```Login``` component can be rendered
                    login_render.set(LoginRender(true));
                }
                || ()
            },
            user_status.clone(),
        )
    }

    html! {
        <>
        if let AuthStatus::LoggedIn(user) = (*user_status).clone() {
            <ContextProvider <User> context={user}>
                {props.children.clone()}
            </ContextProvider <User>>
        }
        else if let AuthStatus::RequestPending = (*user_status).clone() {
            // TODO: Show a Loading bar
            <h1>{"Wait for server response"}</h1>

        }


        </>
    }
}
