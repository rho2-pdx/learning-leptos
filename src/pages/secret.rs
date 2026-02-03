use leptos::prelude::*;

/// Hidden test secret page
#[component]
pub fn Secret() -> impl IntoView {
    let values = vec![0, 1, 2];
    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();
    view! {
        <h1>"you found the secret page!"</h1>
        <p>{values.clone()}</p>
        /*
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect::<Vec<_>>()}
                // this is unkeyed
        </ul>
        */
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect_view()}
        </ul>
        <ul><h3>"this is the counter button part"
        </h3>{counter_buttons}</ul>

    }
}
