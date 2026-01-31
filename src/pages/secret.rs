use leptos::prelude::*;

/// Hidden test secret page
#[component]
pub fn Secret() -> impl IntoView {
    let values = vec![0, 1, 2];
    view! {
        <p>{values.clone()}</p>
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect::<Vec<_>>()}
        </ul>
        <h1>"you found the secret page!"</h1>
    }
}
