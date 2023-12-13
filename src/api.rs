use dotenvy_macro::dotenv;
use eyre::Result;
use serde::{Deserialize, Serialize};
use yew::AttrValue;

const API_URL: &str = dotenv!("API_URL");

pub async fn create_post(post_text: AttrValue) -> Result<Post> {
    let data = CreatePost {
        text: post_text.to_string(),
    };
    let url = format!("{API_URL}/api/v1/posts");

    let result = gloo::net::http::Request::post(&url)
        .json(&data)?
        .send()
        .await?
        .json::<CreatePostResponse>()
        .await?;

    let post = Post {
        id: result.id,
        text: post_text,
    };

    Ok(post)
}

#[derive(Serialize)]
pub struct CreatePost {
    text: String,
}

#[derive(Deserialize)]
pub struct CreatePostResponse {
    id: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Post {
    pub id: i32,
    pub text: AttrValue,
}
