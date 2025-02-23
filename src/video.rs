use gloo_net::http::Request;
use serde::Deserialize;

pub async fn search_youtube(text_to_search: String) -> Result<VideoItem, gloo_net::Error> {
    let query_url: String = format!(
        "https://www.googleapis.com/youtube/v3/search?part=id%2Csnippet&q={text_to_search}"
    );

    let response = Request::get(&query_url).send().await?;

    let search_result = response.json::<SearchResult>().await?;
    let empty_video = build_empty_videos();
    let video = match search_result.items.first() {
        Some(video) => video,
        None => &empty_video,
    };

    Ok(video.clone())
}

fn build_empty_videos() -> VideoItem {
    VideoItem {
        id: VideoItemId {
            kind: "".to_string(),
            video_id: "".to_string(),
        },
        snippet: VideoSnippet {
            title: "".to_string(),
            description: "".to_string(),
        },
    }
}

#[derive(Deserialize)]
struct SearchResult {
    region_code: String,
    items: Vec<VideoItem>,
}

#[derive(Clone, Deserialize)]
pub struct VideoItem {
    pub id: VideoItemId,
    pub snippet: VideoSnippet,
}

#[derive(Clone, Deserialize)]
pub struct VideoItemId {
    pub kind: String,
    pub video_id: String,
}

#[derive(Clone, Deserialize)]
pub struct VideoSnippet {
    pub title: String,
    pub description: String,
}
