use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let is_dark_mode = use_state(|| false);

    let toggle_theme = {
        let is_dark_mode = is_dark_mode.clone();
        Callback::from(move |_| {
            let new_mode = !*is_dark_mode;
            is_dark_mode.set(new_mode);

            let document_element = gloo::utils::document().document_element().unwrap();
            document_element.set_attribute("data-theme", if new_mode { "dark" } else { "light" }).unwrap();
        })
    };

    html! {
        <div class="header">
            <div class="home-menu pure-menu pure-menu-horizontal pure-menu-fixed">
                <a class="pure-menu-heading" href="/">
                    <i class="fas fa-home"></i>
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

#[derive(Properties, PartialEq)]
pub struct Skill {
    pub name: &'static str,
    pub icon: &'static str,
}

#[function_component(Skills)]
pub fn skills() -> Html {
    let skills = vec![
        Skill { name: "C/C++", icon: "static/icons/cpp_logo.svg" },
        Skill { name: "CMake", icon: "static/icons/Cmake.svg" },
        Skill { name: "Docker", icon: "static/icons/docker.svg" },
        Skill { name: "ARM Assembly", icon: "static/icons/asm.svg" },
        Skill { name: "Python", icon: "static/icons/python.svg" },
        Skill { name: "Matlab", icon: "static/icons/matlab.svg" },
        Skill { name: "Simulink", icon: "static/icons/simulink.png" },
        Skill { name: "Altium", icon: "static/icons/altium.png" },
        Skill { name: "LTSpice", icon: "static/icons/ltspice.png" },
        Skill { name: "STM32", icon: "static/icons/stm32.png" },
    ];

    let curr_learning = vec![
        Skill { name: "Rust", icon: "static/icons/rust.svg" },
        Skill { name: "Google Test", icon: "static/icons/googletest.png" },
        Skill { name: "OpenFOAM", icon: "static/icons/openfoam.png" },
    ];

    html! {
        <>
        <div class="skills-container">
            <h2 class="skills-title">{"Experienced with"}</h2>
            <div class="skills-grid">
                { for skills.iter().map(|skill| html! {
                    <div class="skill-item">
                        <img src={skill.icon} alt={format!("{} icon", skill.name)} class="skill-icon" />
                        <span class="skill-name">{ skill.name }</span>
                    </div>
                })}
            </div>
        </div>
        <div class="skills-container">
            <h2 class="skills-title">{"Currently learning"}</h2>
            <div class="skills-grid">
                { for curr_learning.iter().map(|skill| html! {
                    <div class="skill-item">
                        <img src={skill.icon} alt={format!("{} icon", skill.name)} class="skill-icon" />
                        <span class="skill-name">{ skill.name }</span>
                    </div>
                })}
            </div>
        </div>
        </>
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