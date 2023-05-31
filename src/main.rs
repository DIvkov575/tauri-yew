use chrono::*;
use yew::prelude::*;
mod MyTime;
// use crate::MyTime::MyTime::MyTime;

fn main() {
    yew::Renderer::<App>::new().render();
}

fn gtype<T>(_: &T) {
    log::info!("{}", std::any::type_name::<T>())
}

#[function_component(App)]
pub fn app() -> Html {
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");

    loop {
        let dt: DateTime<Local> = chrono::prelude::Local::now();
        let a = MyTime::MyTime::naive_conv(dt.time());
    }

    html! {
        <h1>{"a"}</h1>
    }
}

// #[function_component()]
