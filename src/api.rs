use dotenvy_macro::dotenv;
use eyre::Result;
use serde::{Deserialize, Serialize};
use yew::AttrValue;

const API_URL: &str = dotenv!("API_URL");

pub async fn create_post(post_text: AttrValue, parent_id: Option<u32>) -> Result<Post> {
    let data = CreatePost {
        text: post_text.to_string(),
        parent_id,
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
        text: post_text.to_string(),
        likes: 0,
        replies: 0,
    };

    Ok(post)
}

#[derive(Serialize)]
pub struct CreatePost {
    text: String,
    parent_id: Option<u32>,
}

#[derive(Deserialize)]
pub struct CreatePostResponse {
    id: u32,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub struct Post {
    pub id: u32,
    pub text: String,
    pub likes: u32,
    pub replies: u32,
}

pub async fn get_all_posts() -> Result<Vec<Post>> {
    let url = format!("{API_URL}/api/v1/posts");
    let result = gloo::net::http::Request::get(&url)
        .send()
        .await?
        .json::<Vec<Post>>()
        .await?;
    Ok(result)
}

pub async fn get_one_post(id: u32) -> Result<Option<ApiOnePost>> {
    let url = format!("{API_URL}/api/v1/posts/{id}");
    let result = gloo::net::http::Request::get(&url)
        .send()
        .await?
        .json::<ApiOnePost>()
        .await?;

    Ok(Some(result))
}

#[derive(Debug, Deserialize)]
pub struct ApiOnePost {
    pub id: u32,
    pub text: String,
    pub likes: u32,
    pub replies: Vec<Post>,
}
