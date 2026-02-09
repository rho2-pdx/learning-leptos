use leptos::ev::*;
use leptos::html::*;
use leptos::prelude::*;

// two basic patterns of interacting with inputs in Leptos, similar to React/SolidJS
//
// Controlled and Uncontrolled

/// this page is going to feature forms and inputs work
#[component]
pub fn FormPage() -> impl IntoView {
    // -- Controlled inputs --
    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let favorite_color = RwSignal::new("red".to_string());
    let spam_me = RwSignal::new(true);

    // -- Uncontrolled input --
    let (uncontrolled_name, set_uncontrolled_name) = signal("Uncontrolled".to_string());

    let (value, set_value) = signal(0i32); // special case
    let some_value = RwSignal::new("special case".to_string());
    let input_element: NodeRef<Input> = NodeRef::new();
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_uncontrolled_name.set(value);
    };
    //
    // Controlled:
    //  - framework controls the state of the input element
    //  - on every input event, local signal with current state updates value prop of input
    // 1. input event fires on (almost) every change to the element
    // 2. change event fires (more or less) when you unfocus the input (usually on:input)
    // 3. value attribute only sets initial value of th einput
    // 4. value property continues updating afterwards (usually prop:value)

    // prop:value is due to a distinction between HTML attributes and DOM element properties
    // ".setAttribute()" vs "property" field in JS class representation
    //
    // if you edit the contents of this input field manually, then try:
    // 1. ".setAttribute()" will no longer do anything
    // 2. ".value" will be a modifiable variable

    // "bind" lets you do the same thing as a "controlled input" by auto-binding signals to inputs
    // bind:value = text
    // bind:checked = checkboxes
    // bind:group = radio button groups

    view! {
        <input type="text"
            on:input:target=move |ev| {
                set_name.set(ev.target().value());
            }
            // the 'prop:' syntax lets you update a DOM property,
            // rather than an attribute
            prop:value=name
        />
        <div>
            <input type="email"
                bind:value=email
            />
            <label>
                "Please send me lots of spam email."
                <input type="checkbox"
                    bind:checked=spam_me
                />
            </label>

            <fieldset>
                <legend>"Favorite color"</legend>
                <label>
                    "Red"
                    <input
                        type="radio"
                        name="color"
                        value="red"
                        bind:group=favorite_color
                    />
                </label>
                <label>
                    "Green"
                    <input
                        type="radio"
                        name="color"
                        value="green"
                        bind:group=favorite_color
                    />
                </label>
                <label>
                    "Blue"
                    <input
                        type="radio"
                        name="color"
                        value="blue"
                        bind:group=favorite_color
                    />
                </label>
            </fieldset>
        </div>
        <p>"your favorite color is "{favorite_color} "."</p>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You'll receive great spam"</p>
        </Show>
    <hr style="border: 1px solid orange;"></hr>

    // Uncontrolled:
    //
    // browser controls the state of the input element (rather than the framework)
    // Instead of using a signal to continuously update, it's held in a NodeRef
    // Typically you use this when you want a "submit" button before anything is saved
    //
    // leptos::html module provides lots of types for every HTML element
    // leptos::ev module is for the submit functionality
    //
    // 1. use value (not prop:value) since browser controls state
    // 2. use node_ref=... to fill NodeRef, a kind of reactive smart pointer
    //  - NodeRef is used to access the underlying DOM node, value set when element renders
    //
    // on_submit accesses input's value to call set_name.set()
    // to access DOM node in NodeRef, we can call as function or use .get()
    // returns "Option<leptos::HtmlElement<html::Input>>" which is not always safe to unwrap!
    //      - however in this case, we know it's already been mounted due to firing the event so it's safe
    // now that it's unwrapped, we can call .value() since it's correctly-typed HTML
    // uncontrolled:
        <div>
            <form on:submit=on_submit>
                <input type="text"
                    value=uncontrolled_name
                    node_ref=input_element
                />
                <input type="submit" value="Submit" />
            </form>
            <p>"Uncontrolled Name is: "{uncontrolled_name}</p>
        </div>

        // Special cases!

        // text area does not support HTML "value", gets that as plain text child node
        <div>
            <textarea
                prop:value=move || some_value.get()
                on:input:target=move |ev| some_value.set(ev.target().value())
            >
                {some_value}
            </textarea>

            // the select element can be controlled via value property on the select itself
            <select
                on:change:target=move |ev| {
                    set_value.set(ev.target().value().parse().unwrap());
                }
                prop:value=move || value.get().to_string()
            >
                <option value="0">"0"</option>
                <option value="1">"1"</option>
                <option value="2">"2"</option>
            </select>
            <button on:click=move |_| set_value.update(|n| {
                if *n == 2 {
                    *n = 0;
                } else {
                    *n += 1;
                }

            })>
                "Next Option"
            </button>
        </div>

    }
}
