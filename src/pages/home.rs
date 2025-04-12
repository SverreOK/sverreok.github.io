use yew::prelude::*;

use crate::pages::components::Footer;
use crate::pages::components::Navbar;
use crate::pages::components::Skills;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Navbar />
            <div class="content-wrapper" style="margin-top: 2em;"> // Add spacing from the navbar
                <div class="content">
                    <h1 class="content-head is-center">{"Sverre Kristensen"}</h1>
                    <div class="content-subhead">
                        <p>
                        {"23 year old university student from Norway, currently specializing in "}
                        <a href="https://en.wikipedia.org/wiki/Guidance,_navigation,_and_control" target="_blank" rel="noopener noreferrer">
                        {"GNC"}
                        </a>
                        {" and space systems."}
                        </p>
                        <p>{"This website is made as a hobby project in Rust and
                        as a place to collect a writeup of my projects."}</p>
                    </div>
                </div>
                <Skills />
            </div>
            <Footer />
        </>
    }
}