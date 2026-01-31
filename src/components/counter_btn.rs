use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    view! {
        <button
            on:click=move |_| {
                set_count.set(count.get() + increment)
            }
            style="position: relative; display: block; margin-left: 0; margin-right: auto;"
            style:left=move || format!("{}px", count.get())
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            style=("--columns", move || count.get().to_string())
        >

            "Click me: " {count}
        </button>
        <progress
            max="50" // signals are functions, so 'value=count', 'value=move || count.get()' are the same

            //value=count
            //value=move || count.get() * 2
            value = double_count
        />
        <p>
            "Double Count: "
            {double_count}
        </p>
    }
}
