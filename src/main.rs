use components::app_module::application::App;

mod components;
mod pages;
mod roures;

fn main() {
    yew::Renderer::<App>::new().render();
}
