use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{
    Home,
    NotFound,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
    }
}