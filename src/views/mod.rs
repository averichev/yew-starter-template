use crate::views::another_view::AnotherView;
use crate::views::main_view::MainView;
use yew::{html, Component, Context, Html};
use yew_router::*;

pub mod another_view;
pub mod main_view;

pub struct RootComponent {}

impl Component for RootComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RootComponent {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <BrowserRouter>
                    <Switch<Route> render={RootComponent::switch} />
                </BrowserRouter>
                <footer style="margin-top: 100px;"><a href="https://github.com/averichev/yew-starter-template">{"Source of this starter template"}</a></footer>
            </>
        }
    }
}

impl RootComponent {
    fn switch(routes: Route) -> Html {
        match routes {
            Route::Main => {
                html! {
                    <MainView />
                }
            }
            Route::Another => {
                html! {
                    <AnotherView />
                }
            }
        }
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Main,
    #[at("/another")]
    Another,
}
