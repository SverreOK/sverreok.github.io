use yew::prelude::*;

use crate::pages::components::Footer;
use crate::pages::components::Navbar;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Navbar />
            <div class="splash-container">
            </div>
            <Footer />
        </>
    }
}

// do intro -> 
