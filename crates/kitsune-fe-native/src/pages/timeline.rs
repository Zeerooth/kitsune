use dioxus::prelude::{dioxus_elements, inline_props, render, Element, Props, Scope};

#[inline_props]
pub fn Timeline(cx: Scope, timeline: String) -> Element {
    render! {
        p { "Showing posts from {timeline}" }
    }
}
