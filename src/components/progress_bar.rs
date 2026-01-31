use leptos::prelude::*;

/// shows progress toward a goal
#[component]
pub fn ProgressBar(
    /// maximum value of progress bar
    #[prop(default = 100)]
    max: u16,
    /// how much progress to display
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            // now this works
            value=progress
        />
        <br />
    }
}
