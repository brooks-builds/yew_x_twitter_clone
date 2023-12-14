use yew::AttrValue;
use yewdux::store::Store;

use crate::api;

#[derive(Default, Debug, Clone, PartialEq, Eq, Store)]
pub struct AppState {
    pub posts: Vec<Post>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Post {
    pub id: i32,
    pub text: AttrValue,
    pub likes: i32,
    pub response_count: i32,
    pub responses: Vec<Post>,
}

impl Post {
    pub fn new(id: i32, text: AttrValue) -> Self {
        Self {
            id,
            text,
            likes: 0,
            response_count: 0,
            responses: vec![],
        }
    }
}

impl From<api::Post> for Post {
    fn from(value: api::Post) -> Self {
        Self {
            id: value.id,
            text: value.text.into(),
            likes: value.likes,
            response_count: value.replies,
            responses: vec![],
        }
    }
}
