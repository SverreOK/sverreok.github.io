use yew::prelude::*;

use crate::pages::components::Footer;
use crate::pages::components::Navbar;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
            <Navbar />
            <div class="splash-container">
                <div class="error-container">
                    <h1 class="error-code">{"404"}</h1>
                    <p class="error-message">{"Oops! The page you're looking for doesn't exist."}</p>
                    <a href="/" class="back-home">{"Go Back Home"}</a>
                </div>
            </div>
            <Footer />
        </>
    }
}