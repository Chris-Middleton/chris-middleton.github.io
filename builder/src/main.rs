extern crate yew;
use std::{fs::File, future::poll_fn, io::Write, path::Path};

use yew::prelude::*;
#[tokio::main]
async fn main() {
    render_page::<App>("index.html").await;
}
async fn render_page<COMP: BaseComponent>(name: &str)
where
    COMP::Properties: Default,
{
    File::create(format!("..\\web\\{}", name))
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
        html!("Heading 1")
    }
}
