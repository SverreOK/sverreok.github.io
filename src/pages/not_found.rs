use yew::prelude::*;

use crate::pages::components::Footer;
use crate::pages::components::Navbar;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
            <Navbar />
            <div class="splash-container">
            <h1>{"404"}</h1>
            </div>
            <Footer />
        </>
    }
}