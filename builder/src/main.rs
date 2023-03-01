extern crate yew;
use std::{fs::File, io::Write, process};

use yew::prelude::*;
#[tokio::main]
async fn main() {
    render_page::<App>("index").await;
}
async fn render_page<COMP: BaseComponent>(name: &str)
where
    COMP::Properties: Default,
{
    File::create(format!("..\\docs\\{}.html", name))
        .unwrap()
        .write_all(yew::ServerRenderer::<COMP>::new().render().await.as_bytes())
        .unwrap()
}

struct App;
struct Nav;
struct Content;

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
            <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto Slab"/>
            <link rel="stylesheet" href="./styles.css"/>
            </head>
            <body>
            <Nav/>
            <Content/>
            </body>
            </html>
        }
    }
}

impl Component for Nav {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <div class="header-hides"> // shows/hides on scroll
            <text>{"hides"}</text>
            </div>
            <div class="header-perm"> // always shown
            <text>{"doesnt"}</text>
            </div>
            </div>
        }
    }
}

impl Component for Content {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <h1>{"Heading 1"}</h1>
            <h2>{"Heading 2"}</h2>
            <h3>{"Heading 3"}</h3>
            <h4>{"Heading 4"}</h4>
            <h5>{"Heading 5"}</h5>
            <h6>{"Heading 6"}</h6>
            <LoremIpsum sentences=10 paragraphs=60/>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
struct LoremProps {
    paragraphs: Option<usize>,
    sentences: Option<usize>,
}
#[function_component]
fn LoremIpsum(props: &LoremProps) -> Html {
    static TEXT: [&'static str; 74] = ["Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.","Parturient montes nascetur ridiculus mus mauris vitae ultricies leo integer.","Eget nunc lobortis mattis aliquam faucibus purus in.","Diam vulputate ut pharetra sit.","Pharetra vel turpis nunc eget lorem dolor sed.","Bibendum at varius vel pharetra vel turpis nunc eget.","Dolor sit amet consectetur adipiscing elit ut aliquam purus sit.","Vulputate enim nulla aliquet porttitor.","Quam adipiscing vitae proin sagittis.","Duis convallis convallis tellus id interdum velit laoreet id donec.","Aliquam ultrices sagittis orci a scelerisque purus semper eget.","Consequat nisl vel pretium lectus quam.","Senectus et netus et malesuada fames ac turpis.","Diam quis enim lobortis scelerisque fermentum dui faucibus in ornare.","Leo a diam sollicitudin tempor id.","Sed velit dignissim sodales ut eu sem integer vitae justo.","Orci phasellus egestas tellus rutrum tellus pellentesque.",
        "Ac auctor augue mauris augue neque.","Sollicitudin tempor id eu nisl.","Mauris a diam maecenas sed enim ut.","Netus et malesuada fames ac turpis egestas sed tempus urna.","Egestas pretium aenean pharetra magna ac placerat.","Sit amet facilisis magna etiam tempor orci.","Ipsum suspendisse ultrices gravida dictum fusce.","Aliquam malesuada bibendum arcu vitae.","Sit amet porttitor eget dolor.","Ultricies tristique nulla aliquet enim tortor at auctor urna nunc.","Blandit volutpat maecenas volutpat blandit aliquam etiam erat velit.",
        "Urna neque viverra justo nec ultrices dui.","Sed ullamcorper morbi tincidunt ornare massa eget egestas purus.","Elit eget gravida cum sociis natoque penatibus.","Ut sem nulla pharetra diam sit amet.","Eget mi proin sed libero enim sed faucibus turpis.","Cursus turpis massa tincidunt dui ut ornare lectus sit.","Sodales ut eu sem integer vitae justo.","Purus semper eget duis at tellus at.","Ultrices tincidunt arcu non sodales.","Facilisis mauris sit amet massa.","Gravida dictum fusce ut placerat orci nulla.","Nunc non blandit massa enim nec.","Enim sed faucibus turpis in eu mi.","Ac placerat vestibulum lectus mauris ultrices.","Mauris commodo quis imperdiet massa.","Vitae tempus quam pellentesque nec nam.",
        "Rhoncus mattis rhoncus urna neque viverra justo nec ultrices.","Ultricies leo integer malesuada nunc.","Gravida cum sociis natoque penatibus et magnis.","Turpis nunc eget lorem dolor sed viverra ipsum nunc aliquet.","Bibendum enim facilisis gravida neque.","Arcu non odio euismod lacinia.","Mollis nunc sed id semper risus in hendrerit.","Integer malesuada nunc vel risus commodo viverra maecenas accumsan lacus.","Tincidunt nunc pulvinar sapien et ligula ullamcorper malesuada proin.","Sed libero enim sed faucibus turpis in.","Laoreet suspendisse interdum consectetur libero.","Aliquet bibendum enim facilisis gravida neque convallis a cras.","Tincidunt arcu non sodales neque sodales.","Molestie ac feugiat sed lectus vestibulum mattis.","Sed ullamcorper morbi tincidunt ornare massa eget.","Egestas integer eget aliquet nibh.","Magna ac placerat vestibulum lectus mauris ultrices eros in cursus.","Sagittis nisl rhoncus mattis rhoncus urna neque viverra.","Quam elementum pulvinar etiam non.",
        "Netus et malesuada fames ac turpis egestas sed tempus.","Urna et pharetra pharetra massa massa ultricies mi quis hendrerit.","Erat velit scelerisque in dictum.","Ut sem nulla pharetra diam sit amet nisl suscipit.","Arcu odio ut sem nulla pharetra diam sit.","Leo a diam sollicitudin tempor id eu nisl nunc mi.","Gravida arcu ac tortor dignissim.","Viverra tellus in hac habitasse platea.","Nibh ipsum consequat nisl vel pretium lectus quam id leo.","Consectetur a erat nam at lectus.","Donec adipiscing tristique risus nec feugiat in fermentum posuere urna."];

    let len = props.sentences.unwrap_or(1);

    (0..props.paragraphs.unwrap_or(1))
        .into_iter()
        .map(|p| {
            let start = p * len % TEXT.len();
            let end = start + len;
            if end > TEXT.len() {
                let s1 = TEXT[start..].join(" ");
                let s2 = TEXT[..end % TEXT.len()].join(" ");
                html! {<p>{[s1, s2].join(" ")}</p>}
            } else {
                html! {<p>{TEXT[start..end].join(" ")}</p>}
            }
        })
        .collect::<Html>()
}
