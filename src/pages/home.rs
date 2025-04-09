use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="home">
            <h1>{ "Welcome to the Home Page!" }</h1>
            <p>{ "This is the home page of our Yew application." }</p>
        </div>
    }
}