use crate::components::dynamiclist::DynamicList;
use leptos::prelude::*;

#[component]
pub fn IteratingPage() -> impl IntoView {
    view! {
        <DynamicList initial_length=5 />
    }
}
