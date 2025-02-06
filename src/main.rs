use yew::{callback, function_component, html, use_state, Callback, Properties, UseStateHandle};
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
    let on_search = Callback::from(|_| {});
    let video: UseStateHandle<Option<Video>> = use_state(|| None);

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

#[function_component(VideoControl)]
fn video_control(props: &VideoControlProps) -> Html {
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
    let yt_url = format!("https://www.youtube.com/embed/{}", props.id);
    html! {
        <main>
        <div>
            <iframe width="560" height="315"  src={yt_url}/>
        </div>
        </main>
    }
}
