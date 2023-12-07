use crate::views::prelude::Link;
use crate::views::Route;
use yew::{html, Component, Context, Html};

pub struct AnotherView {}

impl Component for AnotherView {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        AnotherView {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <h1>{"Another view"}</h1>
            <Link<Route> to={Route::Main}>{ "Main view" }</Link<Route>>
            </>
        }
    }
}
