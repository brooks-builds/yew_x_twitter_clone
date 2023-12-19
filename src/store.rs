use yew::AttrValue;
use yewdux::store::Store;

use crate::api::{self};

#[derive(Default, Debug, Clone, PartialEq, Eq, Store)]
pub struct AppState {
    pub posts: Vec<Post>,
}

impl AppState {
    pub fn get_post_by_id(&self, id: u32) -> Option<&Post> {
        self.posts.iter().find(|post| post.id == id)
    }

    pub fn add_post(&mut self, post: Post) {
        let post_id = post.id;
        if let Some(saved_post) = self
            .posts
            .iter_mut()
            .find(move |state_post| state_post.id == post_id)
        {
            *saved_post = post;
        } else {
            self.posts.push(post);
        }
    }

    pub fn add_reply(&mut self, reply: Post, parent_id: u32) {
        if let Some(parent) = self.posts.iter_mut().find(|post| post.id == parent_id) {
            parent.response_count += 1;
            parent.responses.push(reply);
        }
    }

    pub fn update_top_level_post(&mut self, new_text: AttrValue, id: u32) {
        let Some(post) = self.posts.iter_mut().find(|post| post.id == id) else {
            return;
        };

        post.text = new_text;
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Post {
    pub id: u32,
    pub text: AttrValue,
    pub likes: u32,
    pub response_count: u32,
    pub responses: Vec<Post>,
}

impl Post {
    pub fn new(id: u32, text: AttrValue) -> Self {
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

impl From<api::ApiOnePost> for Post {
    fn from(
        api::ApiOnePost {
            id,
            text,
            likes,
            replies,
        }: api::ApiOnePost,
    ) -> Self {
        Self {
            id,
            text: text.into(),
            likes,
            response_count: replies.len() as u32,
            responses: replies.into_iter().map(Into::into).collect(),
        }
    }
}
