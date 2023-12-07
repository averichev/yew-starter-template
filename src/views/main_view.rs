use crate::views::prelude::*;
use crate::views::Route;
use yew::{html, Component, Context, Html};

pub struct MainView {}

impl Component for MainView {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MainView {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <h1>{"Hello world!"}</h1>
            <img src={"img/logo.svg"} />
            <Link<Route> to={Route::Another}>{ "Another view" }</Link<Route>>
            </>
        }
    }
}
