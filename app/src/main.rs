#![recursion_limit = "1024"]

extern crate yew;

use yew::{html, Html};
use yew::prelude::*;
use yew_router::prelude::*;
// Our App is the root component in our component hierarchy.


// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    //A logger that sends a message with its Rust source's line and filename to the browser console.
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

pub struct App {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    // Matches message, always return true since all of our messages produce a different
    // dom to render.
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    // The app view is the frame around the page the user is on.
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
                     <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
        }
    }
}

#[derive(Clone, Routable, PartialEq,Debug)]
enum Route{
    #[at("/")]
    Index,
    #[at("/404")]
    #[not_found]
    NotFound
}
fn switch(routes:&Route)->Html{
    gloo_console::log!(format!("{:?}",routes.clone()));
    match routes.clone() {
        Route::Index => html!{"index"},
        Route::NotFound => html!{}
    }
}