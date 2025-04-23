use yew::prelude::*;
use yew_router::prelude::*;
mod routes;
mod pages;

use crate::routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new()
        .render();
}