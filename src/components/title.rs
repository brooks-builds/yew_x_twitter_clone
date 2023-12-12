use stylist::{style, yew::styled_component, Style};
use yew::{classes, html, virtual_dom::VNode, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default()]
    pub center: bool,
    pub level: BBTitleLevel,
}

#[styled_component]
pub fn BBTitle(props: &Props) -> Html {
    let mut styles = vec![];

    if props.center {
        styles.push(
            style! {
                text-align: center;
            }
            .expect("style failed to create"),
        );
    }

    props.level.render(styles, props.children.clone())
}

#[derive(PartialEq)]
pub enum BBTitleLevel {
    One,
    Two,
}

impl BBTitleLevel {
    pub fn render(&self, styles: Vec<Style>, children: VNode) -> Html {
        match self {
            Self::One => html! { <h1 class={classes!(styles)}>{children}</h1> },
            Self::Two => html! { <h2 class={classes!(styles)}>{children}</h2> },
        }
    }
}
