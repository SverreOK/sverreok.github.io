use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{
    Home,
    NotFound,
    Projects,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
        Route::Projects => html! { <Projects /> },
    }
}