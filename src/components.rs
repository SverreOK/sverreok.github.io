use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="">
                <p>{ "© 2023 Your Name" }</p>
                <p>{ "All rights reserved." }</p>
            </div>
        </footer>
    }
}

#[function_component(Navbar)]
pub fn navbar() -> Html {

}