use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let is_dark_mode = use_state(|| false);

    let toggle_theme = {
        let is_dark_mode = is_dark_mode.clone();
        Callback::from(move |_| {
            let new_mode = !*is_dark_mode;
            is_dark_mode.set(new_mode);

            if new_mode {
                gloo::utils::document().body().unwrap().set_class_name("dark-mode");
            } else {
                gloo::utils::document().body().unwrap().set_class_name("light-mode");
            }
        })
    };

    html! {
        <div class="header">
            <div class="home-menu pure-menu pure-menu-horizontal pure-menu-fixed">
                <a class="pure-menu-heading" href="">
                    <i class="fas fa-home"></i> // Font Awesome home icon
                </a>

                <ul class="pure-menu-list">
                    <li class="pure-menu-item">
                        <button class="theme-toggle" onclick={toggle_theme}>
                            <i class={ if *is_dark_mode { "fas fa-sun" } else { "fas fa-moon" } }></i>
                        </button>
                    </li>
                    <li class="pure-menu-item"><a href="#" class="pure-menu-link">{"About"}</a></li>
                    <li class="pure-menu-item"><a href="#" class="pure-menu-link">{"Projects"}</a></li>
                </ul>
            </div>
        </div>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer l-box is-center">
            <p>
                <a href="https://www.linkedin.com/in/9263391b4/" target="_blank">
                    <i class="fab fa-linkedin"></i>
                </a>
                <a href="https://github.com/SverreOK" target="_blank">
                    <i class="fab fa-github"></i>
                </a>
                <a href="/404">
                    <i class="fas fa-envelope"></i>
                </a>
            </p>
        </footer>
    }
}