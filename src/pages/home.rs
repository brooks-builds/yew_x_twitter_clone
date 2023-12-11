use stylist::{style, yew::styled_component};
use yew::{function_component, html, Html};

#[styled_component]
pub fn Home() -> Html {
    let center = style! {
        text-align: center;
    }
    .expect("style failed to create");

    let form = style! {
        display: flex;
        flex-direction: column;

        button {
            margin-top: 1rem;
            width: 6rem;
        }
    }
    .unwrap();

    html! {
        <>
            <h1 class={center}>{"X/Twitter Clone"}</h1>
            <form class={form}>
                <h2>{"Create new post"}</h2>
                <textarea rows="5" cols="55">
                </textarea>
                <button type={"submit"}>{"Submit post"}</button>
            </form>
        </>
    }
}
