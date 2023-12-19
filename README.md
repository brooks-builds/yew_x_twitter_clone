# Yew X/Twitter Clone

This is a learning project, we are taking the very basics of x/twitter and implementing them to practice our frontend skills using [Yew](https://yew.rs/).

We are using the [axum x/twitter clone](https://github.com/brooks-builds/axum_x_twitter_clone) project for the backend.

## Features

- [x] As a user, I want to create a post
- [x] As a user, I want to see all posts
- [x] As a user, I want to see replies to a single post
- [x] As a user, I want to reply to a post
- [x] As a user, I want to update the text of a post
- [ ] As a user, I want to delete a post

## Tech used

- Rust 1.74.1
  - With toolchains
    - wasm32-unknown-unknown
- yew v0.21.0
  - with features
    - csr
- yew-router v0.18.0
- stylist v0.13.0
  - with features
    - yew
- gloo v0.11.0
- wasm-bindgen v0.2.89
- web-sys v0.3.66
  - with features
    - HtmlTextAreaElement
- serde v1.0.193
  - with features
    - derive
- eyre v0.6.9
- wasm-bindgen-futures v0.4.39
- yewdux v0.9.4

- cli
  - trunk v0.17.5
