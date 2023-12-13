use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::InputEvent;
use yew::{html, AttrValue, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub onchange: Callback<AttrValue>,
    pub value: AttrValue,
}

#[styled_component]
pub fn BBTextArea(props: &Props) -> Html {
    let oninput = {
        let props_onchange = props.onchange.clone();

        Callback::from(move |event: InputEvent| {
            let target = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>();
            let value = target.value();

            props_onchange.emit(value.into());
        })
    };

    html! {
        <textarea rows="5" cols="55" {oninput} value={props.value.clone()}>
        </textarea>
    }
}
