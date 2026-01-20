use crate::backend::*;
use bevy::ecs::entity::Entity;
use bevy_dioxus_hooks::query::use_bevy_query;
use bevy_dioxus_hooks::resource::hook::use_bevy_resource;
use bevy_dioxus_interop::signals::CrossDomSignal;
use bevy_dioxus_sync::panels::DioxusElementMarker;
use dioxus::prelude::*;

#[derive(Debug)]
pub struct AppUi;

impl DioxusElementMarker for AppUi {
    fn element(&self) -> Element {
        // app_ui()
        game_ui()
    }
}

#[component]
pub fn game_ui() -> Element {
    let fps = use_bevy_resource::<FPS>();
    let idle_time_res = use_bevy_resource::<CurrentIdleTimeSeconds>();
    let best_idle_time_res = use_bevy_resource::<LongestIdleTimeSeconds>();
    let test_component = use_bevy_query::<(Entity, &TestComponent), ()>();

    rsx! {
        document::Stylesheet { href: asset!("src/frontend/ui.css") }

        main {
            progress_bar_comp { curent_time: idle_time_res.get().map(|time| time.0).unwrap_or(0.0), longest_time: best_idle_time_res.get().map(|time| time.0).unwrap_or(0.0) }
            // { progress_bar(idle_time_res, best_idle_time_res) }

            div {
                id: "panel",
                class: "catch-events",

                if let Ok(test_comps) = test_component.get() {
                    for (_, comp) in test_comps.iter() {

                        div {
                            "{comp.1.0.get():?}"
                        }
                    }
                }

                div {
                    id: "footer",
                    p { "Bevy framerate: {fps}" }
                }
            }
        }
    }
}

#[component]
fn progress_bar_comp(curent_time: f64, longest_time: f64) -> Element {
    rsx! {
        div {
            style: "
                width: 100%;
                height: 1.5rem;
                background-color: #585b70ff;
                border-radius: calc(1.5rem / 2);
                overflow: hidden;
                color: #11111bff;
                text-justify: center;
                align-content: center;
                align-items: center;
                display: flex;
                flex-direction: row;
            ",

            div {
                style: format!("
                    height: 100%;
                    background-color: #a6e3a1ff;
                    text-align: left;
                    color: #11111bff;
                    width: {}%;
                ",
                    if curent_time + longest_time > 0.0 {
                        curent_time / longest_time * 100.
                    } else {
                        100.0
                    }

                ),

                "{curent_time:.2} sec / {longest_time:.1} sec"
            }
        }
    }
}
