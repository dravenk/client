use yew::prelude::*;
use yew_router::prelude::*;

use super::chat::Chat;
use super::conversation::Conversation;
use super::settings::Settings;
use super::status::Status;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum MainRoute {
    #[at("/")]
    Main,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/status")]
    Status,
    #[at("/settings")]
    Settings,
    #[at("/conversation")]
    Conversation,
    #[at("/chat")]
    Chat,
}

pub fn switch(route: MainRoute) -> Html {
    match route {
        MainRoute::Main => html! { <h1>{ "Main" }</h1> },
        MainRoute::NotFound => html! { <h1>{ "404" }</h1> },
        MainRoute::Status => {
            html! {
                <Status />
            }
        }
        MainRoute::Settings => {
            html! {
                <Settings />
            }
        }
        MainRoute::Conversation => html! {
            <Conversation />
        },
        MainRoute::Chat => html! {
            <Chat />
        },
    }
}
