use yew::{Component, Context, Html, html};

pub struct SomeComponent;

impl Component for SomeComponent{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SomeComponent{}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <>
            {"This is some component"}
            </>
        }
    }
}