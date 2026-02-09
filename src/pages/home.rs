use crate::components::counter_btn::Button;
use crate::components::example::Example;
use crate::components::slider::Slider;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <input type="text"
                // adding :target gives us typed access to the element
                // that is the target of the event that fires
                on:input:target=move |ev| {
                    // .value() returns the current value of an HTML input element
                    set_name.set(ev.target().value());
                }

                // the `prop:` syntax lets you update a DOM property,
                // rather than an attribute.
                prop:value=name
            />
            <p>"Name is: " {name}</p>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>
                    <a href="/secret">"Welcome to Leptos"</a>
                    <a href="/iterating">"\ndemo"</a>
                </h1>
                    <p>
                        <a href="/forms">"\n\nforms+inputs"</a>
                    </p>
                <Button increment=5 />
                <Button increment=10 />
                <div class="buttons">
                    <Example />
                </div>
                <div>
                    <Slider />
                </div>

            </div>
        </ErrorBoundary>
    }
}
