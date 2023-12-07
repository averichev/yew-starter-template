use crate::views::RootComponent;

mod views;

fn main() {
    yew::Renderer::<RootComponent>::new().render();
}
