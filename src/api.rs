use dotenvy_macro::dotenv;
use serde::Serialize;
use yew::AttrValue;

pub async fn create_post(post_text: AttrValue) {
    let data = CreatePost {
        text: post_text.to_string(),
    };
    let url = dotenv!("API_URL");

    let result = gloo::net::http::Request::post(url).json(&data).unwrap();
}

#[derive(Serialize)]
pub struct CreatePost {
    text: String,
}
