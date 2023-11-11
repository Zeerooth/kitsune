use dioxus::prelude::{dioxus_elements, render, Element, Scope};

pub fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Welcome to Kitsune!" }
    }
}
