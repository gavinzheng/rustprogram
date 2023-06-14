/// Import all goodies from Yew.
use yew::prelude::*;

/// Our app's state. It is now just a unit struct with no property
/// because our state will be encapsulated within the Counter component.
struct App;

/// A Counter functional component ala React.
#[function_component(Counter)]
fn counter() -> Html {

    // Initializing the component's state.
    let counter = use_state(|| 0);

    // The onclick callback function.
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    // The component written in plain-old HTML, thanks to html! macro.
    html! {
        <div class="uk-position-center uk-text-center">
            <button 
                {onclick}
                class="uk-button uk-button-primary uk-button-large"
            >
                { "+1" }
            </button>
            <p>{ *counter }</p>
        </div>
    }
}

/// The main App container.
impl Component for App {

    // The internal types that are required to be implemented to comply Component trait.
    type Message = ();
    type Properties = ();

    // The constructor function, which instantiates the App.
    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    // The view lifecycle function, which renders the App.
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Counter />
        }
    }
}

/// Start the Yew app
fn main() {
    yew::start_app::<App>();
}