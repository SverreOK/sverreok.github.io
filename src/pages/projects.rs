use yew::prelude::*;

use crate::pages::components::Footer;
use crate::pages::components::Navbar;

pub struct Project {
    pub name: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
}

#[function_component(Projects)]
pub fn projects() -> Html {

    let projects = vec![
        Project {   name: "Flight Computer", 
                    icon: "static/icons/flight_computer.png",
                    description: "Development of a rocketry flight computer capable of controlling multiple autonomous vehicles" },
        Project {   name: "6DOF Flight Simulator", 
                    icon: "static/icons/6dof.png",
                    description: "A 6DOF flight simulator for testing and development purposes." },
        Project {   name: "Bifrost", 
                    icon: "static/icons/bifrost.png",
                    description: "Propulse's first biliquid rocket. Created in 2023 and was the biggest biliquid at its time." },
    ];

    html! {
        <>
            <Navbar />
            <div class="box-container">
            <h2 class="box-title">{"Projects"}</h2>
            <div class="box-grid project-grid">
                { for projects.iter().map(|skill| html! {
                    <div class="box-item project-item">
                        <img src={skill.icon} alt={format!("{} icon", skill.name)} class="box-icon" />
                        <span class="box-name">{ skill.name }</span>
                        <p class="box-description">{ skill.description }</p>
                    </div>
                })}
            </div>
            </div>
            <Footer />
        </>
    }
}