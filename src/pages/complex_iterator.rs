use leptos::prelude::*;
use reactive_stores::Store;

/// option 1 basically just rewrites the whole thing each time
#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

/// by default, these values only change if the KEY is ALSO changed
/// Four ways to solve
/// 1. change the key so that it always updates when the data struct updates
/// 2. change the value so that it's reactive
/// 3. take a reactive slice of the data structure instead of using each row directly
/// 4. use a Store
#[component]
pub fn ComplexIterator() -> impl IntoView {
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);

    view! {
        // when we click we update each row, doubling its value
        <button style="position: sticky; top: 10px;" on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;
                }
            });
            // log new value of the signal
            leptos::logging::log!("{:?}", data.get());
        }>
            "update values"
        </button>

        // iterate over the rows and display each value.. but nothing changes?
        <For
            each=move || data.get()
            key=|state| state.key.clone()
            let(child)
        >
            <p>{child.value}</p>
        </For>

        /// this is option 3, memoized slices
        // Memo is a derived computation that only triggers a reactive update when its value has changed
        // you can make reactive values for subfields without wrapping them in signals
        // combine with <ForEnumerate/> to get an index

        // children prop allows running non-view code ???

        // define a value memo and use it in view. "value" doesn't actually use "child" being passed into each row
        // instead, it uses the index and reaches back into the original data to get the value

        // PROS of this: signal-wrapped benefits but without needing to wrap data in signals
        // CONS: it's complicated setting up memo-per-row inside <ForEnumerate/>
            // have to guard against panic if memo gets re-run after row removed
        <hr style="border: 1px solid orange;"></hr>
        <ForEnumerate
            each=move || data.get()
            key=|state| state.key.clone()
            children=move |index, _| {
                let value = Memo::new(move |_| {
                    data.with(|data| data.get(index.get()).map(|d| d.value).unwrap_or(0))
                });
                view! {
                    <p>{value}</p>
                }
            }
        />


        // change the key each time (easy but not efficient)
        <hr style="border: 1px solid orange;"></hr>
        <For
            each=move || data.get()
            key=|state| (state.key.clone(), state.value)
            let(child)
        >
            <p>{child.value}</p>
        </For>

        <hr style="border: 1px solid orange;"></hr>
        <NestedSignals />
        <hr style="border: 1px solid orange;"></hr>
        <UsingStores />


    }
}

/// option 2, change the value so that it's reactive by itself, key stays the same
#[derive(Debug, Clone)]
struct DatabaseEntry2 {
    key: String,
    value: RwSignal<i32>,
}

/// nested signals puts the value into a signal
/// "RwSignal<_>" is a read-write signal that combines getter and setter in 1 obj
/// easier than storing separate getters and setters in a struct
///
/// pros: most efficient and fits mental model of the framework:
/// !! values that change over time are wrapped in signals so the interface can respond to them !!
///
/// cons: nested reactivity can be cumbersome if you're receiving data,
///     from an API or other source you don't control, and you don't want to create,
///     a different struct wrapping each field in a signal (this should make more sense eventually)
#[component]
pub fn NestedSignals() -> impl IntoView {
    // start with a set of three rows
    let (data, _set_data) = signal(vec![
        DatabaseEntry2 {
            key: "foo".to_string(),
            value: RwSignal::new(10),
        },
        DatabaseEntry2 {
            key: "bar".to_string(),
            value: RwSignal::new(20),
        },
        DatabaseEntry2 {
            key: "baz".to_string(),
            value: RwSignal::new(15),
        },
    ]);
    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            for row in &*data.read() {
                row.value.update(|value| *value *= 2);
            }
            // log the new value of the signal
            leptos::logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        // iterate over the rows and display each value
        <For
            each=move || data.get()
            key=|state| state.key.clone()
            let(child)
        >
            <p>{child.value}</p>
        </For>
    }
}

/// option 4: Stores
/// Store-specific structs (separate from the plain structs used in options 1-3)
#[derive(Store, Debug, Clone)]
struct DatabaseEntry4 {
    key: String,
    value: i32,
}

#[derive(Store, Debug, Clone)]
pub struct Data4 {
    #[store(key: String = |row| row.key.clone())]
    rows: Vec<DatabaseEntry4>,
}

/// Stores provide fine-grained reactivity to struct fields automatically.
/// The Store derive macro creates reactive getters for each field.
/// Updating one field doesn't notify sibling fields.
///
/// PROS: fine grained reactivity, but avoids nesting signals or memoized slices
/// Can work with plain data (struct and Vec<_>, annotated with a Derive not special stuff)
///
/// CONS: newest API and likely will have some bugs
#[component]
pub fn UsingStores() -> impl IntoView {
    use reactive_stores::StoreFieldIterator;

    let data = Store::new(Data4 {
        rows: vec![
            DatabaseEntry4 {
                key: "foo".to_string(),
                value: 10,
            },
            DatabaseEntry4 {
                key: "bar".to_string(),
                value: 20,
            },
            DatabaseEntry4 {
                key: "baz".to_string(),
                value: 15,
            },
        ],
    });

    view! {
        <button on:click=move |_| {
            for row in data.rows().iter_unkeyed() {
                *row.value().write() *= 2;
            }
        }>
            "Update Values option 4"
        </button>


        <For
            each=move || data.rows()
            key=|row| row.read().key.clone()
            children=|child| {
                let value = child.value();
                view! { <p>{move || value.get()}</p> }
            }
        />
        <hr style="border: 1px solid orange;"></hr>
    }
}
