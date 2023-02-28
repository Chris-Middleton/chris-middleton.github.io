extern crate yew;
use std::{fs::File, io::Write};

use yew::prelude::*;
#[tokio::main]
async fn main() {
    render_page::<App>("index").await;
}
async fn render_page<COMP: BaseComponent>(name: &str)
where
    COMP::Properties: Default,
{
    File::create(format!("..\\web\\{}.html", name))
        .unwrap()
        .write_all(yew::ServerRenderer::<COMP>::new().render().await.as_bytes())
        .unwrap()
}

struct App;
impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <html>
            <head>
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/base-min.css"/>
            </head>
            <body>
            <h1>{"I love my girlfriend Trisha!"}</h1>
            </body>
            </html>
        }
    }
}
