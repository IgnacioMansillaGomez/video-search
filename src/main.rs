use std::string;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{
    callback, function_component, html, use_state, Callback, Event, InputEvent, MouseEvent,
    Properties, UseStateHandle,
};
fn main() {
    yew::start_app::<App>();
}

#[derive(Clone)]
struct Video {
    id: String,
    name: String,
}

#[function_component(App)]
fn app() -> Html {
    let video: UseStateHandle<Option<Video>> = use_state(|| None);

    let on_search: Callback<String> = {
        let video: UseStateHandle<Option<Video>> = video.clone();
        Callback::from(move |text_to_seach: String| {
            let video_id: String = search_youtube(text_to_seach);
            video.set(Some(Video {
                id: video_id,
                name: "name".to_string(),
            }))
        })
    };

    let video_section = match (*video).clone() {
        Some(video) => html! {
            <VideoSection name={video.name} id={video.id}/>
        },
        None => html! {},
    };

    html! {
        <main>
        <VideoControl on_search={on_search}/>
        {video_section}
        </main>
    }
}

#[derive(Properties, PartialEq)]
struct VideoControlProps {
    on_search: Callback<String>,
}

fn search_youtube(text_to_search: String) -> String {
    web_sys::console::log_1(&text_to_search.into());

    String::from("wv_dJvjuC04")
}

#[function_component(VideoControl)]
fn video_control(props: &VideoControlProps) -> Html {
    let text_to_search: UseStateHandle<String> = use_state(|| String::new());

    let handle_input: Callback<yew::InputEvent> = {
        let text_to_search = text_to_search.clone();

        Callback::from(move |input_event: InputEvent| {
            //get text from input event
            let text: String = get_value_from_input_event(input_event);
            text_to_search.set(text)
        })
    };

    let handle_search: Callback<MouseEvent> = {
        let on_search = props.on_search.clone();
        Callback::from(move |_| on_search.emit(text_to_search.to_string()))
    };

    html! {
        <main>
            <div>
                {"Search..."}
            </div>
            <div>
                <input type="text" oninput={handle_input} />
            </div>
            <div>
                <button onclick={handle_search}>{"Search"}</button>
            </div>
        </main>
    }
}

#[derive(Properties, PartialEq)]
struct VideoSectionProps {
    id: String,
    name: String,
}

#[function_component(VideoSection)]
fn video_sectionl(props: &VideoSectionProps) -> Html {
    let yt_url: String = format!("https://www.youtube.com/embed/{}", props.id);
    html! {
        <main>
        <div>
            <iframe width="560" height="315"  src={yt_url}/>
        </div>
        </main>
    }
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}
