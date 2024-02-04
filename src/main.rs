use leptos::{html::Input, *};
use web_sys::SubmitEvent;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <h1>Leptos Hello world</h1>

        <h2>Basics</h2>

        <h3>Basic signals, derived signals, components</h3>

        <button
            on:click=move |_| { set_count.update(|n| *n += 1) }

            class:red=move || count() % 2 == 1
            class:blue=move || count() % 2 == 0
        >
            "Click me: "
            {count}
        </button>

        <ProgressBar progress=count/>

        <ProgressBar progress=Signal::derive(double_count)/>

        <hr/>

        <h3>Static views with a Vec</h3>

        <StaticViewsWithVec/>

        <hr/>

        <h3>"Dynamic Rendering with the <For/> Component"</h3>

        <DynamicList initial_length=3/>

        <hr/>

        <h3>"Iterating over More Complex Data with <For/>"</h3>

        <IteratingOverMoreComplexDataWithFor/>

        <hr/>

        <h2>Forms and inputs</h2>

        <h3>Controlled input</h3>

        <ControlledInput/>

        <hr/>

        <h3>Uncontrolled input & form</h3>

        <UncontrolledInput/>

        <hr/>

        <h2>Control flow</h2>

        <h3>Control flow with if/else</h3>

        <ControlFlowIfElse/>

        <hr/>

        <h3>Control flow with option</h3>

        <ControlFlowWithOption/>

        <hr/>

        <h3>Control flow with option concise</h3>

        <ControlFlowWithOptionConcise/>

        <hr/>

        <h3>Control flow with match</h3>

        <ControlFlowWithMatch/>

        <hr/>

        <h3>Control flow with show component</h3>

        <ControlFlowWithShowComponent/>

        <hr/>

        <h3>"Control flow with type conversion"</h3>

        <ControlFlowWithTypeConversions/>

        <hr/>

        <h2>Error handling</h2>

        <h3>Error handling number input</h3>

        <ErrorHandlingNumberInput/>

        <h2>Context</h2>

        <h3>Context provider</h3>

        <ContextProvider/>

        <hr/>

        <h2>Children</h2>

        <h3>Render prop and children</h3>

        <TakesChildren render_prop=|| view! { <p>Hello there</p> }>"General Kenobi"</TakesChildren>
    }
}

#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

#[component]
fn StaticViewsWithVec() -> impl IntoView {
    let length = 5;
    let counters = (1..=length).map(create_signal);

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
                </li>
            }
        })
        .collect_view();

    view! { <ul>{counter_buttons}</ul> }
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=counters
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button on:click=move |_| {
                                    set_count.update(|n| *n += 1)
                                }>{count}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                }>

                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />

            </ul>
        </div>
    }
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn IteratingOverMoreComplexDataWithFor() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = create_signal(vec![
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
        <button on:click=move |_| {
            set_data
                .update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
            logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        // iterate over the rows and display each value
        <For
            each=move || data().into_iter().enumerate()
            key=|(_, state)| state.key.clone()
            children=move |(index, _)| {
                let value = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                });
                view! { <p>{value}</p> }
            }
        />
    }
}

#[component]
pub fn ControlledInput() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <label for="name">Name:</label>
        <input
            prop:value=name
            id="name"
            type="text"
            on:input=move |e| { set_name(event_target_value(&e)) }
        />

        <p>Name is {name}</p>
    }
}

#[component]
pub fn UncontrolledInput() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());

    let input_element: NodeRef<Input> = create_node_ref();

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();

        let value = input_element().expect("<input> to exist").value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <label for="name">Name:</label>
            <input type="text" value=name node_ref=input_element id="name"/>
            <input type="submit" value="Submit"/>
        </form>

        <p>"Name is: " {name}</p>
    }
}

#[component]
pub fn ControlFlowIfElse() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;

    view! {
        <button on:click=move |_| set_value.update(|v| *v += 1)>increment</button>
        <p>{value} " " {move || if is_odd() { "is odd" } else { "is even" }}</p>
    }
}

#[component]
pub fn ControlFlowWithOption() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;

    let message = move || if is_odd() { Some("is odd") } else { None };

    view! {
        <button on:click=move |_| set_value.update(|v| *v += 1)>increment</button>
        <p>{value} " " {message}</p>
    }
}

#[component]
pub fn ControlFlowWithOptionConcise() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;

    let message = move || is_odd().then_some("Is odd");

    view! {
        <button on:click=move |_| set_value.update(|v| *v += 1)>increment</button>
        <p>{value} " " {message}</p>
    }
}

pub fn is_n_odd(value: i32) -> bool {
    value & 1 == 1
}

#[component]
pub fn ControlFlowWithMatch() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    let message = move || match value() {
        0 => "Zero",
        1 => "One",
        n if is_n_odd(n) => "Odd",
        _ => "Even",
    };

    view! {
        <button on:click=move |_| set_value.update(|v| *v += 1)>increment</button>
        <p>{value} " " {message}</p>
    }
}

#[component]
pub fn ControlFlowWithShowComponent() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    view! {
        <Show when=move || { value() > 5 } fallback=move || view! { <p>{value} is less than 5</p> }>
            <p>{value} is greater than 5</p>
        </Show>
        <button on:click=move |_| set_value.update(|v| *v += 1)>increment</button>
    }
}

#[component]
pub fn ControlFlowWithTypeConversions() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;

    view! {
        <main>
            {move || match is_odd() {
                true if value() == 1 => view! { <pre>"One"</pre> }.into_any(),
                false if value() == 2 => view! { <p>"Two"</p> }.into_any(),
                _ => view! { <textarea>{value()}</textarea> }.into_any(),
            }}
            <button on:click=move |_| set_value.update(|v| *v += 1)>increment</button>
        </main>
    }
}

#[component]
pub fn ErrorHandlingNumberInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |e| set_value(event_target_value(&e).parse::<i32>());

    view! {
        <label>
            "Type a number (or not):" <input on:input=on_input/>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}

                        </ul>
                    </div>
                }
            }>
                <p>"You entered: " <strong>{value}</strong></p>
            </ErrorBoundary>

        </label>
    }
}

#[component]
pub fn ContextProvider() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    provide_context(set_value);

    view! {
        <p>"Value: " {value}</p>
        <ContextConsumer/>
    }
}

#[component]
pub fn ContextConsumer() -> impl IntoView {
    let set_value = use_context::<WriteSignal<i32>>().expect("Expected setter for i32 value");

    view! { <button on:click=move |_| { set_value.update(|n| *n += 1) }>"Increment"</button> }
}

#[component]
pub fn TakesChildren<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h4>"Render prop: "</h4>
        {render_prop()}

        <h4>"Children: "</h4>
        {children()}
    }
}
