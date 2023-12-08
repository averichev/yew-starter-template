use crate::views::another_view::AnotherView;
use crate::views::main_view::MainView;
use yew::{html, Html};
use yew_router::Routable;

pub fn switch(routes: MainRouter) -> Html {
    match routes {
        MainRouter::Main => {
            html! {
                <MainView />
            }
        }
        MainRouter::Another => {
            html! {
                <AnotherView />
            }
        }
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum MainRouter {
    #[at("/")]
    Main,
    #[at("/another")]
    Another,
}
