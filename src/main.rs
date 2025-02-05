use yew::{function_component, html, Callback};
fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    let handle_input = Callback::from(|_| {});

    html! {
        <main>
            <div>
            {"Search..."}
            </div>
            <div>
            <input type="text" oninput={handle_input} />
            </div>
            <div>
            <button>{"Search"}</button>
            </div>
            <div>
            <iframe width="560" height="315" src="https://www.youtube.com/embed/tmYhb0efRIw"></iframe>
            </div>
        </main>
    }
}
