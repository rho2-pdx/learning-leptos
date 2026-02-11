use leptos::prelude::*;

/// this is for practicing the control flow section, like with impl and reactive T funcs
#[component]
pub fn ControlFlow() -> impl IntoView {
    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 != 0;

    view! {
        <p>
            {move || if is_odd() {
                "Odd'"
            } else {
                "Even"
            }}
        </p>
    }
}

// Rust is expression oriented
// things like if x() {y} return their values, useful for declarative interfaces
//
// for a T that implements IntoView, Option<T> and Result<T, impl Error> also implement IntoView
// F(n) -> T renders a reactive T, Fn() -> Option<T> and Result<xx> also are reactive
// Rust has helpers like Option::map, Option::ok_or, etc.
// these convert between standard types, that can be rendered, in a declarative way
//
// TO BE REACTIVE values must be functions. Hence wrapped things in move || closures
//
