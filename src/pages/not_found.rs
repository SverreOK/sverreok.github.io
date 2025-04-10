use yew::prelude::*;

use crate::pages::components::Footer;
use crate::pages::components::Navbar;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
        <Navbar />
        <div class="not-found">
            <h1>{ "404 - Page Not Found" }</h1>
            <p>{ "Sorry, the page you are looking for does not exist." }</p>
            <a href="/">{"Go back to Home"}</a>
        </div>
        <Footer />
        </>
    }
}